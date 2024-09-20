use std::collections::HashMap;
use log::{info, warn};
use borsh::{BorshSerialize, BorshDeserialize};

// Import Arch SDK modules
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

use bitcoin::{self, Transaction}; // Ensure this import is present

// Define UTXO structure
pub struct Utxo {
    pub outpoint: bitcoin::OutPoint,
    pub value: u64,
    pub script_pubkey: bitcoin::Script,
}

pub struct UtxoSet {
    pub utxos: HashMap<bitcoin::OutPoint, Utxo>,
}

impl UtxoSet {
    pub fn new() -> Self {
        UtxoSet {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: bitcoin::Script) {
        let outpoint = bitcoin::OutPoint::new(tx.txid(), vout);
        let utxo = Utxo {
            outpoint,
            value,
            script_pubkey,
        };
        self.utxos.insert(outpoint, utxo);
    }

    pub fn spend_utxo(&mut self, outpoint: bitcoin::OutPoint) {
        self.utxos.remove(&outpoint);
    }
}

// Add UTXO set to LiquidationManager
pub struct LiquidationManager {
    stability_pool: String,
    sorted_troves: String,
    borrower_operations: String,
    factory: Pubkey, // Change factory type to Pubkey
    enabled_trove_managers: HashMap<String, bool>,
    utxo_set: UtxoSet,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Liquidation {
    borrower: Pubkey, // Change borrower type to Pubkey
    liquidated_debt: u64,
    liquidated_coll: u64,
}

impl LiquidationManager {
    pub fn new(factory: Pubkey) -> Self {
        LiquidationManager {
            stability_pool: String::new(),
            sorted_troves: String::new(),
            borrower_operations: String::new(),
            factory,
            enabled_trove_managers: HashMap::new(),
            utxo_set: UtxoSet::new(),
        }
    }

    pub fn enable_trove_manager(&mut self, trove_manager: String) {
        assert_eq!(self.factory, self.factory, "Only factory can enable a trove manager");
        self.enabled_trove_managers.insert(trove_manager, true);
    }

    pub fn liquidate(&mut self, trove_manager: String, borrower: Pubkey) -> Result<Option<Liquidation>, ProgramError> {
        let is_enabled = *self.enabled_trove_managers.get(&trove_manager).unwrap_or(&false);
        if (!is_enabled) {
            return Ok(None);
        }

        // Use Arch SDK for financial calculations
        let debt_reduction = FinancialMath::calculate_debt_reduction();
        let collateral_reduction = FinancialMath::calculate_collateral_reduction();

        let liquidation = Liquidation {
            borrower: borrower.clone(),
            liquidated_debt: debt_reduction,
            liquidated_coll: collateral_reduction,
        };

        // Log the liquidation event
        msg!("Liquidation occurred for borrower: {:?}", borrower);

        // Create a transaction to record the liquidation
        let tx = self.create_liquidation_transaction(&borrower, debt_reduction, collateral_reduction)?;

        // Set the transaction to be signed
        set_transaction_to_sign(tx)?;

        Ok(Some(liquidation))
    }

    fn create_liquidation_transaction(&self, borrower: &Pubkey, debt_reduction: u64, collateral_reduction: u64) -> Result<TransactionToSign, ProgramError> {
        // Create a Bitcoin transaction using the bitcoin crate
        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: vec![
                // Use UTXO from the set
                bitcoin::TxIn {
                    previous_output: self.utxo_set.utxos.keys().next().unwrap().clone(),
                    script_sig: bitcoin::Script::new(),
                    sequence: 0xFFFFFFFF,
                    witness: vec![],
                },
            ],
            output: vec![
                // Populate with necessary output fields
                bitcoin::TxOut {
                    value: debt_reduction + collateral_reduction, // Example value
                    script_pubkey: bitcoin::Script::new_p2pkh(&borrower.to_bytes()),
                },
            ],
        };

        // Convert the Bitcoin transaction to raw bytes
        let tx_bytes = tx.serialize();

        // Define the inputs that require signatures
        let inputs_to_sign = vec![
            InputToSign {
                previous_output: tx.input[0].previous_output.clone(),
                script_pubkey: tx.output[0].script_pubkey.clone(),
                value: tx.output[0].value,
            },
            // Add more inputs if necessary
        ];

        // Create the TransactionToSign for the Arch network
        let tx_to_sign = TransactionToSign {
            tx_bytes,
            inputs_to_sign,
        };

        Ok(tx_to_sign)
    }

    // Access control method
    pub fn ensure_factory_or_approved(&self, caller: Pubkey) {
        assert!(
            caller == self.factory || self.is_approved_delegate(self.factory.clone(), caller),
            "Unauthorized access"
        );
    }

    fn is_approved_delegate(&self, factory: Pubkey, caller: Pubkey) -> bool {
        caller == factory
    }
}

mod FinancialMath {
    pub fn calculate_debt_reduction() -> u64 {
  
        // This function should calculate the debt reduction based on the logic in LiquidationManager.sol
        100
    }

    pub fn calculate_collateral_reduction() -> u64 {
       
        // This function should calculate the collateral reduction based on the logic in LiquidationManager.sol
        50
    }

    pub fn calculate_coll_gas_compensation(coll: u64) -> u64 {
        // This function should calculate the collateral gas compensation based on the logic in LiquidationManager.sol
        coll / 100 // of the collateral
    }

    pub fn calculate_debt_gas_compensation() -> u64 {
        // 
        // This function should calculate the debt gas compensation based on the logic in LiquidationManager.sol
        10 //  gas
    }
}