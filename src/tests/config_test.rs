#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn set_env_vars() {
        env::set_var("INTERFACE_NAME", "wg0");
        env::set_var("PRIVATE_KEY", "test_private_key");
        env::set_var("PUBLIC_KEY", "test_public_key");
        env::set_var("ENDPOINT", "test_endpoint");
        env::set_var("ALLOWED_IPS", "10.0.0.1/24,10.0.0.2/24");
        env::set_var("DNS", "8.8.8.8,8.8.4.4");
    }

    fn clear_env_vars() {
        env::remove_var("INTERFACE_NAME");
        env::remove_var("PRIVATE_KEY");
        env::remove_var("PUBLIC_KEY");
        env::remove_var("ENDPOINT");
        env::remove_var("ALLOWED_IPS");
        env::remove_var("DNS");
    }

    #[test]
    fn test_from_env_success() {
        set_env_vars();
        let config = VPNConfig::from_env();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.interface_name, Some("wg0".to_string()));
        assert_eq!(config.private_key, "test_private_key");
        assert_eq!(config.public_key, "test_public_key");
        assert_eq!(config.endpoint, "test_endpoint");
        assert_eq!(config.allowed_ips, vec!["10.0.0.1/24".to_string(), "10.0.0.2/24".to_string()]);
        assert_eq!(config.dns, Some(vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()]));
        clear_env_vars();
    }

    #[test]
    fn test_from_env_missing_required_vars() {
        clear_env_vars();
        let config = VPNConfig::from_env();
        assert!(config.is_err());
        let err = config.unwrap_err();
        match err {
            ConfigurationError::EnvVarError(_) => assert!(true),
            _ => assert!(false, "Expected EnvVarError"),
        }
    }

    #[test]
    fn test_from_env_partial_success() {
        clear_env_vars();
        env::set_var("PRIVATE_KEY", "test_private_key");
        env::set_var("PUBLIC_KEY", "test_public_key");
        env::set_var("ENDPOINT", "test_endpoint");
        env::set_var("ALLOWED_IPS", "10.0.0.1/24,10.0.0.2/24");

        let config = VPNConfig::from_env();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.interface_name, None);
        assert_eq!(config.private_key, "test_private_key");
        assert_eq!(config.public_key, "test_public_key");
        assert_eq!(config.endpoint, "test_endpoint");
        assert_eq!(config.allowed_ips, vec!["10.0.0.1/24".to_string(), "10.0.0.2/24".to_string()]);
        assert_eq!(config.dns, None);
        clear_env_vars();
    }
}