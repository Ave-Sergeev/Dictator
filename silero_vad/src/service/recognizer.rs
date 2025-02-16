use crate::service::silero::SileroSession;
use crate::service::vad_iter::VadIter;
use crate::utils::utils::{SampleRate, TimeStamp, VadParams};
use crate::{error, OnnxSession};
use error::error::ServiceError;
use lockfree_object_pool::MutexObjectPool;
use ort::session::builder::GraphOptimizationLevel;
use parking_lot::Mutex;
use std::iter::Cycle;
use std::sync::Arc;
use std::vec::IntoIter;

pub struct Recognizer {
    vad_iter_pool: Arc<MutexObjectPool<VadIter>>,
    sessions: Arc<Mutex<Cycle<IntoIter<Arc<OnnxSession>>>>>,
}

impl Recognizer {
    pub fn new(model_path: &str, vad_params: VadParams, sessions_num: u8) -> Result<Self, ServiceError> {
        if sessions_num == 0 {
            return Err(ServiceError::InvalidConfiguration("Sessions number must be greater than 0".to_string()));
        }

        let onnx_sessions = Self::create_onnx_sessions(model_path, sessions_num)?;
        let sessions_iter = Arc::new(Mutex::new(onnx_sessions.into_iter().cycle()));

        let sample_rate = vad_params.sample_rate.into();

        let vad_iter_pool = Self::create_vad_iter_pool(sessions_iter.clone(), sample_rate, vad_params);

        Ok(Self {
            vad_iter_pool: Arc::new(vad_iter_pool),
            sessions: sessions_iter,
        })
    }

    fn create_vad_iter_pool(
        sessions_iter: Arc<Mutex<Cycle<IntoIter<Arc<OnnxSession>>>>>,
        sample_rate: SampleRate,
        vad_params: VadParams,
    ) -> MutexObjectPool<VadIter> {
        MutexObjectPool::new(
            move || {
                let session = sessions_iter.lock().next().expect("no onnx sessions to cycle");
                let silero = SileroSession::new(session, sample_rate).expect("error creating Silero session");
                VadIter::new(silero, vad_params.clone())
            },
            |_| {},
        )
    }

    fn create_onnx_sessions(model_path: &str, sessions_num: u8) -> Result<Vec<Arc<OnnxSession>>, ServiceError> {
        (0..sessions_num)
            .map(|_| Self::make_onnx_session(model_path).map(Arc::new))
            .collect()
    }

    fn make_onnx_session(model_path: &str) -> crate::Result<OnnxSession> {
        let session = OnnxSession::builder()?
            .with_inter_threads(1)?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .commit_from_file(model_path)?;
        Ok(session)
    }

    pub fn process(&self, samples: &[i16]) -> Result<Vec<TimeStamp>, ServiceError> {
        let mut vad = self.vad_iter_pool.pull();
        vad.process(samples)?;
        Ok(vad.speeches())
    }
}
