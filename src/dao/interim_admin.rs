use borsh::{BorshSerialize, BorshDeserialize};
use sdk::arch_program::{pubkey::Pubkey, utxo::UtxoMeta};
use crate::model::{Proposal, Action};
use crate::utils::helpers;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InterimAdmin {
    pub babel_core: Pubkey,
}

impl InterimAdmin {
    pub fn create_new_proposal(&self, payload: Vec<Action>, current_utxos: &[UtxoMeta]) -> Vec<UtxoMeta> {
        let current_time = helpers::current_timestamp();
        let proposal = Proposal {
            created_at: current_time,
            can_execute_after: current_time + 86400, // 1 day
            processed: false,
        };

        // Serialize proposal and actions into new UTXOs
        let proposal_utxo = UtxoMeta {
            data: borsh::to_vec(&proposal).unwrap(),
            // Additional UTXO fields here
        };

        let action_utxos = payload.into_iter().map(|action| UtxoMeta {
            data: borsh::to_vec(&action).unwrap(),
            // Additional UTXO fields here
        }).collect::<Vec<_>>();

        // Return new UTXOs to be added to the blockchain
        let mut new_utxos = vec![proposal_utxo];
        new_utxos.extend(action_utxos);
        new_utxos
    }

    // Additional methods like `execute_proposal`, `cancel_proposal`, etc.
}
