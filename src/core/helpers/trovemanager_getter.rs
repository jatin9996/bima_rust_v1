use std::collections::HashMap;

pub struct TroveManagerGetters {
    trove_managers: HashMap<u32, String>,
    trove_to_collateral: HashMap<String, String>,
    trove_status: HashMap<(String, String), i32>,
}

impl TroveManagerGetters {
    pub fn new() -> Self {
        let mut trove_to_collateral = HashMap::new();
        let mut trove_status = HashMap::new();

        trove_to_collateral.insert("trove_manager_1".to_string(), "ETH".to_string());
        trove_to_collateral.insert("trove_manager_2".to_string(), "BTC".to_string());
        trove_to_collateral.insert("trove_manager_3".to_string(), "DAI".to_string());

        trove_status.insert(("trove_manager_1".to_string(), "account_1".to_string()), 1);
        trove_status.insert(("trove_manager_2".to_string(), "account_2".to_string()), 0);
        trove_status.insert(("trove_manager_3".to_string(), "account_3".to_string()), 1);

        TroveManagerGetters {
            trove_managers: HashMap::new(),
            trove_to_collateral,
            trove_status,
        }
    }

    pub fn get_collateral_token(&self, trove_manager: &str) -> String {
        self.trove_to_collateral.get(trove_manager).cloned().unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn get_trove_status(&self, trove_manager: &str, account: &str) -> i32 {
        *self.trove_status.get(&(trove_manager.to_string(), account.to_string())).unwrap_or(&0)
    }

    pub fn get_active_trove_managers_for_account(&self, account: &str) -> Vec<String> {
        let mut active_managers = Vec::new();
        for (key, value) in self.trove_status.iter() {
            if key.1 == account && *value > 0 {
                active_managers.push(key.0.clone());
            }
        }
        active_managers
    }
}