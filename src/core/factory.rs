use std::collections::HashMap;
use borsh::{BorshSerialize, BorshDeserialize};
use arch_program::{
    account::AccountInfo,
    entrypoint,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    instruction::Instruction,
    msg,
    program::{get_account_script_pubkey, get_bitcoin_tx, get_network_xonly_pubkey, invoke, next_account_info, set_return_data, set_transaction_to_sign, validate_utxo_ownership},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
};
use bitcoin::{self, Transaction};

struct BabelOwnable {
    owner: Pubkey,
}

impl BabelOwnable {
    fn new(owner: Pubkey) -> Self {
        Self { owner }
    }

    fn is_owner(&self, caller: &Pubkey) -> bool {
        self.owner == *caller
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Factory {
    babel_ownable: BabelOwnable,
    trove_manager_impl: TroveManager,
    sorted_troves_impl: SortedTroves,
    trove_managers: HashMap<String, TroveManager>,
}

impl Factory {
    pub fn new(owner: Pubkey) -> Self {
        Self {
            babel_ownable: BabelOwnable::new(owner),
            trove_manager_impl: TroveManager::new("default_address".to_string()),
            sorted_troves_impl: SortedTroves::new(),
            trove_managers: HashMap::new(),
        }
    }

    pub fn deploy_new_instance(
        &mut self,
        caller: &Pubkey,
        collateral: String,
        price_feed: String,
        params: DeploymentParams,
        accounts: &[AccountInfo],
    ) -> Result<(), ProgramError> {
        if !self.babel_ownable.is_owner(caller) {
            return Err(ProgramError::Unauthorized);
        }

        let trove_manager_impl = self.trove_manager_impl.clone();
        let sorted_troves_impl = self.sorted_troves_impl.clone();

        // Initialize the cloned instances
        trove_manager_impl.set_addresses(&price_feed, &sorted_troves_impl, &collateral);
        sorted_troves_impl.set_addresses(&trove_manager_impl);

        // Verify that the oracle is correctly working
        trove_manager_impl.fetch_price();

        // Enable collateral and configure the new trove manager
        self.stability_pool.enable_collateral(&collateral);
        self.liquidation_manager.enable_trove_manager(&trove_manager_impl);
        self.debt_token.enable_trove_manager(&trove_manager_impl);
        self.borrower_operations.configure_collateral(&trove_manager_impl, &collateral);

        // Set parameters on the new trove manager
        trove_manager_impl.set_parameters(params);

        let id = format!("tm_{}", self.trove_managers.len() + 1);
        self.trove_managers.insert(id, trove_manager_impl);

        // Use Arch SDK to validate UTXO ownership
        let utxo_meta = UtxoMeta::new();
        validate_utxo_ownership(&utxo_meta, accounts)?;

        // Create a new Bitcoin transaction
        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: vec![],  // Add inputs as needed
            output: vec![], // Add outputs as needed
        };

        // Use Arch SDK to set transaction to sign
        let tx_bytes = tx.serialize();
        let inputs_to_sign = vec![InputToSign {
            index: 0,
            signer: accounts[0].key.clone(),
        }];
        let tx_to_sign = TransactionToSign::new(tx_bytes, inputs_to_sign);
        set_transaction_to_sign(&tx_to_sign)?;

        // Use Arch SDK to get state transition transaction
        let mut state_tx = get_state_transition_tx(accounts);
        state_tx.input.push(tx.input[0].clone());

        msg!("State transition transaction: {:?}", state_tx);

        Ok(())
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization failed")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization failed")
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
struct TroveManager {
    troves: HashMap<String, Trove>,
    address: String,
}

impl TroveManager {
    pub fn new(address: String) -> Self {
        Self {
            troves: HashMap::new(),
            address,
        }
    }

    pub fn set_addresses(&mut self, price_feed: &str, sorted_troves: &SortedTroves, collateral: &str) {
        self.price_feed = price_feed.to_string();
        self.sorted_troves = sorted_troves.clone();
        self.collateral = collateral.to_string();
    }

    pub fn fetch_price(&self) {
        // Implementation to fetch price from the oracle
    }

    pub fn set_parameters(&mut self, params: DeploymentParams) {
        self.params = params;
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
struct SortedTroves {
    troves: HashMap<String, Trove>,
}

impl SortedTroves {
    pub fn new() -> Self {
        Self {
            troves: HashMap::new(),
        }
    }

    pub fn set_addresses(&mut self, trove_manager: &TroveManager) {
        // Link back to the TroveManager
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct DeploymentParams {
    // Parameters as needed
}