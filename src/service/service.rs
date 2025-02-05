use crate::pb::simple_pb::simple_server::Simple;
use crate::pb::simple_pb::SimpleResponse;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct SimpleService {}

#[tonic::async_trait]
impl Simple for SimpleService {
    async fn test(&self, _: Request<()>) -> Result<Response<SimpleResponse>, Status> {
        let reply = SimpleResponse {
            text: "Тестовый ответ".to_string(),
        };

        Ok(Response::new(reply))
    }
}
