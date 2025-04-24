use crate::pb::inference_pb::transcribe_service_server::TranscribeServiceServer;
use crate::service::transcribe_service::ServiceImpl;
use crate::settings::Settings;
use env_logger::Builder;
use log::LevelFilter;
use std::error::Error;
use std::str::FromStr;
use tonic::transport::Server;
use vosk::Model;

mod error;
mod pb;
mod service;
mod settings;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new("config.yaml").map_err(|err| format!("Failed to load settings: {err}"))?;

    Builder::new()
        .filter_level(LevelFilter::from_str(settings.logging.log_level.as_str()).unwrap_or(LevelFilter::Info))
        .init();

    log::info!("Settings:\n{}\n", settings.json_pretty());

    let address = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .map_err(|err| format!("Invalid server address: {err}"))?;

    log::info!("Server listening on: {address}");

    let model = Model::new(&settings.vosk.model_path)
        .ok_or_else(|| format!("Failed to load model from: {}", &settings.vosk.model_path))?;

    let transcribe_service =
        ServiceImpl::new(model, &settings).map_err(|err| format!("Failed to initialize service: {err}"))?;

    Server::builder()
        .add_service(TranscribeServiceServer::new(transcribe_service))
        .serve(address)
        .await
        .map_err(|err| format!("GRPC server returned error:{err}"))?;

    Ok(())
}
