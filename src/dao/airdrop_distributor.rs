#![cfg_attr(not(feature = "std"), no_std)]

use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Additional imports for event logging and error handling
use sp_std::prelude::*;
use frame_support::{decl_module, decl_event, decl_error};

// Import Borsh traits
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct AirdropDistributor {
    owner: String,
    merkle_root: Option<Vec<u8>>,
    can_claim_until: Option<u64>,
    claimed_bitmap: HashMap<u32, bool>,
    token_locker: String,
    vault: String,
}

impl AirdropDistributor {
    pub fn new(owner: String, token_locker: String, vault: String) -> Self {
        Self {
            owner,
            merkle_root: None,
            can_claim_until: None,
            claimed_bitmap: HashMap::new(),
            token_locker,
            vault,
        }
    }

    pub fn set_merkle_root(&mut self, merkle_root: Vec<u8>) {
        assert!(self.merkle_root.is_none(), "Merkle root already set");
        self.merkle_root = Some(merkle_root);
        self.can_claim_until = Some(Self::current_timestamp() + 7889231); // Simulate CLAIM_DURATION
        // Emit event here
    }

    // Updated claim method with token transfer, locking, and callback
    pub fn claim(&mut self, index: u32, claimant: String, receiver: String, amount: u64, merkle_proof: Vec<Vec<u8>>) {
        assert!(self.is_claim_period_active() && !self.is_claimed(index), "Claim period has ended or already claimed");
        if self.verify_merkle_proof(index, &claimant, amount, &merkle_proof) {
            // Simulate token transfer from vault to this contract
            // Simulate token locking
            self.claimed_bitmap.insert(index, true);
            // Simulate callback if receiver is different from claimant
            // Emit Claimed event
        } else {
            panic!("Invalid merkle proof");
        }
    }

    pub fn is_claimed(&self, index: u32) -> bool {
        *self.claimed_bitmap.get(&index).unwrap_or(&false)
    }

    pub fn is_claim_period_active(&self) -> bool {
        match self.can_claim_until {
            Some(t) => t > Self::current_timestamp(),
            None => false,
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    // Implement Merkle proof verification
    fn verify_merkle_proof(&self, index: u32, claimant: &String, amount: u64, merkle_proof: &Vec<Vec<u8>>) -> bool {
        // Actual implementation needed here
        true // Placeholder
    }
}

// Event declarations to mirror Solidity events
decl_event! {
    pub enum Event {
        MerkleRootSet(Vec<u8>, u64),
        Claimed(u32, String, u64),
    }
}

// Error handling
decl_error! {
    pub enum Error {
        MerkleRootAlreadySet,
        InvalidMerkleProof,
        ClaimPeriodEnded,
        AlreadyClaimed,
    }
}

