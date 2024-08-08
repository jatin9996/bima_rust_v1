use std::collections::HashMap;

pub struct StabilityPool {
    deposits: HashMap<String, u64>, // User deposits mapped by user ID
    total_stablecoins: u64,         // Total stablecoins held in the pool
}

impl StabilityPool {
    pub fn new() -> Self {
        Self {
            deposits: HashMap::new(),
            total_stablecoins: 0,
        }
    }

    pub fn deposit(&mut self, user_id: String, amount: u64) {
        let current_deposit = self.deposits.entry(user_id).or_insert(0);
        *current_deposit += amount;
        self.total_stablecoins += amount;
    }

    pub fn withdraw(&mut self, user_id: String, amount: u64) -> bool {
        if let Some(current_deposit) = self.deposits.get_mut(&user_id) {
            if *current_deposit >= amount {
                *current_deposit -= amount;
                self.total_stablecoins -= amount;
                return true;
            }
        }
        false
    }
}
