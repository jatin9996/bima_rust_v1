#![cfg_attr(not(feature = "std"), no_std)]

use log::{info, warn};
use crate::interfaces::babel_core::BabelCore; // Ensure BabelCore is in scope

struct Proposal {
    created_at: u64,
    can_execute_after: u64,
    processed: bool,
}

struct Action {
    target: String, // Assuming target is a string identifier
    data: Vec<u8>,
}

struct InterimAdmin {
    babel_core: String,
    proposals: Vec<Proposal>,
    proposal_payloads: Vec<Vec<Action>>,
    daily_proposals_count: Vec<u32>,
    guardian: Option<String>,
    owner: String, // Ownership management
}

impl InterimAdmin {
    pub fn new(babel_core: String, owner: String) -> Self {
        Self {
            babel_core,
            proposals: Vec::new(),
            proposal_payloads: Vec::new(),
            daily_proposals_count: vec![0; 365], // Tracking for each day of the year
            guardian: None,
            owner,
        }
    }

    pub fn set_guardian(&mut self, caller: &str, guardian: String) {
        if self.is_owner(caller) && self.guardian.is_none() {
            self.guardian = Some(guardian);
            info!("Guardian set");
        } else {
            warn!("Guardian already set or unauthorized access");
        }
    }

    pub fn create_new_proposal(&mut self, caller: &str, payload: Vec<Action>) {
        if !self.is_owner(caller) {
            warn!("Unauthorized attempt to create proposal");
            return;
        }

        let current_time = self.get_current_time();
        let day = (current_time / 86400) as usize;
        assert!(self.daily_proposals_count[day] < 3, "MAX_DAILY_PROPOSALS reached");

        let proposal_id = self.proposals.len();
        self.proposals.push(Proposal {
            created_at: current_time,
            can_execute_after: current_time + 86400,
            processed: false,
        });
        self.proposal_payloads.push(payload);
        self.daily_proposals_count[day] += 1;
        info!("Proposal {} created", proposal_id);
    }

    pub fn cancel_proposal(&mut self, caller: &str, index: usize) {
        if let Some(guardian) = &self.guardian {
            if (caller == guardian || self.is_owner(caller)) && !self.proposals[index].processed {
                self.proposals[index].processed = true;
                info!("Proposal {} cancelled by {}", index, caller);
            } else {
                warn!("Unauthorized or already processed");
            }
        }
    }

    fn get_current_time(&self) -> u64 {
        //  for actual time fetching logic
        0
    }

    fn is_owner(&self, caller: &str) -> bool {
        self.owner == caller
    }
}