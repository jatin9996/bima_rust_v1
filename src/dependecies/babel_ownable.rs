// Standard Rust module for BabelOwnable without blockchain-specific features
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelOwnable {
    babel_core: String, 
    guardian_account: String, // Separate field for the guardian
}

impl BabelOwnable {
    // Constructor to create a new BabelOwnable
    pub fn new(babel_core: String, guardian_account: String) -> Self {
        BabelOwnable { babel_core, guardian_account }
    }

    // Method to get the owner
    pub fn owner(&self) -> &String {
        &self.babel_core
    }

    // Method to get the guardian
    pub fn guardian(&self) -> &String {
        &self.guardian_account
    }

    // Method to check if the caller is the owner
    pub fn only_owner(&self, caller: &String) -> bool {
        caller == &self.babel_core
    }
}

// Example usage
fn main() {
    let babel_ownable = BabelOwnable::new("owner_account_id".to_string(), "guardian_account_id".to_string());
    println!("Owner: {}", babel_ownable.owner());
    println!("Guardian: {}", babel_ownable.guardian());
    println!("Is owner: {}", babel_ownable.only_owner(&"owner_account_id".to_string()));

    // Serialize the struct
    let serialized_data = babel_ownable.try_to_vec().unwrap();
    println!("Serialized: {:?}", serialized_data);

    // Deserialize the struct
    let deserialized_babel_ownable: BabelOwnable = BabelOwnable::try_from_slice(&serialized_data).unwrap();
    println!("Deserialized Owner: {}", deserialized_babel_ownable.owner());
    println!("Deserialized Guardian: {}", deserialized_babel_ownable.guardian());
}

