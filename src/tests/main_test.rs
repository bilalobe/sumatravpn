use actix_web::{test, App};
use crate::config::VPNConfig;
use crate::wg_controller::{configure_wg, connect_wg, disconnect_wg, check_wg_status};
use crate::main::{configure, connect, disconnect, status};

#[actix_rt::test]
async fn test_configure_endpoint() {
    let config = VPNConfig {
        interface_name: Some("wg0".to_string()),
        private_key: "test_private_key".to_string(),
        endpoint: "test_endpoint".to_string(),
    };

    let app = test::init_service(App::new().service(configure)).await;
    let req = test::TestRequest::post()
        .uri("/configure")
        .set_json(&config)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_connect_endpoint() {
    let config = VPNConfig {
        interface_name: Some("wg0".to_string()),
        private_key: "test_private_key".to_string(),
        endpoint: "test_endpoint".to_string(),
    };

    let app = test::init_service(App::new().service(connect)).await;
    let req = test::TestRequest::post()
        .uri("/connect")
        .set_json(&config)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_disconnect_endpoint() {
    let config = VPNConfig {
        interface_name: Some("wg0".to_string()),
        private_key: "test_private_key".to_string(),
        endpoint: "test_endpoint".to_string(),
    };

    let app = test::init_service(App::new().service(disconnect)).await;
    let req = test::TestRequest::post()
        .uri("/disconnect")
        .set_json(&config)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_status_endpoint() {
    let app = test::init_service(App::new().service(status)).await;
    let req = test::TestRequest::get().uri("/status").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[tokio::test]
async fn test_configure_wg() {
    let config = VPNConfig {
        interface_name: Some("wg0".to_string()),
        private_key: "test_private_key".to_string(),
        endpoint: "test_endpoint".to_string(),
    };

    let result = configure_wg(&config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_connect_wg() {
    let config = VPNConfig {
        interface_name: Some("wg0".to_string()),
        private_key: "test_private_key".to_string(),
        endpoint: "test_endpoint".to_string(),
    };

    let result = connect_wg(&config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_disconnect_wg() {
    let config = VPNConfig {
        interface_name: Some("wg0".to_string()),
        private_key: "test_private_key".to_string(),
        endpoint: "test_endpoint".to_string(),
    };

    let result = disconnect_wg(&config).await;
    assert!(result.is_ok());
}

#[test]
fn test_check_wg_status() {
    let result = check_wg_status("wg0");
    assert!(result.is_ok());
}