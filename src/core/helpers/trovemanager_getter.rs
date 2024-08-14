use std::collections::HashMap;

struct Collateral {
    collateral: String,
    trove_managers: Vec<String>,
}

struct Factory {
    trove_managers: Vec<String>,
}

impl Factory {
    fn trove_manager_count(&self) -> usize {
        self.trove_managers.len()
    }

    fn trove_managers(&self, index: usize) -> &String {
        &self.trove_managers[index]
    }
}

struct TroveManagerGetters {
    factory: Factory,
}

impl TroveManagerGetters {
    fn new(factory: Factory) -> Self {
        TroveManagerGetters { factory }
    }

    fn get_all_collaterals_and_trove_managers(&self) -> Vec<Collateral> {
        let length = self.factory.trove_manager_count();
        let mut unique_collaterals = HashMap::new();

        for i in 0..length {
            let trove_manager = self.factory.trove_managers(i);
            let collateral = self.get_collateral_token(trove_manager);

            unique_collaterals.entry(collateral.clone())
                .or_insert_with(Vec::new)
                .push(trove_manager.clone());
        }

        unique_collaterals.into_iter()
            .map(|(collateral, trove_managers)| Collateral { collateral, trove_managers })
            .collect()
    }

    fn get_active_trove_managers_for_account(&self, account: &str) -> Vec<String> {
        let length = self.factory.trove_manager_count();
        let mut trove_managers = Vec::new();

        for i in 0..length {
            let trove_manager = self.factory.trove_managers(i);
            if self.get_trove_status(trove_manager, account) > 0 {
                trove_managers.push(trove_manager.clone());
            }
        }

        trove_managers
    }

    //  the collateral token retrieval
    fn get_collateral_token(&self, trove_manager: &String) -> String {
        let mut trove_to_collateral: HashMap<String, String> = HashMap::new();
        
        trove_to_collateral.insert("trove_manager_1".to_string(), "ETH".to_string());
        trove_to_collateral.insert("trove_manager_2".to_string(), "BTC".to_string());
        trove_to_collateral.insert("trove_manager_3".to_string(), "DAI".to_string());

        // Retrieve the collateral token based on the trove manager
        trove_to_collateral.get(trove_manager).cloned().unwrap_or_else(|| "Unknown".to_string())
    }

    //  the trove status retrieval
    fn get_trove_status(&self, trove_manager: &String, account: &str) -> i32 {
        let mut trove_status: HashMap<(String, String), i32> = HashMap::new();
        
        trove_status.insert(("trove_manager_1".to_string(), "account_1".to_string()), 1);
        trove_status.insert(("trove_manager_2".to_string(), "account_2".to_string()), 0);
        trove_status.insert(("trove_manager_3".to_string(), "account_3".to_string()), 1);

        // Retrieve the trove status based on the trove manager and account
        *trove_status.get(&(trove_manager.clone(), account.to_string())).unwrap_or(&0)
    }
}