#[macro_use]
extern crate log;

#[macro_use]
mod error;

mod server;
mod service;
mod wireguard_config;

use env_logger;

use crate::server::WireguardManagerServer;
use crate::wireguard_config::WireguardManagerConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Loading wireguard manager service...");

    let settings = WireguardManagerConfig::load().unwrap();

    info!("Starting server...");
    info!("Using host: {}", settings.host);
    info!("Using port: {}", settings.port);

    WireguardManagerServer::start(settings.host, settings.port).await?;

    Ok(())
}
