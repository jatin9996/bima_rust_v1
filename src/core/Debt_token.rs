#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use secp256k1::{Secp256k1, Message, PublicKey, Signature};
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

    pubkey::Pubkey, // Ensure Pubkey is imported
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign, // Ensure this import is present
    utxo::UtxoMeta,
};
use bitcoin::{self, Transaction}; // Import bitcoin crate and Transaction struc
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
    bitcoin::{self, Transaction},
};

// Constants
const FLASH_LOAN_FEE: u128 = 9; // 0.09% fee, similar to Solidity's 0.09%

// Add a mapping to store authorized Trove Managers
use std::collections::HashSet;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DebtToken {
    name: String,
    symbol: String,
    total_supply: Balance,
    balances: HashMap<AccountId, Balance>,
    allowances: HashMap<(AccountId, AccountId), Balance>,
    collateral: HashMap<AccountId, Balance>,
    debt: HashMap<AccountId, Balance>,
    trove_managers: HashSet<AccountId>,
    factory: AccountId, // Account ID of the factory
    borrower_operations: AccountId, // Account ID of the BorrowerOperations contract
    stability_pool: AccountId, // Account ID of the StabilityPool contract
}

pub type AccountId = Pubkey; // Use Pubkey for AccountId
pub type Balance = u128;

impl DebtToken {
    // Add error types for unauthorized access and invalid operations
    pub enum DebtTokenError {
        UnauthorizedAccess,
        InvalidOperation,
        // Existing errors...
        InsufficientFunds,
        InvalidAccountData,
    }

    // Modify existing methods to use new error types
    pub fn mint(&mut self, account: AccountId, amount: Balance, account_info: &AccountInfo) -> Result<(), DebtTokenError> {
        // Check if the caller is authorized as a Trove Manager or Borrower Operations
        if !self.trove_managers.contains(&account_info.key) && account_info.key != self.borrower_operations {
            return Err(DebtTokenError::UnauthorizedAccess);
        }

        // Validate account ownership using Arch SDK
        if !self.validate_utxo_ownership(account_info)? {
            return Err(DebtTokenError::InvalidAccountData);
        }

        // Create a transaction to sign
        let mut tx = get_state_transition_tx(&[account_info.clone()])?;
        tx.input.push(get_bitcoin_tx(account_info)?.input[0].clone());

        let tx_to_sign = TransactionToSign {
            tx_bytes: bitcoin::consensus::serialize(&tx),
            inputs_to_sign: vec![InputToSign {
                index: 0,
                signer: account_info.key.clone(),
            }],
        };

        // Set the transaction to sign
        set_transaction_to_sign(tx_to_sign)?;

        // Mint the tokens
        let balance = self.balances.entry(account).or_default();
        *balance += amount;
        self.total_supply += amount;

        Ok(())
    }

    // Similar changes for other methods...

    pub fn new(name: String, symbol: String) -> Self {
        Self {
            name,
            symbol,
            total_supply: 0,
            balances: HashMap::new(),
            allowances: HashMap::new(),
            collateral: HashMap::new(),
            debt: HashMap::new(),
            trove_managers: HashSet::new(),
            factory: Pubkey::default(),
            borrower_operations: Pubkey::default(),
            stability_pool: Pubkey::default(),
        }
    }

    pub fn burn(&mut self, account: AccountId, amount: Balance, account_info: &AccountInfo) -> Result<(), ProgramError> {
        // Validate account ownership using Arch SDK
        if !self.validate_utxo_ownership(account_info)? {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if the caller is a TroveManager
        if !self.trove_managers.contains(&account_info.key) {
            return Err(ProgramError::InvalidInstructionData);
        }

        let balance = self.balances.entry(account).or_default();
        if *balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        *balance -= amount;
        self.total_supply -= amount;
        Ok(())
    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance, from_info: &AccountInfo, to_info: &AccountInfo) -> Result<(), ProgramError> {
        // Validate account ownership using Arch SDK
        if !self.validate_utxo_ownership(from_info)? || !self.validate_utxo_ownership(to_info)? {
            return Err(ProgramError::InvalidAccountData);
        }

        // Recipient validation (similar to Solidity's _requireValidRecipient)
        if to == self.get_account_id() || to == self.stability_pool || self.trove_managers.contains(&to) || to == self.borrower_operations {
            return Err(ProgramError::InvalidInstructionData);
        }

        let from_balance = self.balances.entry(from).or_default();
        if *from_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        *from_balance -= amount;

        let to_balance = self.balances.entry(to).or_insert(0);
        *to_balance += amount;
        Ok(())
    }

    pub fn issue_debt(&mut self, user: AccountId, amount: Balance, account_info: &AccountInfo) -> Result<(), ProgramError> {
        // Validate account ownership using Arch SDK
        if !self.validate_utxo_ownership(account_info)? {
            return Err(ProgramError::InvalidAccountData);
        }


        // Create a transaction to sign
        let mut tx = get_state_transition_tx(&[account_info.clone()])?;
        tx.input.push(get_bitcoin_tx(account_info)?.input[0].clone());

        let tx_to_sign = TransactionToSign {
            tx_bytes: bitcoin::consensus::serialize(&tx),
            inputs_to_sign: vec![InputToSign {
                index: 0,
                signer: account_info.key.clone(),
            }],
        };

        // Set the transaction to sign
        set_transaction_to_sign(tx_to_sign)?;


        let debt_balance = self.debt.entry(user.clone()).or_insert(0);
        let collateral_balance = *self.collateral.get(&user).unwrap_or(&0);

        if collateral_balance >= amount * 2 { // Ensure 200% collateralization
            *debt_balance += amount;
            self.total_supply += amount; // Mint debt tokens
            let user_balance = self.balances.entry(user).or_insert(0);
            *user_balance += amount;
            Ok(())
        } else {
            Err(ProgramError::InsufficientFunds)
        }
    }

    pub fn validate_utxo_ownership(&self, account_info: &AccountInfo) -> Result<bool, ProgramError> {
        // Use Arch SDK's validate_utxo_ownership function
        validate_utxo_ownership(&UtxoMeta::default(), account_info)
    }

    pub fn set_transaction(&self, tx: TransactionToSign) -> Result<(), ProgramError> {
        set_transaction_to_sign(tx)
    }

    pub fn get_state_transition(&self) -> Result<Transaction, ProgramError> {
        get_state_transition_tx()
    }

    pub fn invoke_external_program(&self, instruction: Instruction, account_infos: &[AccountInfo]) -> Result<(), ProgramError> {
        invoke(&instruction, account_infos)
    }

    // Function to calculate flash loan fee
    fn calculate_flash_loan_fee(&self, amount: Balance) -> Balance {
        (amount * FLASH_LOAN_FEE) / 10000
    }

    // Flash loan function
    pub fn flash_loan(&mut self, receiver: AccountId, amount: Balance, account_info: &AccountInfo, data: Vec<u8>) -> Result<(), ProgramError> {
        // Validate account ownership using Arch SDK
        if !self.validate_utxo_ownership(account_info)? {
            return Err(ProgramError::InvalidAccountData);
        }

        // Ensure the token amount requested does not exceed the maximum allowed
        if amount > self.max_flash_loan()? {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Calculate fee
        let fee = self.calculate_flash_loan_fee(amount);

        // Mint the amount to the receiver
        let receiver_balance = self.balances.entry(receiver).or_insert(0);
        *receiver_balance += amount;

        // Simulate the flash loan receiver logic (must be implemented by the receiver)
        // This is a placeholder for actual interaction with a receiver contract that handles the loan
        if !self.invoke_flash_loan_receiver(receiver, amount, fee, data, &[account_info.clone()])? {
            return Err(ProgramError::CustomError(0)); // Custom error for flash loan failure
        }

        // Burn the tokens returned by the receiver
        *receiver_balance -= amount + fee;

        // Transfer fee to a fee account or similar
        let fee_account_balance = self.balances.entry(self.fee_account).or_insert(0);
        *fee_account_balance += fee;

        Ok(())
    }

    // Helper function to check the maximum flash loan available
    fn max_flash_loan(&self) -> Result<Balance, ProgramError> {
        Ok(u128::MAX - self.total_supply)
    }

    // Placeholder for invoking the flash loan receiver logic
    fn invoke_flash_loan_receiver(&self, receiver: AccountId, amount: Balance, fee: Balance, data: Vec<u8>, account_infos: &[AccountInfo]) -> Result<bool, ProgramError> {
        // This should interact with an external contract or logic that handles the received amount and fee
        // For now, we simulate successful handling
        Ok(true)
    }

    pub fn add_collateral(&mut self, user: AccountId, amount: Balance) {
        let collateral_balance = self.collateral.entry(user).or_insert(0);
        *collateral_balance += amount;
    }

    pub fn verify_signature(&self, message: &[u8], sig: &[u8], pub_key: &[u8]) -> bool {
        let secp = Secp256k1::new();
        let message = Message::from_slice(message).expect("32 bytes");
        let sig = Signature::from_der(sig).expect("Signature in DER format");
        let pub_key = PublicKey::from_slice(pub_key).expect("Public key");

        secp.verify(&message, &sig, &pub_key).is_ok()
    }

    // Function to authorize Trove Managers by the factory
    pub fn enable_trove_manager(&mut self, trove_manager: AccountId) -> Result<(), DebtTokenError> {
        if get_caller() != self.factory {
            return Err(DebtTokenError::UnauthorizedAccess);
        }
        self.trove_managers.insert(trove_manager);
        Ok(())
    }

    // Helper function to get the account ID of the contract itself
    fn get_account_id(&self, program_id: &Pubkey) -> AccountId {
        // Assuming `program_id` is passed to the function which represents the ID of the current program
        *program_id
    }

    // Function to handle minting with gas compensation
    pub fn mint_with_gas_compensation(&mut self, account: AccountId, amount: Balance) -> Result<(), DebtTokenError> {
        if self.borrower_operations != get_caller() {
            return Err(DebtTokenError::UnauthorizedAccess);
        }
        self.mint(account, amount)?;
        self.mint(self.gas_pool, self.debt_gas_compensation)?;
        Ok(())
    }

    // Function to handle burning with gas compensation
    pub fn burn_with_gas_compensation(&mut self, account: AccountId, amount: Balance, account_info: &AccountInfo) -> Result<(), DebtTokenError> {
        if account_info.key != self.borrower_operations {
            return Err(DebtTokenError::UnauthorizedAccess);
        }
        self.burn(account, amount, account_info)?;
        self.burn(self.gas_pool, self.debt_gas_compensation, account_info)?;
        Ok(())
    }

    // Function to send debt tokens to the Stability Pool
    pub fn send_to_sp(&mut self, sender: AccountId, amount: Balance, sender_info: &AccountInfo) -> Result<(), DebtTokenError> {
        if sender_info.key != self.stability_pool {
            return Err(DebtTokenError::UnauthorizedAccess);
        }
        self.transfer(sender, self.stability_pool, amount, sender_info, &AccountInfo::from(self.stability_pool))?;
        Ok(())
    }

    // Function to return debt tokens from the pool
    pub fn return_from_pool(&mut self, pool: AccountId, receiver: AccountId, amount: Balance, pool_info: &AccountInfo) -> Result<(), DebtTokenError> {
        if pool_info.key != self.stability_pool && !self.trove_managers.contains(&pool_info.key) {
            return Err(DebtTokenError::UnauthorizedAccess);
        }
        self.transfer(pool, receiver, amount, pool_info, &AccountInfo::from(receiver))?;
        Ok(())
    }

    // Helper function to transfer tokens with checks
    fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance, from_info: &AccountInfo, to_info: &AccountInfo) -> Result<(), DebtTokenError> {
        // Validate account ownership using Arch SDK
        if !self.validate_utxo_ownership(from_info)? || !self.validate_utxo_ownership(to_info)? {
            return Err(DebtTokenError::InvalidAccountData);
        }

        // Recipient validation (similar to Solidity's _requireValidRecipient)
        if to == self.get_account_id() || to == self.stability_pool || self.trove_managers.contains(&to) || to == self.borrower_operations {
            return Err(DebtTokenError::InvalidOperation);
        }

        let from_balance = self.balances.entry(from).or_default();
        if *from_balance < amount {
            return Err(DebtTokenError::InsufficientFunds);
        }
        *from_balance -= amount;

        let to_balance = self.balances.entry(to).or_insert(0);
        *to_balance += amount;
        Ok(())
    }
}

impl DebtToken {
    // Cached domain separator
    cached_domain_separator: Option<(u64, [u8; 32])>,

    // Function to get or update the cached domain separator
    fn domain_separator(&mut self) -> [u8; 32] {
        let current_chain_id = get_chain_id();
        if let Some((chain_id, separator)) = self.cached_domain_separator {
            if chain_id == current_chain_id {
                return separator;
            }
        }
        let new_separator = self.calculate_domain_separator();
        self.cached_domain_separator = Some((current_chain_id, new_separator));
        new_separator
    }
}