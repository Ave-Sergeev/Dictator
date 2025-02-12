use crate::pb::inference_pb::transcribe_service_server::TranscribeServiceServer;
use crate::service::transcribe_service::ServiceImpl;
use crate::settings::Settings;
use tonic::transport::Server;
use vosk::Model;

mod error;
mod pb;
mod service;
mod settings;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new("config.yaml")?;
    println!("Settings:\n{}", settings.json_pretty());

    let address = format!("{}:{}", settings.server.host, settings.server.port).parse()?;
    println!("Server listening on {}", address);

    let model = Model::new(settings.vosk.model_path).expect("Could not initialize Vosk model!");
    let transcribe_service = ServiceImpl::new(model, settings.vosk.pause_threshold);

    Server::builder()
        .add_service(TranscribeServiceServer::new(transcribe_service))
        .serve(address)
        .await?;

    Ok(())
}
