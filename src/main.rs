use crate::service::service::SimpleService;
use crate::settings::Settings;
use pb::simple_pb::simple_server::SimpleServer;
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

    let simple_service = SimpleService::default();

    Server::builder()
        .add_service(SimpleServer::new(simple_service))
        .serve(address)
        .await?;

    Ok(())
}
