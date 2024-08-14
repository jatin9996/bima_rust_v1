use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct AccountId(String);  // Simplified representation of an account ID

struct LiquidationManager {
    stability_pool: AccountId,
    borrower_operations: AccountId,
    factory: AccountId,
    enabled_trove_managers: HashMap<AccountId, bool>,
}

struct Liquidation {
    borrower: AccountId,
    liquidated_debt: f64,  // Assuming debt and collateral are represented as f64 for simplicity
    liquidated_coll: f64,
}

impl LiquidationManager {
    fn new(stability_pool: AccountId, borrower_operations: AccountId, factory: AccountId) -> Self {
        Self {
            stability_pool,
            borrower_operations,
            factory,
            enabled_trove_managers: HashMap::new(),
        }
    }

    fn enable_trove_manager(&mut self, trove_manager: AccountId) {
        // Simulating only the factory can enable a trove manager
        // This would be where you check the caller in a real smart contract
        self.enabled_trove_managers.insert(trove_manager, true);
    }

    fn liquidate(&mut self, trove_manager: AccountId, borrower: AccountId) -> Option<Liquidation> {
        if !self.enabled_trove_managers.get(&trove_manager).unwrap_or(&false) {
            return None;  // Trove manager not approved, no liquidation occurs
        }

        // Simulate liquidation logic
        let liquidation = Liquidation {
            borrower,
            liquidated_debt: 1000.0,  // Example values
            liquidated_coll: 500.0,
        };

        Some(liquidation)
    }
}

fn main() {
    let mut manager = LiquidationManager::new(
        AccountId("stability_pool".to_string()),
        AccountId("borrower_operations".to_string()),
        AccountId("factory".to_string()),
    );

    let trove_manager = AccountId("trove_manager_1".to_string());
    manager.enable_trove_manager(trove_manager.clone());

    if let Some(liquidation) = manager.liquidate(trove_manager, AccountId("borrower_1".to_string())) {
        println!("Liquidation occurred: {:?}", liquidation);
    } else {
        println!("Liquidation failed: Trove manager not approved or other error");
    }
}