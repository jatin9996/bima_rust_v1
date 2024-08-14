use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const OWNERSHIP_TRANSFER_DELAY: u64 = 86400 * 3; // 3 days

struct BabelCore {
    fee_receiver: String,
    price_feed: String,
    owner: String,
    pending_owner: Option<String>,
    ownership_transfer_deadline: Option<u64>,
    guardian: String,
    paused: bool,
    start_time: u64,
}

impl BabelCore {
    fn new(owner: String, guardian: String, price_feed: String, fee_receiver: String) -> Self {
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        BabelCore {
            fee_receiver,
            price_feed,
            owner,
            pending_owner: None,
            ownership_transfer_deadline: None,
            guardian,
            paused: false,
            start_time: start_time - (start_time % (7 * 86400)), // Rounded down to the nearest week
        }
    }

    fn set_fee_receiver(&mut self, new_fee_receiver: String) {
        self.fee_receiver = new_fee_receiver;
    }

    fn set_price_feed(&mut self, new_price_feed: String) {
        self.price_feed = new_price_feed;
    }

    fn set_guardian(&mut self, new_guardian: String) {
        self.guardian = new_guardian;
    }

    fn set_paused(&mut self, new_paused: bool) {
        if new_paused && self.guardian != self.owner {
            panic!("Unauthorized");
        }
        self.paused = new_paused;
    }

    fn commit_transfer_ownership(&mut self, new_owner: String) {
        self.pending_owner = Some(new_owner);
        self.ownership_transfer_deadline = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + OWNERSHIP_TRANSFER_DELAY);
    }

    fn accept_transfer_ownership(&mut self) {
        if let Some(pending_owner) = &self.pending_owner {
            if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() >= self.ownership_transfer_deadline.unwrap() {
                self.owner = pending_owner.clone();
                self.pending_owner = None;
                self.ownership_transfer_deadline = None;
            } else {
                panic!("Deadline not passed");
            }
        } else {
            panic!("No pending owner");
        }
    }

    fn revoke_transfer_ownership(&mut self) {
        self.pending_owner = None;
        self.ownership_transfer_deadline = None;
    }
}
