use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use arch_program::{
    account::AccountInfo,
    entrypoint,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    instruction::Instruction,
    msg,
    program::{get_account_script_pubkey, get_bitcoin_tx, get_network_xonly_pubkey, invoke, next_account_info, set_return_data, set_transaction_to_sign, validate_utxo_ownership},
    program_error::ProgramError,
    pubkey::Pubkey, // Ensure Pubkey is imported
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
};
use bitcoin::{self, Transaction}; // Importing bitcoin crate and Transaction struct

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BorrowerOperationsState {
    min_net_debt: u64,
    trove_manager: TroveManager,
    debt_token: DebtToken,
    babel_base: BabelBase,
    babel_ownable: BabelOwnable,
    delegated_ops: DelegatedOps,
    trove_managers_data: HashMap<String, TroveManagerData>,
}

impl BorrowerOperationsState {
    pub fn new(
        min_net_debt: u64,
        trove_manager: TroveManager,
        debt_token: DebtToken,
        babel_base: BabelBase,
        babel_ownable: BabelOwnable,
        delegated_ops: DelegatedOps,
    ) -> Self {
        Self {
            min_net_debt,
            trove_manager,
            debt_token,
            babel_base,
            babel_ownable,
            delegated_ops,
            trove_managers_data: HashMap::new(),
        }
    }

    pub fn adjust_trove(&mut self, user_id: AccountId, coll_change: i64, debt_change: i64) {
        self.babel_ownable.only_owner();
        self.delegated_ops.ensure_caller_or_delegated(user_id);

        self.trove_manager.adjust_trove(user_id, coll_change, debt_change);
        if debt_change > 0 {
            self.debt_token.issue(debt_change as u64);
        } else {
            self.debt_token.burn((-debt_change) as u64);
        }

        // Example of using Arch SDK functionality
        let account_info = AccountInfo::new();
        let tx = get_state_transition_tx();
        let input_to_sign = InputToSign::new();
        let instruction = Instruction::new();
        msg!("Arch SDK functionality used in adjust_trove");

        // Construct TransactionToSign
        let tx_bytes = tx.serialize();
        let inputs_to_sign = vec![input_to_sign];
        let transaction_to_sign = TransactionToSign::new(tx_bytes, inputs_to_sign);

        // Set transaction to sign
        set_transaction_to_sign(transaction_to_sign);

        // Validate UTXO ownership
        validate_utxo_ownership(&account_info, &self.trove_manager.get_collateral_token());

        // Set return data
        set_return_data(&self.serialize());

        // Add UTXO management
        let utxo_set = UtxoSet::new();
        utxo_set.add_utxo(&tx, 0, coll_change as u64, account_info.key.to_string());
    }

    pub fn open_trove(&mut self, trove_manager: String, account: String, collateral_amount: u128, debt_amount: u128) {
        let tm_data = self.trove_managers_data.get(&trove_manager).expect("Invalid Trove Manager");

        println!(
            "Opening trove for account: {} with collateral: {} and debt: {}",
            account, collateral_amount, debt_amount
        );

        // Update internal state
        self.debt_token.issue(debt_amount); // Assuming a method to handle debt token issuance

        // Example of using Arch SDK functionality
        let account_info = AccountInfo::new();
        let tx = get_state_transition_tx();
        let input_to_sign = InputToSign::new();
        let instruction = Instruction::new();
        msg!("Arch SDK functionality used in open_trove");

        // Construct TransactionToSign
        let tx_bytes = tx.serialize();
        let inputs_to_sign = vec![input_to_sign];
        let transaction_to_sign = TransactionToSign::new(tx_bytes, inputs_to_sign);

        // Set transaction to sign
        set_transaction_to_sign(transaction_to_sign);

        // Validate UTXO ownership
        validate_utxo_ownership(&account_info, &tm_data.collateral_token);

        // Set return data
        set_return_data(&self.serialize());

        // Add UTXO management
        let utxo_set = UtxoSet::new();
        utxo_set.add_utxo(&tx, 0, collateral_amount as u64, account_info.key.to_string());
    }

    pub fn issue_debt(&mut self, amount: u128) {
        println!("Issuing debt: {}", amount);
        self.debt_token.issue(amount);
    }

    pub fn burn_debt(&mut self, amount: u128) {
        println!("Burning debt: {}", amount);
        self.debt_token.burn(amount);
    }

    pub fn calculate_icr(&self, collateral: u128, debt: u128, price: u128) -> u128 {
        if debt == 0 {
            return u128::MAX; // To handle division by zero
        }
        collateral * price / debt
    }

    pub fn ensure_owner_or_delegate(&self, caller: &str) {
        assert!(
            self.babel_ownable.only_owner(caller)
                || self.delegated_ops.is_approved_delegate(self.babel_ownable.owner(), caller),
            "Unauthorized"
        );
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization failed")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization failed")
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TroveManagerData {
    collateral_token: String,
    index: u16,
}

struct LocalVariablesOpenTrove {
    price: u128,
    total_priced_collateral: u128,
    total_debt: u128,
    net_debt: u128,
    composite_debt: u128,
    icr: u128,
    nicr: u128,
    stake: u128,
    array_index: u128,
}

// Define UTXO structure
pub struct UtxoSet {
    pub utxos: HashMap<OutPoint, UtxoMeta>,
}

impl UtxoSet {
    pub fn new() -> Self {
        UtxoSet {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: Script) {
        let outpoint = OutPoint::new(tx.txid(), vout);
        let utxo = UtxoMeta::new(outpoint, value, script_pubkey);
        self.utxos.insert(outpoint, utxo);
    }

    pub fn spend_utxo(&mut self, outpoint: OutPoint) {
        self.utxos.remove(&outpoint);
    }
}