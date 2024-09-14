use borsh::{BorshDeserialize, BorshSerialize};
use crate::models::params::{BoostParams, WeekParams};
use crate::utils::crypto;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BoostCalculator {
    // Define the properties of the BoostCalculator
    pub locker: Vec<u8>, // Public key or identifier for the locker
    pub max_boost_grace_weeks: u32,
    pub account_weekly_lock_pct: HashMap<(Vec<u8>, u32), u32>, // AccountId replaced by Vec<u8>
    pub total_weekly_weights: HashMap<u32, u64>,
}

impl BoostCalculator {
    pub fn new(locker: Vec<u8>, grace_weeks: u32) -> Self {
        Self {
            locker,
            max_boost_grace_weeks: grace_weeks,
            account_weekly_lock_pct: HashMap::new(),
            total_weekly_weights: HashMap::new(),
        }
    }

    pub fn get_week(current_time: u64) -> u32 {
        let start_date = 1_600_000_000; // start date in UNIX timestamp (seconds)
        let seconds_per_week = 60 * 60 * 24 * 7;
        ((current_time - start_date) / seconds_per_week) as u32
    }

    pub fn get_boosted_amount(&self, params: BoostParams) -> u64 {
        let week = Self::get_week(params.current_time);
        if week < self.max_boost_grace_weeks {
            return params.amount;
        }

        let adjusted_week = week - 1;
        let account_weight = self.account_weekly_lock_pct.get(&(params.account.clone(), adjusted_week)).copied().unwrap_or(0);
        let total_weight = self.total_weekly_weights.get(&adjusted_week).copied().unwrap_or(1);
        let pct = 1_000_000_000 * account_weight / total_weight;

        self.calculate_adjusted_amount(params.amount, params.previous_amount, params.total_weekly_emissions, pct)
    }

    fn calculate_adjusted_amount(&self, amount: u64, previous_amount: u64, total_weekly_emissions: u64, pct: u64) -> u64 {
        let base_amount = amount * pct / 1_000_000_000;
        if total_weekly_emissions > 0 {
            let emission_factor = previous_amount * 1_000_000_000 / total_weekly_emissions;
            base_amount * emission_factor / 1_000_000_000
        } else {
            base_amount // If no emissions, return the base amount
        }
    }
}
