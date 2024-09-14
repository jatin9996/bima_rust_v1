use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::{AuthorityMessage, AssignAuthorityParams};
use arch_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AuthorityManager;

impl AuthorityManager {
    pub fn assign_authority(&self, utxo: Utxo, new_authority: Pubkey) {
        // Logic to assign a new authority to a UTXO
    }

    pub fn verify_authority(&self, utxo: Utxo, authority: Pubkey) -> bool {
        // Logic to verify the authority of a UTXO
    }
}
