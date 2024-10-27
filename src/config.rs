use serde::{Deserialize, Serialize};
use thiserror::Error;
use actix_web::ResponseError;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VPNConfig {
    pub interface_name: Option<String>,
    pub private_key: String,
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: Vec<String>,
    pub dns: Option<Vec<String>>,
}

impl VPNConfig {
    pub fn from_env() -> Result<Self, ConfigurationError> {
        dotenv().ok(); // Load environment variables from a .env file if present

        Ok(Self {
            interface_name: env::var("INTERFACE_NAME").ok(),
            private_key: env::var("PRIVATE_KEY").map_err(|e| ConfigurationError::EnvVarError(e.to_string()))?,
            public_key: env::var("PUBLIC_KEY").map_err(|e| ConfigurationError::EnvVarError(e.to_string()))?,
            endpoint: env::var("ENDPOINT").map_err(|e| ConfigurationError::EnvVarError(e.to_string()))?,
            allowed_ips: env::var("ALLOWED_IPS")
                .map_err(|e| ConfigurationError::EnvVarError(e.to_string()))?
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            dns: env::var("DNS").ok().map(|s| s.split(',').map(|s| s.to_string()).collect()),
        })
    }
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Command error: {0}")]
    CommandError(String),
    #[error("Command failed")]
    CommandFailed,
    #[error("Environment variable error: {0}")]
    EnvVarError(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl ResponseError for ConfigurationError {}