use crate::core::instructions::{ContractInstruction, InstructionType};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// Define the UtxoInfo struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UtxoInfo {
    txid: String,
    vout: u32,
    authority: Pubkey,
    data: Vec<u8>,
}

pub fn process_instructions(
    program_id: &Pubkey,
    utxos: &[UtxoInfo],
    instruction_data: &[u8],
) -> Result<(), u64> {
    let instruction: ContractInstruction = ContractInstruction::try_from_slice(instruction_data)
        .map_err(|_| 1)?;

    let mut new_authorities: HashMap<String, Pubkey> = HashMap::new();
    let mut new_data: HashMap<String, Vec<u8>> = HashMap::new();

    for utxo in utxos {
        let current_authority = &utxo.authority;
        let current_data = &utxo.data;

        // Process each UTXO based on the instruction
        match instruction.method {
            InstructionType::OpenPool => {
                
                new_authorities.insert(utxo.txid.clone(), *current_authority);
                new_data.insert(utxo.txid.clone(), current_data.clone());
            },
            InstructionType::AddLiquidity => {
                // Assuming the instruction data includes new authority and data for liquidity
                let new_authority = Pubkey::new_unique(); 
                let new_liquidity_data = vec![0u8; 10]; //  new data for liquidity

                new_authorities.insert(utxo.txid.clone(), new_authority);
                new_data.insert(utxo.txid.clone(), new_liquidity_data);
            },
            InstructionType::RemoveLiquidity => {
                //  logic for removing liquidity
                // This could involve simply removing the entries or updating them
                // Here, we'll assume we remove the entries for simplicity
                new_authorities.remove(&utxo.txid);
                new_data.remove(&utxo.txid);
            },
            // Handle other cases
        }
    }

    //  to the network (pseudo-code)
    //commit_changes(&new_authorities, &new_data);

    Ok(())
}

fn commit_changes(new_authorities: &HashMap<String, Pubkey>, new_data: &HashMap<String, Vec<u8>>) {
    // Logic to commit changes to the network
}