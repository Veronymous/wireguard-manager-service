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

    let config = WireguardManagerConfig::load().unwrap();

    info!("Starting server...");
    info!("Using host: {}", config.host);
    info!("Using port: {}", config.port);

    WireguardManagerServer::start(&config).await?;

    Ok(())
}
