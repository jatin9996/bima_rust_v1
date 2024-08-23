use std::collections::HashMap;
use crate::dependecies::delegated_ops::DelegatedOps;
use crate::dependecies::system_start::SystemStart;
use crate::interfaces::token_locker::ITokenLocker;

type AccountId = String;
type Timestamp = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    week: u16,
    created_at: Timestamp,
    can_execute_after: Timestamp,
    current_weight: u64,
    required_weight: u64,   
    processed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    target: AccountId,
    data: Vec<u8>,
}

pub struct AdminVoting {
    token_locker: Box<dyn ITokenLocker>,
    babel_core: AccountId,
    proposal_data: HashMap<u32, Proposal>,
    proposal_payloads: HashMap<u32, Vec<Action>>,
    account_vote_weights: HashMap<(AccountId, u32), u64>,
    latest_proposal_timestamp: HashMap<AccountId, Timestamp>,
    min_create_proposal_pct: u32,
    passing_pct: u32,
    system_start: SystemStart,
}

impl AdminVoting {
    pub fn new(token_locker: Box<dyn ITokenLocker>, babel_core: AccountId, min_create_proposal_pct: u32, passing_pct: u32, system_start: SystemStart) -> Self {
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

    pub fn create_new_proposal(&mut self, account: AccountId, payload: Vec<Action>) {
        let current_time = self.system_start.get_week();
        let last_proposal_time = *self.latest_proposal_timestamp.get(&account).unwrap_or(&0);

        if current_time <= last_proposal_time + Self::min_time_between_proposals() {
            panic!("Minimum time between proposals not met");
        }

        let week = current_time;
        if week == 0 {
            panic!("No proposals in the first week");
        }

        let account_weight = self.token_locker.get_account_weight_at(account.clone(), week - 1).unwrap();
        let min_weight = self.min_create_proposal_weight(week - 1);

        if account_weight < min_weight {
            panic!("Not enough weight to propose");
        }

        let proposal_id = self.proposal_data.len() as u32;
        let new_proposal = Proposal {
            week: week as u16,
            created_at: current_time,
            can_execute_after: 0,
            current_weight: 0,
            required_weight: self.calculate_required_weight(week - 1, self.passing_pct),
            processed: false,
        };

        self.proposal_data.insert(proposal_id, new_proposal);
        self.proposal_payloads.insert(proposal_id, payload);
        self.latest_proposal_timestamp.insert(account, current_time);
    }

    fn calculate_required_weight(&self, week: u32, pct: u32) -> u64 {
        let total_weight = self.token_locker.get_total_weight_at(week).unwrap();
        (total_weight * pct as u64) / 10000
    }

    fn min_create_proposal_weight(&self, week: u32) -> u64 {
        let total_weight = self.token_locker.get_total_weight_at(week).unwrap();
        (total_weight * self.min_create_proposal_pct as u64) / 10000
    }

    fn min_time_between_proposals() -> Timestamp {
        24 * 60 * 60 // 24 hours
    }
}