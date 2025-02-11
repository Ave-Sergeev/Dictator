use crate::error::error::TranscribeServiceError;
use crate::pb::inference_pb::transcribe_service_server::TranscribeService;
use crate::pb::inference_pb::{AudioConfig, AudioType, RecognizeRequest, RecognizeResponse};
use crate::service::local_recognizer::LocalRecogniser;
use crate::utils::transcode::pcm_s16be_to_pcm_s16le;
use crate::utils::wav::{bytes_to_i16, get_samples_from_wav};
use tonic::{Request, Response, Status};
use vosk::Model;

pub struct ServiceImpl {
    pub model: Model,
}

impl ServiceImpl {
    pub fn new(model: Model) -> ServiceImpl {
        Self { model }
    }

    fn get_audio_and_config_from_request(request: &RecognizeRequest) -> Result<(Vec<i16>, AudioConfig), Status> {
        let config = request
            .config
            .ok_or_else(|| Status::invalid_argument("No config provided"))?;

        let audio = Self::transform_audio_to_i16(&request.content, config)?;

        Ok((audio, config))
    }

    fn transform_audio_to_i16(audio: &[u8], config: AudioConfig) -> Result<Vec<i16>, Status> {
        if audio.is_empty() {
            return Err(Status::invalid_argument("Audio input is empty"));
        }

        match config.audio_type() {
            AudioType::WavPcmS16le => get_samples_from_wav(audio),
            AudioType::RawPcmS16le => Ok(bytes_to_i16(audio)),
            AudioType::RawPcmS16be => {
                let bytes = pcm_s16be_to_pcm_s16le(audio);
                Ok(bytes_to_i16(&bytes))
            }
            AudioType::Unspecified => Err(TranscribeServiceError::InvalidAudio(
                "Only pcm_s16le and pcm_s16be are supported".to_string(),
            )),
        }
        .map_err(|err| Status::invalid_argument(format!("{err}")))
    }
}

#[tonic::async_trait]
impl TranscribeService for ServiceImpl {
    async fn transcribe(&self, request: Request<RecognizeRequest>) -> Result<Response<RecognizeResponse>, Status> {
        let recognize_request = request.into_inner();
        let (audio_data, config) = Self::get_audio_and_config_from_request(&recognize_request)?;
        let sample_rate = config.sample_rate as f32;
        let model = &self.model;

        let mut local_recognizer = LocalRecogniser::new(&model, sample_rate);
        let response = local_recognizer.transcribe(audio_data)?;

        Ok(Response::new(response))
    }
}
