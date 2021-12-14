use std::net::{IpAddr, SocketAddr};
use tonic::transport::Server;

use crate::service::wg_manager_service::wireguard_manager_service_server::WireguardManagerServiceServer;
use crate::service::WireguardManagerServiceImpl;

pub struct WireguardManagerServer {}

impl WireguardManagerServer {
    pub async fn start(ip: IpAddr, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let socket_addr = SocketAddr::new(ip, port);

        let wireguard_manager_service =
            WireguardManagerServiceServer::new(WireguardManagerServiceImpl::default());

        Server::builder()
            .add_service(wireguard_manager_service)
            .serve(socket_addr)
            .await?;

        Ok(())
    }
}
