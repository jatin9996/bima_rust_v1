use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const MIN_TIME_TO_EXECUTION: u64 = 86400; // 1 day in seconds
const MAX_TIME_TO_EXECUTION: u64 = 1814400; // 3 weeks in seconds
const MAX_DAILY_PROPOSALS: u32 = 3;

struct Action {
    target: String, // Simplified representation
    data: Vec<u8>,
}

struct Proposal {
    created_at: u64,
    can_execute_after: u64,
    processed: bool,
}

struct InterimAdmin {
    proposals: Vec<Proposal>,
    proposal_payloads: HashMap<usize, Vec<Action>>,
    daily_proposals_count: HashMap<u64, u32>,
}

impl InterimAdmin {
    fn new() -> Self {
        InterimAdmin {
            proposals: Vec::new(),
            proposal_payloads: HashMap::new(),
            daily_proposals_count: HashMap::new(),
        }
    }

    fn create_new_proposal(&mut self, payload: Vec<Action>) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let day = current_time / 86400;
        let current_daily_count = *self.daily_proposals_count.get(&day).unwrap_or(&0);
        assert!(current_daily_count < MAX_DAILY_PROPOSALS, "MAX_DAILY_PROPOSALS reached");

        let proposal_index = self.proposals.len();
        self.proposals.push(Proposal {
            created_at: current_time,
            can_execute_after: current_time + MIN_TIME_TO_EXECUTION,
            processed: false,
        });
        self.proposal_payloads.insert(proposal_index, payload);
        self.daily_proposals_count.insert(day, current_daily_count + 1);
    }

    // Additional methods like `execute_proposal`, `cancel_proposal`, etc.
}

fn main() {
    let mut admin = InterimAdmin::new();
    
    admin.create_new_proposal(vec![Action {
        target: "SomeAddress".to_string(),
        data: vec![],
    }]);
}