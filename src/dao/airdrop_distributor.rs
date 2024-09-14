use borsh::{BorshSerialize, BorshDeserialize};
use crate::interface::utxo_interface::UtxoInterface;
use crate::helper::utxo_helper::UtxoHelper;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AirdropDistributor {
    owner: Pubkey,
    merkle_root: Option<String>,
    can_claim_until: Option<u64>,
    claimed_bitmap: Vec<bool>,
}

impl AirdropDistributor {
    pub fn new(owner: Pubkey) -> Self {
        Self {
            owner,
            merkle_root: None,
            can_claim_until: None,
            claimed_bitmap: vec![],
        }
    }

    pub fn set_merkle_root(&mut self, merkle_root: String) {
        self.merkle_root = Some(merkle_root);
        self.can_claim_until = Some(UtxoHelper::current_timestamp() + 7889231);
    }

    pub fn claim(&mut self, index: u32, claimant: Pubkey, amount: u64, merkle_proof: Vec<String>) {
        assert!(self.can_claim(claimant, index), "Claim period has ended or already claimed");
        UtxoInterface::transfer_tokens(self.owner, claimant, amount);
        self.claimed_bitmap[index as usize] = true;
    }

    fn can_claim(&self, claimant: Pubkey, index: u32) -> bool {
        UtxoHelper::is_claim_period_active(self.can_claim_until) && !self.claimed_bitmap[index as usize]
    }
}
