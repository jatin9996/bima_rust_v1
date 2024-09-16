#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::interfaces::token_locker::ITokenLocker;
use crate::interfaces::babel_core::BabelCore;

pub struct AdminVoting {
    token_locker: Box<dyn ITokenLocker>,
    babel_core: Box<dyn BabelCore>,
    proposal_data: HashMap<u32, Proposal>,
    proposal_payloads: HashMap<u32, Vec<Action>>,
    account_vote_weights: HashMap<(String, u32), u64>,
    latest_proposal_timestamp: HashMap<String, u64>,
    min_create_proposal_pct: u32,
    passing_pct: u32,
    system_start: u64,
}

pub struct Proposal {
    week: u16,
    created_at: u64,
    can_execute_after: u64,
    current_weight: u64,
    required_weight: u64,
    processed: bool,
}

pub struct Action {
    target: String,
    data: Vec<u8>,
}

impl AdminVoting {
    pub fn new(token_locker: Box<dyn ITokenLocker>, babel_core: Box<dyn BabelCore>, min_create_proposal_pct: u32, passing_pct: u32, system_start: u64) -> Self {
        Self {
            token_locker,
            babel_core,
            proposal_data: HashMap::new(),
            proposal_payloads: HashMap::new(),
            account_vote_weights: HashMap::new(),
            latest_proposal_timestamp: HashMap::new(),
            min_create_proposal_pct,
            passing_pct,
            system_start,
        }
    }

    pub fn create_new_proposal(&mut self, account: String, payload: Vec<Action>) {
        let current_time = self.get_current_time();
        let last_proposal_time = *self.latest_proposal_timestamp.get(&account).unwrap_or(&0);

        if current_time <= last_proposal_time + Self::min_time_between_proposals() {
            panic!("Minimum time between proposals not met");
        }

        let week = self.system_start; 
        if week == 0 {
            panic!("No proposals in the first week");
        }

        let account_weight = 1000; // Placeholder for actual weight fetching logic
        let min_weight = 500; // Placeholder for actual minimum weight calculation

        if account_weight < min_weight {
            panic!("Not enough weight to propose");
        }

        let proposal_id = self.proposal_data.len() as u32;
        let new_proposal = Proposal {
            week: week as u16,
            created_at: current_time,
            can_execute_after: 0,
            current_weight: 0,
            required_weight: 1000, // Placeholder for actual required weight calculation
            processed: false,
        };

        self.proposal_data.insert(proposal_id, new_proposal);
        self.proposal_payloads.insert(proposal_id, payload);
        self.latest_proposal_timestamp.insert(account, current_time);
    }

    pub fn vote_for_proposal(&mut self, account: String, proposal_id: u32, weight: u64) {
        let proposal = self.proposal_data.get_mut(&proposal_id).expect("Invalid proposal ID");

        if proposal.processed {
            panic!("Proposal already processed");
        }

        let current_time = Self::get_current_time();
        if current_time > proposal.created_at + Self::voting_period() {
            panic!("Voting period has closed");
        }

        let current_weight = self.account_vote_weights.entry((account.clone(), proposal_id)).or_insert(0);
        if *current_weight > 0 {
            panic!("Already voted");
        }

        *current_weight = weight;
        proposal.current_weight += weight;

        if proposal.current_weight >= proposal.required_weight {
            proposal.can_execute_after = current_time + Self::min_time_to_execution();
        }
    }

    pub fn execute_proposal(&mut self, proposal_id: u32) {
        let proposal = self.proposal_data.get_mut(&proposal_id).expect("Invalid proposal ID");

        let current_time = Self::get_current_time();
        if !proposal.processed && proposal.can_execute_after != 0 && proposal.can_execute_after <= current_time && current_time <= proposal.can_execute_after + Self::max_time_to_execution() {
            proposal.processed = true;
            // Execute the actions associated with the proposal
            let actions = self.proposal_payloads.get(&proposal_id).unwrap();
            for action in actions {
                // Placeholder for action execution logic
                println!("Executing action on target: {}", action.target);
            }
        } else {
            panic!("Proposal cannot be executed yet");
        }
    }

    fn min_time_between_proposals() -> u64 {
        24 * 60 * 60 // 24 hours
    }

    fn min_time_to_execution() -> u64 {
        24 * 60 * 60 // 24 hours, for actual minimum time to execution
    }

    fn voting_period() -> u64 {
        7 * 24 * 60 * 60 // 7 days
    }

    fn max_time_to_execution() -> u64 {
        3 * 7 * 24 * 60 * 60 // 3 weeks
    }

    fn get_current_time() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}