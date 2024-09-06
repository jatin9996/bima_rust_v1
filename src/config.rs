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
