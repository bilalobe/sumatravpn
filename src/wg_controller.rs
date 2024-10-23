use wgctrl::Interface;
use crate::config::{VPNConfig, ConfigurationError};

pub async fn configure_wg(config: &VPNConfig) -> Result<(), ConfigurationError> {
    // Logic to configure WireGuard interface using sensitive data
    let output = Command::new("wg")
        .arg("set")
        .arg(&config.interface_name)
        .arg("private-key")
        .arg(&config.private_key) // Sensitive data
        .arg("endpoint")
        .arg(&config.endpoint)
        .output()
        .await?;

    if !output.status.success() {
        return Err(ConfigurationError::CommandFailed);
    }

    Ok(())
}

pub async fn connect_wg(config: &VPNConfig) -> Result<(), ConfigurationError> {
    let interface_name = config.interface_name.clone().unwrap_or_else(|| "wg0".to_string());
    let interface = Interface::new(&interface_name);
    interface.up().map_err(|e| ConfigurationError::CommandError(e.to_string()))?;
    Ok(())
}

pub async fn disconnect_wg(config: &VPNConfig) -> Result<(), ConfigurationError> {
    let interface_name = config.interface_name.clone().unwrap_or_else(|| "wg0".to_string());
    let interface = Interface::new(&interface_name);
    interface.down().map_err(|e| ConfigurationError::CommandError(e.to_string()))?;
    Ok(())
}

pub fn check_wg_status(interface_name: &str) -> Result<bool, ConfigurationError> {
    let interface = Interface::new(interface_name);
    match interface.is_up() {
        Ok(status) => Ok(status),
        Err(e) => Err(ConfigurationError::CommandError(e.to_string())),
    }
}