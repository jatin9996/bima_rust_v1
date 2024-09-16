use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const OWNERSHIP_TRANSFER_DELAY: u64 = 86400 * 3; // 3 days

#[derive(Clone)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub struct BabelCore {
    utxos: HashMap<(Vec<u8>, u32), UTXO>,
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
    pub fn new(owner: String, guardian: String, price_feed: String, fee_receiver: String) -> Self {
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            utxos: HashMap::default(),
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

    pub fn set_fee_receiver(&mut self, new_fee_receiver: String) {
        self.fee_receiver = new_fee_receiver;
    }

    pub fn set_price_feed(&mut self, new_price_feed: String) {
        self.price_feed = new_price_feed;
    }

    pub fn set_guardian(&mut self, new_guardian: String) {
        self.guardian = new_guardian;
    }

    pub fn set_paused(&mut self, new_paused: bool) {
        if new_paused && self.guardian != self.owner {
            panic!("Unauthorized");
        }
        self.paused = new_paused;
    }

    pub fn commit_transfer_ownership(&mut self, caller: String, new_owner: String) {
        if self.is_owner(&caller) {
            self.pending_owner = Some(new_owner.clone());
            self.ownership_transfer_deadline = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + OWNERSHIP_TRANSFER_DELAY);
            println!("Ownership transfer committed to {} with deadline {}", new_owner, self.ownership_transfer_deadline.unwrap());
        } else {
            panic!("Unauthorized attempt to commit ownership transfer by {}", caller);
        }
    }

    pub fn accept_transfer_ownership(&mut self, caller: String) {
        if let Some(ref pending_owner) = self.pending_owner {
            if caller == *pending_owner && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() >= self.ownership_transfer_deadline.unwrap() {
                println!("Ownership transferred from {} to {}", self.owner, pending_owner);
                self.owner = pending_owner.clone();
                self.pending_owner = None;
                self.ownership_transfer_deadline = None;
            } else {
                panic!("Unauthorized or premature attempt to accept ownership by {}", caller);
            }
        } else {
            panic!("No pending owner");
        }
    }

    pub fn revoke_transfer_ownership(&mut self) {
        self.pending_owner = None;
        self.ownership_transfer_deadline = None;
    }

    pub fn transfer_utxo(&mut self, input_utxos: Vec<(Vec<u8>, u32)>, output_utxos: Vec<UTXO>) {
        let mut input_value = 0;
        for (txid, vout) in input_utxos.iter() {
            let utxo = self.utxos.get(&(*txid, *vout)).expect("UTXO not found");
            input_value += utxo.value;
            self.utxos.remove(&(*txid, *vout));
        }

        let mut output_value = 0;
        for utxo in output_utxos.iter() {
            output_value += utxo.value;
            let txid = utxo.txid.clone();
            let vout = utxo.vout;
            self.utxos.insert((txid, vout), utxo.clone());
        }

        if input_value != output_value {
            panic!("Input and output values do not match");
        }
    }

    pub fn adjust_trove(&mut self, user: String, adjustment: i64) {
        println!("Adjusting trove for user: {}, adjustment: {}", user, adjustment);
    }

    pub fn trigger_emergency(&mut self, caller: String) {
        if caller == self.guardian {
            self.paused = true;
            println!("Emergency triggered, system paused by {}", caller);
        } else {
            panic!("Unauthorized attempt to trigger emergency by {}", caller);
        }
    }

    pub fn admin_vote(&mut self, proposal_id: u32, vote: bool) {
        println!("Admin voting on proposal: {}, vote: {}", proposal_id, vote);
    }

    fn is_owner(&self, caller: &String) -> bool {
        &self.owner == caller
    }
}