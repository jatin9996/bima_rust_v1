use std::collections::HashMap;


trait ITroveManager {}
trait ISortedTroves {}

struct DeploymentParams {
    minute_decay_factor: u128,
    redemption_fee_floor: u128,
    max_redemption_fee: u128,
    borrowing_fee_floor: u128,
    max_borrowing_fee: u128,
    interest_rate_in_bps: u128,
    max_debt: u128,
    mcr: u128,
}

struct Factory {
    debt_token: String, 
    stability_pool: String, 
    liquidation_manager: String, 
    borrower_operations: String, 

    sorted_troves_impl: String,
    trove_manager_impl: String,

    trove_managers: Vec<String>, 
}

impl Factory {
    fn new(
        debt_token: String,
        stability_pool: String,
        borrower_operations: String,
        sorted_troves_impl: String,
        trove_manager_impl: String,
        liquidation_manager: String,
    ) -> Self {
        Factory {
            debt_token,
            stability_pool,
            liquidation_manager,
            borrower_operations,
            sorted_troves_impl,
            trove_manager_impl,
            trove_managers: Vec::new(),
        }
    }

    fn deploy_new_instance(&mut self, collateral: String, price_feed: String, params: DeploymentParams) {
        let trove_manager = self.clone_contract(&self.trove_manager_impl);
        self.trove_managers.push(trove_manager);

        let sorted_troves = self.clone_contract(&self.sorted_troves_impl);

        // Simulate setting up the new instance
        println!("Deployed new TroveManager and SortedTroves for collateral: {}", collateral);
    }

    fn clone_contract(&mut self, implementation: &String) -> String {
        // Generate a new unique identifier for the cloned contract
        let new_id = format!("{}_instance_{}", implementation, self.trove_managers.len() + 1);

        // Simulate cloning the state from the original implementation
        let original_state = unsafe {
            CONTRACT_STATES.get(implementation).cloned().unwrap_or_default()
        };

        // Insert the cloned state into the global map under the new ID
        unsafe {
            CONTRACT_STATES.insert(new_id.clone(), original_state);
        }

        new_id
    }
}

// Assuming a global HashMap to simulate a database of contract states
static mut CONTRACT_STATES: HashMap<String, HashMap<String, String>> = HashMap::new();