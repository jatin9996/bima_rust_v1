use log::{info, warn};
use borsh::{BorshDeserialize, BorshSerialize};

// Define a struct to hold state similar to Solidity's contract state
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelCoreState {
    fee_receiver: String,
    guardian: String,
    owner: String,
    pending_owner: String,
    paused: bool,
    price_feed: String,
    ownership_transfer_deadline: u128,
    start_time: u128,
}

pub trait BabelCore {
    // Constants
    const OWNERSHIP_TRANSFER_DELAY: u128 = 86400;

    // Event Emitters with logging
    fn emit_fee_receiver_set(&self, fee_receiver: &str) {
        info!("FeeReceiverSet: {}", fee_receiver);
    }
    fn emit_guardian_set(&self, guardian: &str) {
        info!("GuardianSet: {}", guardian);
    }
   

    // Access control implementation
    fn is_owner(&self, caller: &str) -> bool {
        self.owner() == caller
    }

    // Function implementations with access control
    fn set_fee_receiver(&mut self, caller: &str, fee_receiver: &str) {
        if self.is_owner(caller) {
            self.emit_fee_receiver_set(fee_receiver);
            // logic to set fee receiver
        } else {
            warn!("Unauthorized attempt to set fee receiver by {}", caller);
        }
    }

    fn set_guardian(&mut self, caller: &str, guardian: &str) {
        if self.is_owner(caller) {
            self.emit_guardian_set(guardian);
            // logic to set guardian
        } else {
            warn!("Unauthorized attempt to set guardian by {}", caller);
        }
    }

    fn set_paused(&mut self, caller: &str, paused: bool) {
        if self.is_owner(caller) {
            // logic to set paused
            if paused {
                info!("Contract paused by {}", caller);
            } else {
                info!("Contract unpaused by {}", caller);
            }
        } else {
            warn!("Unauthorized attempt to pause/unpause by {}", caller);
        }
    }

    fn set_price_feed(&mut self, caller: &str, price_feed: &str) {
        if self.is_owner(caller) {
            // logic to set price feed
            info!("PriceFeedSet: {}", price_feed);
        } else {
            warn!("Unauthorized attempt to set price feed by {}", caller);
        }
    }
    

    // Getter functions
    fn ownership_transfer_delay(&self) -> u128 {
        Self::OWNERSHIP_TRANSFER_DELAY
    }

    fn fee_receiver(&self) -> &str {
        &self.fee_receiver
    }

    fn guardian(&self) -> &str {
        &self.guardian
    }

    fn owner(&self) -> &str {
        &self.owner
    }

    fn ownership_transfer_deadline(&self) -> u128 {
        self.ownership_transfer_deadline
    }

    fn paused(&self) -> bool {
        self.paused
    }

    fn pending_owner(&self) -> &str {
        &self.pending_owner
    }

    fn price_feed(&self) -> &str {
        &self.price_feed
    }

    fn start_time(&self) -> u128 {
        self.start_time
    }

    // Add more functions and state management as per Solidity contract
    fn accept_transfer_ownership(&mut self, caller: &str) {
        if caller == self.pending_owner {
            info!("Ownership transferred from {} to {}", self.owner, self.pending_owner);
            self.owner = self.pending_owner.clone();
            self.pending_owner = String::new(); // Clear pending owner
            self.ownership_transfer_deadline = 0; // Reset deadline
        } else {
            warn!("Unauthorized attempt to accept ownership by {}", caller);
        }
    }

    fn commit_transfer_ownership(&mut self, caller: &str, new_owner: &str) {
        if self.is_owner(caller) {
            self.pending_owner = new_owner.to_string();
            self.ownership_transfer_deadline = self.start_time + Self::OWNERSHIP_TRANSFER_DELAY;
            info!("Ownership transfer committed to {} with deadline {}", new_owner, self.ownership_transfer_deadline);
        } else {
            warn!("Unauthorized attempt to commit ownership transfer by {}", caller);
        }
    }

    fn revoke_transfer_ownership(&mut self, caller: &str) {
        if self.is_owner(caller) {
            info!("Ownership transfer to {} revoked by {}", self.pending_owner, caller);
            self.pending_owner = String::new(); // Clear pending owner
            self.ownership_transfer_deadline = 0; // Reset deadline
        } else {
            warn!("Unauthorized attempt to revoke ownership transfer by {}", caller);
        }
    }
   
    // Serialization and Deserialization methods
    fn serialize_state(&self) -> Vec<u8> {
        self.try_to_vec().expect("Failed to serialize state")
    }

    fn deserialize_state(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Failed to deserialize state")
    }
}


