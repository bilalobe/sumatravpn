use std::process::Command;
use std::io::{self, ErrorKind};

use crate::config::{VPNConfig, ConfigurationError};

pub struct Interface {
    name: String,
}

impl Interface {
    pub fn new(name: &str) -> Self {
        Interface {
            name: name.to_string(),
        }
    }

    pub fn up(&self) -> io::Result<()> {
        let output = Command::new("ip")
            .arg("link")
            .arg("set")
            .arg(&self.name)
            .arg("up")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(ErrorKind::Other, "Failed to bring interface up"));
        }

        Ok(())
    }

    pub fn down(&self) -> io::Result<()> {
        let output = Command::new("ip")
            .arg("link")
            .arg("set")
            .arg(&self.name)
            .arg("down")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(ErrorKind::Other, "Failed to bring interface down"));
        }

        Ok(())
    }

    pub fn is_up(&self) -> io::Result<bool> {
        let output = Command::new("ip")
            .arg("link")
            .arg("show")
            .arg(&self.name)
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(ErrorKind::Other, "Failed to check interface status"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.contains("state UP"))
    }
}

pub async fn configure_wg(config: &VPNConfig) -> Result<(), ConfigurationError> {
    let output = Command::new("wg")
        .arg("set")
        .arg(config.interface_name.as_deref().unwrap_or("wg0"))
        .arg("private-key")
        .arg(&config.private_key)
        .arg("endpoint")
        .arg(&config.endpoint)
        .output()
        .map_err(|e| ConfigurationError::CommandError(e.to_string()))?;

    if !output.status.success() {
        return Err(ConfigurationError::CommandFailed);
    }

    Ok(())
}

pub async fn connect_wg(config: &VPNConfig) -> Result<(), ConfigurationError> {
    configure_wg(config).await?;
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