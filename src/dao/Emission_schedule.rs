#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::collections::VecDeque;

#[ink::contract]
mod emission_schedule {
    use super::*;

    #[ink(storage)]
    pub struct EmissionSchedule {
        owner: AccountId,
        system_start: Timestamp,
        vault: AccountId,
        voter: AccountId,
        lock_weeks: u64,
        lock_decay_weeks: u64,
        weekly_pct: u64,
        scheduled_weekly_pct: VecDeque<(u64, u64)>,
    }

    impl EmissionSchedule {
        #[ink(constructor)]
        pub fn new(
            owner: AccountId,
            system_start: Timestamp,
            vault: AccountId,
            voter: AccountId,
            initial_lock_weeks: u64,
            lock_decay_weeks: u64,
            weekly_pct: u64,
            scheduled_weekly_pct: Vec<(u64, u64)>
        ) -> Self {
            ink_env::assert(initial_lock_weeks <= MAX_LOCK_WEEKS, "Cannot exceed MAX_LOCK_WEEKS");
            ink_env::assert(lock_decay_weeks > 0, "Decay weeks cannot be 0");
            ink_env::assert(weekly_pct <= MAX_PCT, "Cannot exceed MAX_PCT");

            Self {
                owner,
                system_start,
                vault,
                voter,
                lock_weeks: initial_lock_weeks,
                lock_decay_weeks,
                weekly_pct,
                scheduled_weekly_pct: scheduled_weekly_pct.into_iter().collect(),
            }
        }

        #[ink(message)]
        pub fn set_weekly_pct_schedule(&mut self, schedule: Vec<(u64, u64)>) {
            self.only_owner();
            let mut last_week = u64::MAX;
            for &(week, pct) in &schedule {
                ink_env::assert(week < last_week, "Must sort by week descending");
                ink_env::assert(pct <= MAX_PCT, "Cannot exceed MAX_PCT");
                last_week = week;
            }
            self.scheduled_weekly_pct = schedule.into_iter().collect();
        }

        #[ink(message)]
        pub fn get_weekly_pct(&self, current_week: u64) -> u64 {
            let week = self.system_start.get_week();
            for &(week, pct) in self.scheduled_weekly_pct.iter().rev() {
                if current_week >= week {
                    return pct;
                }
            }
            self.weekly_pct
        }

        #[ink(message)]
        pub fn lock(&mut self, weeks: u64) {
            self.only_owner();
            ink_env::assert(weeks <= MAX_LOCK_WEEKS, "Lock duration exceeds maximum allowed weeks");
            self.lock_weeks = weeks;
        }

        #[ink(message)]
        pub fn unlock(&mut self) {
            self.only_owner();
            self.lock_weeks = 0;
        }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Only owner can call this function");
        }
    }

    const MAX_PCT: u64 = 10000;
    const MAX_LOCK_WEEKS: u64 = 52;
}