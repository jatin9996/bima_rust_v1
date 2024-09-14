use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::combined_trove_data::CombinedTroveData;
use crate::helpers::utxo_utils::{read_utxo, write_utxo};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TroveManager {
    // UTXO-based storage fields
}

impl TroveManager {
    pub fn get_trove_data(&self, owner: Pubkey) -> Option<CombinedTroveData> {
        read_utxo(owner)
    }

    pub fn update_trove_data(&self, data: CombinedTroveData) -> Result<(), String> {
        write_utxo(data.owner, data)
    }
}
