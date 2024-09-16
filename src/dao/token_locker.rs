#![cfg_attr(not(feature = "std"), no_std)]

use crate::dependencies::system_start::SystemStart;
use crate::dependencies::babel_ownable::BabelOwnable;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct TokenLocker {
    lock_to_token_ratio: u64,
    total_decay_rate: u32,
    total_updated_week: u16,
    account_data: HashMap<AccountId, AccountData>,
    system_start: SystemStart,
    babel_ownable: BabelOwnable,
}

#[derive(Default, Debug)]
struct AccountData {
    locked: u32,
    unlocked: u32,
    frozen: u32,
    week: u16,
    update_weeks: Vec<u32>, // Bitfield for weekly unlocks
}

impl TokenLocker {
    fn new(lock_to_token_ratio: u64, system_start: SystemStart, babel_ownable: BabelOwnable) -> Self {
        TokenLocker {
            lock_to_token_ratio,
            total_decay_rate: 0,
            total_updated_week: 0,
            account_data: HashMap::new(),
            system_start,
            babel_ownable,
        }
    }

    fn lock(&mut self, account: AccountId, amount: u32, weeks: u16) {
        let account_data = self.account_data.entry(account).or_insert_with(AccountData::default);
        account_data.locked += amount;
        account_data.week = weeks;
        // Ensure the vector can hold the week index
        if (weeks as usize) >= account_data.update_weeks.len() {
            account_data.update_weeks.resize(weeks as usize + 1, 0);
        }
        account_data.update_weeks[weeks as usize] |= 1 << (weeks % 32);
        self.transfer_tokens(account, amount); // Token transfer logic
    }

    fn unlock(&mut self, account: AccountId) {
        if let Some(account_data) = self.account_data.get_mut(&account) {
            account_data.unlocked += account_data.locked;
            account_data.locked = 0;
            // Reset the bitfield for the week
            account_data.update_weeks[account_data.week as usize] &= !(1 << (account_data.week % 32));
        }
    }

    fn get_account_balances(&self, account: AccountId) -> (u32, u32) {
        self.account_data.get(&account).map_or((0, 0), |data| (data.locked, data.unlocked))
    }

   

    fn calculate_weight(&self, account: AccountId) -> u64 {
        self.calculate_weight_at(account, self.get_week())
    }

    fn calculate_weight_at(&self, account: AccountId, week: u16) -> u64 {
        if week > self.get_week() {
            return 0;
        }
        let account_data = self.account_data.get(&account).unwrap();
        let mut account_week = account_data.week;
        let mut weight = self.account_weekly_weights[&account][account_week as usize];
        let mut locked = account_data.locked as u64;

        if locked == 0 || account_data.frozen > 0 {
            return weight;
        }

        let mut bitfield = account_data.update_weeks[account_week as usize / 32] >> (account_week % 32);
        while account_week < week {
            account_week += 1;
            weight -= locked;
            if account_week % 32 == 0 {
                bitfield = account_data.update_weeks[account_week as usize / 32];
            } else {
                bitfield >>= 1;
            }
            if bitfield & 1 == 1 {
                let amount = self.account_weekly_unlocks[&account][account_week as usize];
                locked -= amount as u64;
                if locked == 0 {
                    break;
                }
            }
        }
        weight
    }

    fn get_total_weight(&self) -> u64 {
        self.get_total_weight_at(self.get_week())
    }

    fn get_total_weight_at(&self, week: u16) -> u64 {
        if week > self.get_week() {
            return 0;
        }
        let mut updated_week = self.total_updated_week;
        let mut weight = self.total_weekly_weights[updated_week as usize];
        let mut rate = self.total_decay_rate as u64;

        while updated_week < self.get_week() {
            updated_week += 1;
            weight -= rate;
            rate -= self.total_weekly_unlocks[updated_week as usize] as u64;
        }
        weight
    }

    fn get_week(&self) -> u16 {
        let start_time = self.system_start.get_start_time(); // Assuming this method exists
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let week_duration = 7 * 24 * 60 * 60; // One week in seconds
        ((current_time - start_time) / week_duration) as u16
    }

    fn transfer_tokens(&self, account: AccountId, amount: u32) {
        // Assuming we have a method in BabelOwnable to transfer tokens
        self.babel_ownable.transfer_to_locker(account, amount * self.lock_to_token_ratio);
    }
}

type AccountId = u32; 