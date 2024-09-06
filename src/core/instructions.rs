use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum InstructionType {
    OpenPool,
    AddLiquidity,
    RemoveLiquidity,
    // Add other instruction types as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInstruction {
    pub method: InstructionType,
    pub data: Vec<u8>,
}
