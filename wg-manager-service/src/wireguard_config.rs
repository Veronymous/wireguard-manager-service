use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::net::IpAddr;

const WG_MANAGER_ENV_VAR: &str = "WG_MANAGER_CONFIG";
const WG_MANAGER_CONFIG_LOCATION: &str = "wireguard_manager.yml";

#[derive(Clone, Debug, Deserialize)]
pub struct WireguardManagerConfig {
    pub host: IpAddr,

    pub port: u16,

    pub tls_cert: Option<String>,

    pub tls_key: Option<String>,

    pub client_ca: Option<String>,
}

impl WireguardManagerConfig {
    pub fn load() -> Result<Self, ConfigError> {
        // Get the config location
        let config_location =
            std::env::var(WG_MANAGER_ENV_VAR).unwrap_or_else(|_| WG_MANAGER_CONFIG_LOCATION.into());

        // Load the config
        let mut config = Config::new();
        config.merge(File::with_name(&config_location))?;

        let wg_manager_config = config.try_into()?;

        Ok(wg_manager_config)
    }
}
