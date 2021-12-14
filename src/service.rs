use tonic::{Request, Response, Status};
use wg_manager_service::wireguard_manager_service_server::WireguardManagerService;
use wg_manager_service::{AddPeerRequest, AddPeerResponse, RemovePeerRequest, RemovePeerResponse};
use wireguard_manager::WireguardManager;

pub mod wg_manager_service {
    tonic::include_proto!("wg_manager_service");
}

// Wireguard manager service implementation
#[derive(Default)]
pub struct WireguardManagerServiceImpl {
    wireguard_manager: WireguardManager,
}

// Implement the service
#[tonic::async_trait]
impl WireguardManagerService for WireguardManagerServiceImpl {
    async fn add_peer(
        &self,
        request: Request<AddPeerRequest>,
    ) -> Result<Response<AddPeerResponse>, tonic::Status> {
        let request = request.into_inner();

        // TODO: Switch do debug
        info!(
            "Adding peer: {}, with addresses: {:?}",
            request.public_key, request.addresses
        );

        try_wg_function!(
            self.wireguard_manager.add_peer(&request.public_key, &request.addresses)
        );

        Ok(Response::new(AddPeerResponse {}))
    }

    async fn remove_peer(
        &self,
        request: Request<RemovePeerRequest>,
    ) -> Result<Response<RemovePeerResponse>, Status> {
        let request = request.into_inner();

        // TODO: Switch to debug
        info!(
            "Removing peer: {}", request.public_key
        );

        try_wg_function!(
          self.wireguard_manager.remove_peer(&request.public_key)
        );

        Ok(Response::new(RemovePeerResponse {}))
    }
}
