use crate::pb::inference_pb::transcribe_service_server::TranscribeService;
use crate::pb::inference_pb::*;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl TranscribeService for Service {
    async fn transcribe(&self, request: Request<Audio>) -> Result<Response<Phrases>, Status> {
        // TODO: Временно реализована заглушка
        Ok(Response::new(Phrases {
            phrases: vec![Phrase {
                text: "Hello World".to_string(),
                words: vec![
                    Word {
                        word: "Hello".to_string(),
                        start_ms: 0,
                        end_ms: 540,
                    },
                    Word {
                        word: "World".to_string(),
                        start_ms: 890,
                        end_ms: 1400,
                    },
                ],
            }],
        }))
    }
}
