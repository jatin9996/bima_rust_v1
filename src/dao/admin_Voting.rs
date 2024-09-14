use borsh::{BorshSerialize, BorshDeserialize};
use sdk::{Pubkey, UtxoMeta};
use crate::model::types::{Proposal, Action};
use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AdminVoting {
    pub proposal_data: HashMap<u32, Proposal>,
    pub proposal_payloads: HashMap<u32, Vec<Action>>,
    pub account_vote_weights: HashMap<(Pubkey, u32), u64>,
    pub latest_proposal_timestamp: HashMap<Pubkey, u64>,
    pub min_create_proposal_pct: u32,
    pub passing_pct: u32,
    pub system_start: u64,
}

impl AdminVoting {
    pub fn create_new_proposal(&mut self, account: Pubkey, payload: Vec<Action>, utxos: &[UtxoMeta]) {
        // Implementation adapted for Arch Network
    }

    // Additional methods adapted for UTXO and ZKVM
}
