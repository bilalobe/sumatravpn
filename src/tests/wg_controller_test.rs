#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::VPNConfig;

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
}