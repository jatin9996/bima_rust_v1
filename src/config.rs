use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub network_endpoint: String,
    pub feature_toggle: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            network_endpoint: "https://rpc.arch.network".to_string(),
            feature_toggle: true,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_config() {
        let config = Config::new();
        assert_eq!(config.network_endpoint, "https://rpc.arch.network");
        assert!(config.feature_toggle);
    }

    #[test]
    fn test_load_from_file() {
        let config = Config {
            network_endpoint: "https://test.endpoint".to_string(),
            feature_toggle: false,
        };
        let file = NamedTempFile::new().unwrap();
        let config_str = serde_json::to_string(&config).unwrap();
        fs::write(file.path(), config_str).unwrap();

        let loaded_config = Config::load_from_file(file.path().to_str().unwrap()).unwrap();
        assert_eq!(loaded_config.network_endpoint, "https://test.endpoint");
        assert!(!loaded_config.feature_toggle);
    }

    #[test]
    fn test_load_from_file_error() {
        let result = Config::load_from_file("non_existent_file.json");
        assert!(result.is_err());
    }
}