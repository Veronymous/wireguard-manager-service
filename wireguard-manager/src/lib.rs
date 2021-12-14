use crate::error::WireguardManagerError;

use std::process::Command;

pub mod error;

#[derive(Default)]
pub struct WireguardManager {}

impl WireguardManager {
    pub fn add_peer(
        &self,
        public_key: &String,
        addresses: &[String],
    ) -> Result<(), WireguardManagerError> {
        let addresses_argument = Self::construct_addresses_argument(addresses);

        let command = format!(
            "wg set wg0 peer {} allowed-ips {}",
            public_key, addresses_argument
        );

        Self::execute_command(&command)?;

        Ok(())
    }

    pub fn remove_peer(&self, public_key: &String) -> Result<(), WireguardManagerError> {
        let command = format!(
            "wg set wg0 peer {} remove",
            public_key
        );

        Self::execute_command(&command)?;

        Ok(())
    }

    fn execute_command(command: &String) -> Result<(), WireguardManagerError> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| {
                WireguardManagerError::WireguardError(format!(
                    "Could not remove peer. {}",
                    e.to_string()
                ))
            })?;

        if !out.status.success() {
            return Err(WireguardManagerError::WireguardError(format!(
                "Could not add peer. Got an invalid status code: {}",
                out.status
            )));
        }

        Ok(())
    }

    fn construct_addresses_argument(addresses: &[String]) -> String {
        let mut addresses_argument = String::new();

        for i in 0..addresses.len() {
            let address = &addresses[i];

            addresses_argument += &address;

            if i < addresses.len() - 1 {
                addresses_argument += ",";
            }
        }

        addresses_argument
    }
}
