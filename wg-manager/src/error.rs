use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum WireguardManagerError {
    #[error("Initialization error. {0}")]
    InitializationError(String),

    #[error("Wireguard error. {0}")]
    WireguardError(String),
}
