use std::collections::HashMap;
use crate::interfaces::token_locker::ITokenLocker; // Import the ITokenLocker trait
use crate::dependecies::system_start::SystemStart; // Import the SystemStart contract

struct BoostCalculator {
    locker: Box<dyn ITokenLocker>, // Use a trait object for locker
    max_boost_grace_weeks: u32,
    account_weekly_lock_pct: HashMap<(String, u32), u32>, // (account, week) -> pct
    total_weekly_weights: HashMap<u32, u64>,             // week -> total weekly lock weight
}

impl BoostCalculator {
    fn new(locker: Box<dyn ITokenLocker>, grace_weeks: u32) -> Self {
        BoostCalculator {
            locker,
            max_boost_grace_weeks: grace_weeks + Self::get_week(),
            account_weekly_lock_pct: HashMap::new(),
            total_weekly_weights: HashMap::new(),
        }
    }

    fn get_week() -> u32 {
        // Use SystemStart to get the current week
        SystemStart::get_week() as u32
    }

    fn get_boosted_amount(&self, account: &str, amount: u64, previous_amount: u64, total_weekly_emissions: u64) -> u64 {
        let week = Self::get_week();
        if week < self.max_boost_grace_weeks {
            return amount;
        }

        let adjusted_week = week - 1;
        let account_weight = self.locker.get_account_weight_at(account.to_string(), adjusted_week).unwrap_or(0); // Adjusted to use ITokenLocker
        let total_weight = self.locker.get_total_weight_at(adjusted_week).unwrap_or(1); // Adjusted to use ITokenLocker
        let pct = 1_000_000_000 * account_weight / total_weight;

        self.calculate_adjusted_amount(amount, previous_amount, total_weekly_emissions, pct)
    }

    fn calculate_adjusted_amount(&self, amount: u64, previous_amount: u64, total_weekly_emissions: u64, pct: u64) -> u64 {
        // Implement the logic to calculate the adjusted amount based on the boost
        
    }
}