use crate::service::WireguardManagerServiceImpl;
use crate::WireguardManagerConfig;
use std::fs;
use std::net::SocketAddr;
use tonic::transport::Server;
use wg_manager_service_common::wg_manager_service::wireguard_manager_service_server::WireguardManagerServiceServer;

pub struct WireguardManagerServer {}

impl WireguardManagerServer {
    pub async fn start(config: &WireguardManagerConfig) -> Result<(), Box<dyn std::error::Error>> {
        let socket_addr = SocketAddr::new(config.host, config.port);

        let wireguard_manager_service =
            WireguardManagerServiceServer::new(WireguardManagerServiceImpl::default());

        let mut builder = Server::builder();

        let mut tls_config = None;

        // TLS setup
        if config.tls_cert.is_some() {
            info!("Loading tls certificate...");
            let cert = fs::read(config.tls_cert.as_ref().unwrap()).unwrap();
            let key = fs::read(config.tls_key.as_ref().unwrap()).unwrap();

            let id = tonic::transport::Identity::from_pem(cert, key);

            let tls = tonic::transport::ServerTlsConfig::new().identity(id);

            tls_config = Some(tls);
        }

        // Client CA for authentication
        if let Some(client_ca) = &config.client_ca {
            let ca = fs::read(client_ca).unwrap();
            let ca = tonic::transport::Certificate::from_pem(ca);

            if let Some(tls) = tls_config {
                tls_config = Some(tls.client_ca_root(ca));
            } else {
                let tls = tonic::transport::ServerTlsConfig::new().client_ca_root(ca);
                tls_config = Some(tls);
            }
        }

        if let Some(tls) = tls_config {
            builder = builder.tls_config(tls)?;
        }

        builder
            .add_service(wireguard_manager_service)
            .serve(socket_addr)
            .await?;

        Ok(())
    }
}
