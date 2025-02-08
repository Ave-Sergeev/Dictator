use crate::pb::inference_pb::transcribe_service_server::TranscribeService;
use crate::pb::inference_pb::*;
use crate::service::local_recognizer::LocalRecogniser;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl TranscribeService for Service {
    async fn transcribe(&self, request: Request<Audio>) -> Result<Response<Phrases>, Status> {
        let audio_data = request.into_inner().audio_data;
        let samples: Vec<i16> = audio_data
            .chunks_exact(2)
            .map(|chunk| {
                let buf = &chunk[..];
                i16::from_le_bytes([buf[0], buf[1]])
            })
            .collect();

        let model_path = String::from("./resources/vosk-model-small-ru-0.22");
        let sample_rate = 16000.0;

        let mut recognizer = LocalRecogniser::new(&*model_path, sample_rate);
        let result = recognizer.transcribe(samples);

        Ok(Response::new(result))
    }
}
