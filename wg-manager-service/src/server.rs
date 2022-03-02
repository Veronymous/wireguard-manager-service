use std::net::SocketAddr;
use tonic::transport::Server;
use wg_manager_service_common::wg_manager_service::wireguard_manager_service_server::WireguardManagerServiceServer;
use std::fs;
use crate::service::WireguardManagerServiceImpl;
use crate::WireguardManagerConfig;

pub struct WireguardManagerServer {}

impl WireguardManagerServer {
    pub async fn start(config: &WireguardManagerConfig) -> Result<(), Box<dyn std::error::Error>> {
        let socket_addr = SocketAddr::new(config.host, config.port);

        let wireguard_manager_service =
            WireguardManagerServiceServer::new(WireguardManagerServiceImpl::default());

        let mut builder = Server::builder();

        // Tls encryption config
        if config.tls_cert.is_some() {
            info!("Loading tls certificate...");
            let cert = fs::read(config.tls_cert.as_ref().unwrap()).unwrap();
            let key = fs::read(config.tls_key.as_ref().unwrap()).unwrap();

            let id = tonic::transport::Identity::from_pem(cert, key);

            let tls = tonic::transport::ServerTlsConfig::new()
                .identity(id);
            //.client_ca_root();

            builder = builder.tls_config(tls)?;
        }

        builder
            .add_service(wireguard_manager_service)
            .serve(socket_addr)
            .await?;

        Ok(())
    }
}
