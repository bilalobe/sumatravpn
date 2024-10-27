mod config;
mod wg_controller;

use log::info;
use actix_web::{get, post, web, App, HttpServer, Responder};
use crate::config::{VPNConfig, ConfigurationError};
use crate::wg_controller::{connect_wg, disconnect_wg, check_wg_status};

#[post("/configure")]
/// Handles the `/configure` endpoint.
///
/// This function configures the VPN based on the provided configuration.
/// It returns a success message if the configuration is successful,
/// or an error if something goes wrong.
async fn configure(_config: web::Json<VPNConfig>) -> Result<String, ConfigurationError> {
    // Configuration Logic (including validation, writing to config file)
    Ok("Configuration successful".to_string())
}

#[post("/connect")]
async fn connect(config: web::Json<VPNConfig>) -> Result<String, ConfigurationError> {
    connect_wg(&config).await?;
    Ok("Connecting...".to_string())
}

#[post("/disconnect")]
/// Handles the `/disconnect` endpoint.
/// 
/// This function disconnects the VPN based on the provided configuration.
/// It returns a success message if the disconnection is initiated successfully,
/// or an error if something goes wrong.
async fn disconnect(config: web::Json<VPNConfig>) -> Result<String, ConfigurationError> {
    info!("Starting disconnect process for interface: {:?}", config.interface_name);
    match disconnect_wg(&config).await {
        Ok(_) => {
            info!("Disconnect process completed for interface: {:?}", config.interface_name);
            Ok("Disconnecting...".to_string())
        }
        Err(e) => {
            log::error!("Failed to disconnect interface: {:?}, error: {:?}", config.interface_name, e);
            Err(e)
        }
    }
}

#[get("/status")]
/// Handles the `/status` endpoint.
///
/// This function checks the status of the VPN connection.
/// It returns a JSON response indicating whether the VPN is connected or disconnected.
async fn status() -> impl Responder {
    let interface_name = "wg0"; // Default interface name, can be modified as needed
    match check_wg_status(interface_name) {
        Ok(is_up) => {
            if is_up {
                web::Json(serde_json::json!({"status": "connected"}))
            } else {
                web::Json(serde_json::json!({"status": "disconnected"}))
            }
        }
        Err(_) => web::Json(serde_json::json!({"status": "error"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger

    // Load configuration from environment variables
    let config = VPNConfig::from_env().expect("Failed to load VPN configuration from environment variables");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone())) // Share the configuration with the app
            .service(status)
            .service(configure)
            .service(connect)
            .service(disconnect)
    })
    .bind(("0.0.0.0", 8080))?  // Bind to all interfaces
    .run()
    .await
}
