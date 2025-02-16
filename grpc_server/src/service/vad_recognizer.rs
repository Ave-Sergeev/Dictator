use crate::error::error;
use crate::settings::VadSettings;
use silero_vad::service::recognizer::Recognizer;
use silero_vad::utils::utils::{TimeStamp, VadParams};

pub struct VadService {
    recognizer: Recognizer,
}

impl VadService {
    pub fn new(settings: &VadSettings) -> error::Result<Self> {
        fn vad_params(settings: &VadSettings, sample_rate: usize) -> VadParams {
            VadParams {
                sample_rate,
                frame_size: settings.frame_size,
                threshold: settings.threshold,
                min_silence_duration_ms: settings.min_silence_duration_ms,
                speech_pad_ms: settings.speech_pad_ms,
                min_speech_duration_ms: settings.min_speech_duration_ms,
                max_speech_duration_s: settings.max_speech_duration_s,
                ..Default::default()
            }
        }

        let vad_params = vad_params(settings, 16000);

        let recognizer = Recognizer::new(settings.model_path.as_str(), vad_params, settings.sessions_num)?;
        Ok(Self { recognizer })
    }

    pub fn recognize(&self, audio: Vec<i16>, sample_rate: u32) -> error::Result<Vec<TimeStamp>> {
        match sample_rate {
            16000 => Ok(self.recognizer.process(&*audio)?),
            _ => Err(error::ServiceError::InvalidAudio("Unsupported sample rate".to_string())),
        }
    }
}
