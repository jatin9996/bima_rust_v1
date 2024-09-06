pub struct NetworkConfig {
    pub rpc_url: String,
}

impl NetworkConfig {
    pub fn arch_network() -> Self {
        Self {
            rpc_url: "https://rpc.arch.network".to_string(),
        }
    }
}