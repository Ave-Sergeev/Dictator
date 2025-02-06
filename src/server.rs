use crate::pb::inference_pb::transcribe_service_server::TranscribeServiceServer;
use crate::service::transcribe_service::Service;
use crate::settings::Settings;
use tonic::transport::Server;

mod pb;
mod service;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new("config.yaml")?;
    println!("Settings:\n{}", settings.json_pretty());

    let address = format!("{}:{}", settings.server.host, settings.server.port).parse()?;
    println!("Server listening on {}", address);

    let transcribe_service = Service::default();

    Server::builder()
        .add_service(TranscribeServiceServer::new(transcribe_service))
        .serve(address)
        .await?;

    Ok(())
}
