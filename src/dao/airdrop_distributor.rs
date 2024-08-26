#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod airdrop_distributor {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;

    #[ink(storage)]
    pub struct AirdropDistributor {
        owner: AccountId,
        merkle_root: Option<String>,
        can_claim_until: Option<u64>,
        claimed_bitmap: StorageHashMap<u32, bool>,
        token_locker: AccountId,
        vault: AccountId,
    }

    impl AirdropDistributor {
        #[ink(constructor)]
        pub fn new(owner: AccountId, token_locker: AccountId, vault: AccountId) -> Self {
            Self {
                owner,
                merkle_root: None,
                can_claim_until: None,
                claimed_bitmap: StorageHashMap::new(),
                token_locker,
                vault,
            }
        }

        #[ink(message)]
        pub fn set_merkle_root(&mut self, merkle_root: String) {
            assert!(self.merkle_root.is_none(), "Merkle root already set");
            self.merkle_root = Some(merkle_root);
            self.can_claim_until = Some(Self::current_timestamp() + 7889231); // Simulate CLAIM_DURATION
        }

        #[ink(message)]
        pub fn claim(&mut self, index: u32, claimant: AccountId, amount: Balance, merkle_proof: Vec<String>) {
            assert!(self.is_claim_period_active() && !self.is_claimed(index), "Claim period has ended or already claimed");
            // Simulate merkle proof verification
            // Assuming verification is successful, transfer tokens and lock them
            let result = self.vault.transfer_tokens("BABEL".to_string(), claimant.clone(), amount);
            if result.is_ok() {
                self.token_locker.lock(claimant, amount, 52); // Lock tokens for a year
                ink_env::debug_println!("Claimed {} tokens for {:?}", amount, claimant);
                self.claimed_bitmap.insert(index, true);
            }
        }

        #[ink(message)]
        pub fn is_claimed(&self, index: u32) -> bool {
            *self.claimed_bitmap.get(&index).unwrap_or(&false)
        }

        #[ink(message)]
        pub fn is_claim_period_active(&self) -> bool {
            match self.can_claim_until {
                Some(t) => t > Self::current_timestamp(),
                None => false,
            }
        }

        fn current_timestamp() -> u64 {
            ink_env::block_timestamp()
        }
    }
}