#![cfg_attr(not(feature = "std"), no_std)]

use sha2::{Sha256, Digest};
use std::collections::HashMap;
use merkle_light::merkle::MerkleTree;
use merkle_light::hash::{Algorithm, Hashable};
use ring::digest::{Context, Digest, SHA256};
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
            self.transfer_tokens_from_vault(amount);
            // Simulate token locking
            self.lock_tokens(receiver, amount);
            self.claimed_bitmap.insert(index, true);
            // Simulate callback if receiver is different from claimant
            if claimant != receiver {
                self.claim_callback(&receiver, amount);
            }
            // Emit Claimed event
            Self::deposit_event(Event::Claimed(index, receiver.clone(), amount));
        } else {
            panic!("Invalid merkle proof");
        }
    }

    fn transfer_tokens_from_vault(&self, amount: u64) {
        // Simulate the transfer of tokens from the vault to this contract
        println!("Transferring {} tokens from vault to contract", amount);
    }

    fn lock_tokens(&self, receiver: String, amount: u64) {
        // Simulate the locking of tokens
        println!("Locking {} tokens for receiver {}", amount, receiver);
    }

    fn claim_callback(&self, receiver: &String, amount: u64) {
        // Simulate the callback if receiver is different from claimant
        println!("Executing claim callback for receiver {} with amount {}", receiver, amount);
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
        let node = Sha256::digest(&[index.to_le_bytes(), claimant.as_bytes(), &amount.to_le_bytes()].concat());
        let mut hash = node.to_vec();

        for proof in merkle_proof {
            let mut hasher = Sha256::new();
            if hash < proof {
                hasher.update(&[hash, proof].concat());
            } else {
                hasher.update(&[proof, hash].concat());
            }
            hash = hasher.finalize().to_vec();
        }

        match &self.merkle_root {
            Some(root) => &hash == root,
            None => false,
        }
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

