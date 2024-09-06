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
                // Additional logic for AddLiquidity
            },
            InstructionType::RemoveLiquidity => {
                // Additional logic for RemoveLiquidity
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