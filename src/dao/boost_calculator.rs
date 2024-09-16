#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::dependencies::system_start::SystemStart; // Import SystemStart

pub struct BoostCalculator {
    locker: AccountId,
    max_boost_grace_weeks: u32,
    account_weekly_lock_pct: HashMap<(AccountId, u32), u64>,
    total_weekly_weights: HashMap<u32, u64>,
}

impl BoostCalculator {
    pub fn new(locker: AccountId, grace_weeks: u32, system_start: SystemStart) -> Self {
        let current_week = system_start.get_week(); // Use SystemStart for getting the current week
        BoostCalculator {
            locker,
            max_boost_grace_weeks: grace_weeks + current_week,
            account_weekly_lock_pct: HashMap::new(),
            total_weekly_weights: HashMap::new(),
        }
    }

    pub fn get_week(&self, system_start: &SystemStart) -> u32 {
        system_start.get_week() as u32
    }

    pub fn get_boosted_amount(&self, account: AccountId, amount: u64, previous_amount: u64, total_weekly_emissions: u64) -> u64 {
        let week = Self::get_week();
        if week < self.max_boost_grace_weeks {
            return amount;
        }

        let adjusted_week = week - 1;
        let account_weight = self.account_weekly_lock_pct.get(&(account, adjusted_week)).copied().unwrap_or(0);
        let total_weight = self.total_weekly_weights.get(&adjusted_week).copied().unwrap_or(1);
        let pct = 1_000_000_000 * account_weight / total_weight;

        self.calculate_adjusted_amount(amount, previous_amount, total_weekly_emissions, pct)
    }

    fn calculate_adjusted_amount(&self, amount: u64, previous_amount: u64, total_weekly_emissions: u64, pct: u64) -> u64 {
        let base_amount = amount * pct / 1_000_000_000;
        if total_weekly_emissions > 0 {
            let emission_factor = previous_amount * 1_000_000_000 / total_weekly_emissions;
            base_amount * emission_factor / 1_000_000_000
        } else {
            base_amount
        }
    }
}

// Define traits for BoostCalculator and TokenLocker
pub trait IBoostCalculator {
    fn get_boosted_amount(&self, account: AccountId, amount: u64, previous_amount: u64, total_weekly_emissions: u64) -> u64;
}

pub trait ITokenLocker {
    fn lock_tokens(&self, account: AccountId, amount: u64);
    fn claim_tokens(&self, account: AccountId) -> u64;
}