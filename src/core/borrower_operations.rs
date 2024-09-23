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
    program_error::ProgramError
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
    critical_collateral_ratio: u128, // Added critical_collateral_ratio field
}

impl BorrowerOperationsState {
    pub fn new(
        min_net_debt: u64,
        trove_manager: TroveManager,
        debt_token: DebtToken,
        babel_base: BabelBase,
        babel_ownable: BabelOwnable,
        delegated_ops: DelegatedOps,
        critical_collateral_ratio: u128, // Added critical_collateral_ratio parameter
    ) -> Self {
        Self {
            min_net_debt,
            trove_manager,
            debt_token,
            babel_base,
            babel_ownable,
            delegated_ops,
            trove_managers_data: HashMap::new(),
            critical_collateral_ratio,
        }
    }

    pub fn adjust_trove(&mut self, user_id: AccountId, coll_change: i64, debt_change: i64) {
        self.ensure_owner_or_delegate(&user_id.to_string()); // Enhanced check
        if self.check_recovery_mode() {
            assert!(coll_change >= 0, "No collateral withdrawal allowed in recovery mode");
        }

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

        self.log_event("Trove adjusted"); // Log event
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

      set_return_data(&self.serialize());

        // Add UTXO management
        let utxo_set = UtxoSet::new();
        utxo_set.add_utxo(&tx, 0, collateral_amount as u64, account_info.key.to_string());

    }

    pub fn issue_debt(&mut self, amount: u128, max_fee_percentage: u128) {
        println!("Issuing debt: {}", amount);
        let fee = self.decay_base_rate_and_get_borrowing_fee(amount);
        self.ensure_user_accepts_fee(fee, amount, max_fee_percentage);
        self.debt_token.issue(amount + fee); // Issue debt amount including the fee
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
            self.babel_ownable.only_owner(caller) ||
            self.delegated_ops.is_approved_delegate(self.babel_ownable.owner(), caller),
            "Unauthorized: caller is neither owner nor delegate"
        );
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization failed")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization failed")
    }

    // New method to decay the base rate and calculate the borrowing fee
    pub fn decay_base_rate_and_get_borrowing_fee(&mut self, debt_amount: u128) -> u128 {
        // Example decay logic (simplified)
        self.base_rate *= 0.99; // Decay the base rate by 1%
        let fee = (self.base_rate as u128) * debt_amount / 10000; // Calculate fee based on the base rate
        fee
    }

    // New method to ensure the user accepts the fee
    pub fn ensure_user_accepts_fee(&self, fee: u128, debt_amount: u128, max_fee_percentage: u128) {
        let max_fee = debt_amount * max_fee_percentage / 100;
        assert!(fee <= max_fee, "Fee exceeds the maximum fee percentage allowed");
    }

    // New method to check if the new ICR is above the Minimum Collateral Ratio (MCR)
    pub fn require_icr_is_above_mcr(&self, new_icr: u128, mcr: u128) {
        assert!(new_icr >= mcr, "ICR must be above the minimum collateral ratio");
    }

    // New method to check if the new ICR is above the Critical Collateral Ratio (CCR)
    pub fn require_icr_is_above_ccr(&self, new_icr: u128) {
        assert!(new_icr >= self.critical_collateral_ratio, "ICR must be above the critical collateral ratio");
    }

    // New method to check if the new ICR is above the old ICR in Recovery Mode
    pub fn require_new_icr_is_above_old_icr(&self, new_icr: u128, old_icr: u128) {
        assert!(new_icr >= old_icr, "New ICR must be above the old ICR in Recovery Mode");
    }

    // New method to check if the new Total Collateral Ratio (TCR) is above the CCR
    pub fn require_new_tcr_is_above_ccr(&self, new_tcr: u128) {
        assert!(new_tcr >= self.critical_collateral_ratio, "New TCR must be above the critical collateral ratio");
    }

    // New method to validate adjustments based on the current mode (Normal or Recovery)
    pub fn require_valid_adjustment_in_current_mode(
        &self,
        total_priced_collateral: u128,
        total_debt: u128,
        is_recovery_mode: bool,
        coll_withdrawal: u128,
        is_debt_increase: bool,
        new_icr: u128,
        old_icr: u128,
        mcr: u128,
        new_tcr: u128
    ) {
        if is_recovery_mode {
            assert!(coll_withdrawal == 0, "Collateral withdrawal not permitted in Recovery Mode");
            if is_debt_increase {
                self.require_icr_is_above_ccr(new_icr);
                self.require_new_icr_is_above_old_icr(new_icr, old_icr);
            }
        } else {
            // Normal mode validations
            self.require_icr_is_above_mcr(new_icr, mcr);
            self.require_new_tcr_is_above_ccr(new_tcr);
        }
    }

    // Fetch balances from all TroveManagers
    pub fn fetch_balances(&self) -> SystemBalances {
        let mut collaterals = Vec::new();
        let mut debts = Vec::new();
        let mut prices = Vec::new();

        for (key, tm_data) in &self.trove_managers_data {
            let trove_manager = tm_data; // Assuming you have a way to access TroveManager data
            let (collateral, debt, price) = trove_manager.get_entire_system_balances();
            collaterals.push(collateral);
            debts.push(debt);
            prices.push(price);
        }

        SystemBalances {
            collaterals,
            debts,
            prices,
        }
    }

    // Calculate Total Collateral Ratio (TCR)
    pub fn get_tcr_data(&self, balances: &SystemBalances) -> (u128, u128, u128) {
        let mut total_priced_collateral = 0;
        let mut total_debt = 0;

        for i in 0..balances.collaterals.len() {
            total_priced_collateral += balances.collaterals[i] * balances.prices[i];
            total_debt += balances.debts[i];
        }

        let tcr = if total_debt == 0 { u128::MAX } else { total_priced_collateral / total_debt };

        (tcr, total_priced_collateral, total_debt)
    }

    // Get global system balances
    pub fn get_global_system_balances(&self) -> (u128, u128) {
        let balances = self.fetch_balances();
        let (_, total_priced_collateral, total_debt) = self.get_tcr_data(&balances);
        (total_priced_collateral, total_debt)
    }

    // Add a method to check for recovery mode based on TCR
    pub fn check_recovery_mode(&self) -> bool {
        let (tcr, _, _) = self.get_tcr_data(&self.fetch_balances());
        tcr < self.critical_collateral_ratio
    }

    // Implement event logging (assuming a simple logging mechanism)
    pub fn log_event(&self, event: &str) {
        msg!(event); // Using the `msg!` macro to log events
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

// Define SystemBalances structure
pub struct SystemBalances {
    collaterals: Vec<u128>,
    debts: Vec<u128>,
    prices: Vec<u128>,
}