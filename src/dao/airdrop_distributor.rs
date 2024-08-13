use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct AirdropDistributor {
    owner: String,
    merkle_root: Option<String>,
    can_claim_until: Option<u64>,
    claimed_bitmap: HashMap<u32, bool>,
}

impl AirdropDistributor {
    fn new(owner: String) -> Self {
        Self {
            owner,
            merkle_root: None,
            can_claim_until: None,
            claimed_bitmap: HashMap::new(),
        }
    }

    fn set_merkle_root(&mut self, merkle_root: String) {
        if self.merkle_root.is_some() {
            panic!("Merkle root already set");
        }
        self.merkle_root = Some(merkle_root);
        self.can_claim_until = Some(Self::current_timestamp() + 7889231); // Simulate CLAIM_DURATION
    }

    fn claim(&mut self, index: u32, claimant: &str, amount: f64, merkle_proof: Vec<String>) {
        if !self.is_claim_period_active() || self.is_claimed(index) {
            panic!("Claim period has ended or already claimed");
        }
        // Simulate merkle proof verification and token transfer logic
        println!("Claimed {} tokens for {}", amount, claimant);
        self.claimed_bitmap.insert(index, true);
    }

    fn is_claimed(&self, index: u32) -> bool {
        *self.claimed_bitmap.get(&index).unwrap_or(&false)
    }

    fn is_claim_period_active(&self) -> bool {
        match self.can_claim_until {
            Some(t) => t > Self::current_timestamp(),
            None => false,
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}