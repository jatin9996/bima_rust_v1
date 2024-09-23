use crate::babel_ownable::BabelOwnable;
use crate::curve_proxy::CurveProxy;
use crate::curve_deposit_token::CurveDepositToken;
use arch_program_sdk::ArchClient;
use std::collections::HashMap;

#[derive(Default)]
pub struct CurveFactory {
    babel_core: BabelOwnable,
    curve_proxy: CurveProxy,
    deposit_token_impl: CurveDepositToken,
    deployed_tokens: HashMap<String, String>,
    arch_client: ArchClient,
}

impl CurveFactory {
    pub fn new(
        babel_core: BabelOwnable,
        curve_proxy: CurveProxy,
        deposit_token_impl: CurveDepositToken,
        arch_client: ArchClient,
    ) -> Self {
        CurveFactory {
            babel_core,
            curve_proxy,
            deposit_token_impl,
            deployed_tokens: HashMap::new(),
            arch_client,
        }
    }

    /// Deploy a new instance of the CurveDepositToken with a deterministic address based on gauge
    /// After calling this function, the owner should also call `Vault.register_receiver`
    /// to enable BABEL emissions on the newly deployed CurveDepositToken
    pub fn deploy_new_instance(&mut self, gauge: String) -> Result<(), &'static str> {
        if !self.babel_core.is_owner() {
            return Err("Only owner can deploy new instances");
        }

        // Generate deterministic address for the new deployment
        let deposit_token_address = self.clone_deterministic(gauge.clone());

        // Initialize the new deposit token with the gauge address
        let mut new_token = self.deposit_token_impl.clone();
        new_token.initialize(gauge.clone());

        // Set per-gauge approval in the CurveProxy contract
        self.curve_proxy.set_per_gauge_approval(deposit_token_address.clone(), gauge.clone());

        // Store deployed token address in a hashmap
        self.deployed_tokens.insert(gauge.clone(), deposit_token_address.clone());

        // Emit event (simulated in Rust via logs or external system)
        println!("NewDeployment: Gauge: {}, DepositToken: {}", gauge, deposit_token_address);

        Ok(())
    }

    /// Predict the deterministic address of a deposit token based on the gauge address
    pub fn get_deposit_token(&self, gauge: String) -> Result<String, &'static str> {
        if let Some(token_address) = self.deployed_tokens.get(&gauge) {
            Ok(token_address.clone())
        } else {
            Err("Deposit token not found for given gauge")
        }
    }

    /// Clone the deposit token with a deterministic address
    fn clone_deterministic(&self, gauge: String) -> String {
        // Clone the deposit token using a deterministic approach (based on gauge)
        let hash_input = format!("{:?}", gauge);
        let deterministic_address = self.arch_client.generate_address_from_hash(hash_input);
        deterministic_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::babel_ownable::BabelOwnable;
    use crate::curve_proxy::CurveProxy;
    use crate::curve_deposit_token::CurveDepositToken;
    use arch_program_sdk::ArchClient;

    #[test]
    fn test_deploy_new_instance() {
        let babel_core = BabelOwnable::new("owner_address".to_string());
        let curve_proxy = CurveProxy::new();
        let deposit_token_impl = CurveDepositToken::new();
        let arch_client = ArchClient::new();

        let mut factory = CurveFactory::new(babel_core, curve_proxy, deposit_token_impl, arch_client);

        let gauge = "gauge_address_1".to_string();

        let result = factory.deploy_new_instance(gauge.clone());
        assert!(result.is_ok());

        let predicted_address = factory.get_deposit_token(gauge.clone());
        assert!(predicted_address.is_ok());
        println!("Predicted deposit token address: {:?}", predicted_address.unwrap());
    }
}