#![no_std]

use core::collections::BTeeMap;

const MAX_POINTS: u16 = 10000;
const MAX_LOCK_WEEKS: u8 = 52;

pub struct IncentiveVoting {
    token_locker: AccountId,
    vault: AccountId,
    account_lock_data: BTreeMap<AccountId, AccountData>,
    receiver_count: u128,
    receiver_decay_rate: Vec<u32>,
    receiver_updated_week: Vec<u16>,
    receiver_weekly_weights: Vec<Vec<u64>>,
    receiver_weekly_unlocks: Vec<Vec<u32>>,
    total_decay_rate: u32,
    total_updated_week: u16,
    total_weekly_weights: Vec<u64>,
    total_weekly_unlocks: Vec<u32>,
    delegated_ops: AccountId,
    system_start: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AccountData {
    week: u16,
    frozen_weight: u64,
    points: u16,
    lock_length: u8,
    vote_length: u16,
    active_votes: Vec<(u16, u16)>,
    locked_amounts: Vec<u32>,
    weeks_to_unlock: Vec<u8>,
}

impl IncentiveVoting {
    pub fn new(token_locker: AccountId, vault: AccountId, delegated_ops: AccountId, system_start: u64) -> Self {
        Self {
            token_locker,
            vault,
            account_lock_data: BTreeMap::new(),
            receiver_count: 0,
            receiver_decay_rate: vec![0; 65535],
            receiver_updated_week: vec![0; 65535],
            receiver_weekly_weights: vec![vec![0; 65535]; 65535],
            receiver_weekly_unlocks: vec![vec![0; 65535]; 65535],
            total_decay_rate: 0,
            total_updated_week: 0,
            total_weekly_weights: vec![0; 65535],
            total_weekly_unlocks: vec![0; 65535],
            delegated_ops,
            system_start,
        }
    }

    pub fn register_account_weight(&mut self, account: AccountId, min_weeks: u64) {
        // Ensure caller or delegated
        // Get lock data
        let account_data = self.account_lock_data.get_mut(&account).unwrap();
        let existing_votes = if account_data.vote_length > 0 {
            self.get_account_current_votes(account)
        } else {
            vec![]
        };

        // Clear previous votes if any
        if !existing_votes.is_empty() {
            self.remove_vote_weights(account, &existing_votes, account_data.frozen_weight);
        }

        // Get updated account lock weights and store locally
        let frozen_weight = self.register_account_weight_internal(account, min_weeks);

        // Resubmit the account's active vote using the newly registered weights
        self.add_vote_weights(account, &existing_votes, frozen_weight);
        // Emit event
    }

    pub fn vote(&mut self, account: AccountId, votes: Vec<Vote>, clear_previous: bool) {
        let account_data = self.account_lock_data.get_mut(&account).unwrap();
        let frozen_weight = account_data.frozen_weight;
        assert!(frozen_weight > 0 || account_data.lock_length > 0, "No registered weight");

        let mut points = 0;
        let mut offset = 0;

        // Optionally clear previous votes
        if clear_previous {
            self.remove_vote_weights(account, &self.get_account_current_votes(account), frozen_weight);
            // Emit event
        } else {
            points = account_data.points;
            offset = account_data.vote_length;
        }

        // Adjust vote weights based on the new vote
        self.add_vote_weights(account, &votes, frozen_weight);
        // Store the new account votes
        self.store_account_votes(account, account_data, &votes, points, offset);
        // Emit event
    }

    fn register_account_weight_internal(&mut self, account: AccountId, min_weeks: u64) -> u64 {
        let account_data = self.account_lock_data.get_mut(&account).unwrap();

        // Get updated account lock weights and store locally
        let (lock_data, frozen) = self.token_locker.get_account_active_locks(account, min_weeks);
        let length = lock_data.len();
        if frozen > 0 {
            let frozen_weight = frozen * MAX_LOCK_WEEKS as u64;
            account_data.frozen_weight = frozen_weight;
        } else if length > 0 {
            for (i, lock) in lock_data.iter().enumerate() {
                account_data.locked_amounts[i] = lock.amount;
                account_data.weeks_to_unlock[i] = lock.weeks_to_unlock;
            }
        } else {
            panic!("No active locks");
        }
        let week = self.get_week();
        account_data.week = week;
        account_data.lock_length = length as u8;

        // Emit AccountWeightRegistered event
        self.emit_event(Event::AccountWeightRegistered {
            account,
            week,
            frozen_weight: account_data.frozen_weight,
            lock_data,
        });

        account_data.frozen_weight
    }

    fn add_vote_weights(&mut self, account: AccountId, votes: &[Vote], frozen_weight: u64) {
        let current_week = self.get_week();
        let account_data = self.account_lock_data.get_mut(&account).unwrap();

        for vote in votes {
            let vote_id = vote.id as usize;
            let vote_points = vote.points as u64;

            // Update receiver weekly weights
            for week in current_week..(current_week + account_data.lock_length as u64) {
                let week_index = week as usize;
                self.receiver_weekly_weights[vote_id][week_index] += vote_points * frozen_weight;
            }

            // Update total weekly weights
            for week in current_week..(current_week + account_data.lock_length as u64) {
                let week_index = week as usize;
                self.total_weekly_weights[week_index] += vote_points * frozen_weight;
            }
        }

        // Emit VotesUpdated event
        self.emit_event(Event::VotesUpdated {
            account,
            week: current_week as u16,
            votes: votes.to_vec(),
            points: account_data.points,
        });
    }

    fn remove_vote_weights(&mut self, account: AccountId, votes: &[Vote], frozen_weight: u64) {
        let current_week = self.get_week();
        let account_data = self.account_lock_data.get_mut(&account).unwrap();

        for vote in votes {
            let vote_id = vote.id as usize;
            let vote_points = vote.points as u64;

            // Update receiver weekly weights
            for week in current_week..(current_week + account_data.lock_length as u64) {
                let week_index = week as usize;
                self.receiver_weekly_weights[vote_id][week_index] -= vote_points * frozen_weight;
            }

            // Update total weekly weights
            for week in current_week..(current_week + account_data.lock_length as u64) {
                let week_index = week as usize;
                self.total_weekly_weights[week_index] -= vote_points * frozen_weight;
            }
        }

        // Emit ClearedVotes event
        self.emit_event(Event::ClearedVotes {
            account,
            week: current_week as u16,
        });
    }

    fn store_account_votes(&mut self, account: AccountId, account_data: &mut AccountData, votes: &[Vote], points: u16, offset: u16) {
        // Clear previous votes if offset is zero
        if offset == 0 {
            account_data.active_votes.clear();
        }

        // Update account data with new votes
        for (i, vote) in votes.iter().enumerate() {
            if offset as usize + i < account_data.active_votes.len() {
                account_data.active_votes[offset as usize + i] = (vote.id as u16, vote.points as u16);
            } else {
                account_data.active_votes.push((vote.id as u16, vote.points as u16));
            }
        }

        // Update points and vote length
        account_data.points = points + votes.iter().map(|v| v.points as u16).sum::<u16>();
        account_data.vote_length = account_data.active_votes.len() as u16;

        // Emit AccountVotesStored event
        self.emit_event(Event::AccountVotesStored {
            account,
            votes: votes.to_vec(),
            points: account_data.points,
        });
    }

    fn get_account_current_votes(&self, account: AccountId) -> Vec<Vote> {
        if let Some(account_data) = self.account_lock_data.get(&account) {
            account_data.active_votes.iter().map(|(id, points)| Vote {
                id: *id as u128,
                points: *points as u128,
            }).collect()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    id: u128,
    points: u128,
}

// Event definitions
pub enum Event {
    AccountWeightRegistered {
        account: AccountId,
        week: u16,
        frozen_weight: u64,
        lock_data: Vec<LockData>,
    },
    VotesUpdated {
        account: AccountId,
        week: u16,
        votes: Vec<Vote>,
        points: u16,
    },
    ClearedVotes {
        account: AccountId,
        week: u16,
    },
    AccountVotesStored {
        account: AccountId,
        votes: Vec<Vote>,
        points: u16,
    },
}

pub struct LockData {
    amount: u32,
    weeks_to_unlock: u8,
}