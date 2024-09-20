use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use borsh::{BorshDeserialize, BorshSerialize};
use bitcoin::{self, Transaction, TxIn, TxOut}; // Import the bitcoin crate and Transaction struct
use archnetwork::{
    transaction_to_sign::TransactionToSign, // Import the TransactionToSign struct
    program::{
        get_account_script_pubkey, get_bitcoin_tx, get_network_xonly_pubkey, invoke,
        next_account_info, set_return_data, set_transaction_to_sign, validate_utxo_ownership,
    },
    helper::get_state_transition_tx, // Import get_state_transition_tx
    input_to_sign::InputToSign, // Import InputToSign
};
use arch_program::{
    pubkey::Pubkey, // Import Pubkey from Arch SDK
    program_error::ProgramError,
    msg, // Import msg for logging
    utxo::UtxoMeta, // Import UtxoMeta
};

pub struct TroveManager {
    troves: HashMap<Pubkey, Trove>, // Change AccountId to Pubkey
    total_stakes: Balance,
    total_active_collateral: Balance,
    total_active_debt: Balance,
    base_rate: Balance,
    last_fee_operation_time: u64,
    owner: Pubkey, // Change AccountId to Pubkey
    reward_integral: Balance,
    reward_rate: Balance,
    last_update: u64,
    period_finish: u64,
    reward_integral_for: HashMap<Pubkey, Balance>, // Change AccountId to Pubkey
    stored_pending_reward: HashMap<Pubkey, Balance>, // Change AccountId to Pubkey
    surplus_balances: HashMap<Pubkey, Balance>, // Change AccountId to Pubkey
    paused: bool,
    sunsetting: bool,
    utxos: HashMap<Pubkey, Vec<UtxoMeta>>, // Add UTXOs management
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Trove {
    debt: Balance,
    coll: Balance,
    stake: Balance,
    status: Status,
    array_index: u32,
    active_interest_index: Balance,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Status {
    NonExistent,
    Active,
    ClosedByOwner,
    ClosedByLiquidation,
    ClosedByRedemption,
}

impl TroveManager {
    pub fn new(owner: Pubkey) -> Self { // Change AccountId to Pubkey
        Self {
            troves: HashMap::new(),
            total_stakes: 0,
            total_active_collateral: 0,
            total_active_debt: 0,
            base_rate: 0,
            last_fee_operation_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            owner,
            reward_integral: 0,
            reward_rate: 0,
            last_update: 0,
            period_finish: 0,
            reward_integral_for: HashMap::new(),
            stored_pending_reward: HashMap::new(),
            surplus_balances: HashMap::new(),
            paused: false,
            sunsetting: false,
            utxos: HashMap::new(), // Initialize UTXOs
        }
    }

    pub fn set_paused(&mut self, paused: bool) -> Result<(), ProgramError> {
        let caller = self.get_caller();
        if self.owner == caller {
            self.paused = paused;
            Ok(())
        } else {
            msg!("Unauthorized: caller is not the owner");
            Err(ProgramError::Custom(1)) // Custom error code for unauthorized access
        }
    }

    pub fn adjust_base_rate(&mut self, adjustment: Balance) -> Result<(), String> {
        let caller = self.get_caller();
        if self.owner == caller {
            self.base_rate = self.base_rate.saturating_add(adjustment);

            // Create a state transition transaction
            let accounts = vec![]; // Populate with actual accounts
            let mut tx = get_state_transition_tx(&accounts);
            msg!("State transition transaction created: {:?}", tx);

            Ok(())
        } else {
            Err("Unauthorized: caller is not the owner".to_string())
        }
    }

    pub fn add_collateral(&mut self, borrower: Pubkey, amount: Balance) { // Change AccountId to Pubkey
        let trove = self.troves.get_mut(&borrower).unwrap();
        trove.coll += amount;
        self.total_active_collateral += amount;
    }

    pub fn claim_collateral(&mut self, receiver: Pubkey) -> Result<Balance, String> { // Change AccountId to Pubkey
        let claimable_coll = self.surplus_balances.get(&receiver).cloned().unwrap_or(0);
        if claimable_coll > 0 {
            self.surplus_balances.insert(receiver, 0);
            Ok(claimable_coll)
        } else {
            Err("No collateral available to claim".to_string())
        }
    }

    pub fn claim_reward(&mut self, account: Pubkey) -> Result<Balance, String> { // Change AccountId to Pubkey
        let amount = self.apply_pending_rewards(account)?;
        if amount > 0 {
            self.stored_pending_reward.insert(account, 0);
            Ok(amount)
        } else {
            Err("No rewards available to claim".to_string())
        }
    }

    pub fn apply_pending_rewards(&mut self, account: Pubkey) -> Result<Balance, String> { // Change AccountId to Pubkey
        let reward_integral = self.reward_integral;
        let reward_integral_for = self.reward_integral_for.get(&account).cloned().unwrap_or(0);
        if reward_integral > reward_integral_for {
            let pending_reward = self.stored_pending_reward.get(&account).cloned().unwrap_or(0);
            let new_reward = pending_reward + (self.troves.get(&account).unwrap().stake * (reward_integral - reward_integral_for)) / 1e18 as Balance;
            self.stored_pending_reward.insert(account, new_reward);
            self.reward_integral_for.insert(account, reward_integral);
            Ok(new_reward)
        } else {
            Ok(0)
        }
    }

    fn get_caller(&self) -> Pubkey { // Change AccountId to Pubkey
        self.owner 
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization should not fail")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization should not fail")
    }

    pub fn process_bitcoin_transaction(&self, tx: TransactionToSign) -> Result<(), String> {
        // Example function to process a Bitcoin transaction
        let script_pubkey = get_account_script_pubkey(&self.owner);
        msg!("script_pubkey {:?}", script_pubkey);

        // Validate UTXO ownership
        let input = &tx.inputs_to_sign[0];
        if !self.validate_utxo(input.signer, input.txid, input.vout) {
            return Err("Invalid UTXO ownership".to_string());
        }

        // Invoke another program if needed
        invoke(&tx)?;

        Ok(())
    }

    pub fn create_bitcoin_transaction(&self, inputs: Vec<TxIn>, outputs: Vec<TxOut>) -> TransactionToSign {
        // Example function to create a Bitcoin transaction
        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: inputs.clone(),
            output: outputs.clone(),
        };
        let tx_bytes = tx.serialize(); // Serialize the transaction to bytes
        let tx_to_sign = TransactionToSign {
            tx_bytes,
            inputs_to_sign: inputs.iter().enumerate().map(|(index, input)| InputToSign {
                index: index as u32,
                signer: self.owner.clone(),
                txid: input.previous_output.txid,
                vout: input.previous_output.vout,
            }).collect(),
        };

        // Set the transaction to sign
        set_transaction_to_sign(&[self.owner.clone()], tx_to_sign.clone());

        tx_to_sign
    }

    pub fn add_utxo(&mut self, owner: Pubkey, utxo: UtxoMeta) {
        self.utxos.entry(owner).or_insert_with(Vec::new).push(utxo);
    }

    pub fn spend_utxo(&mut self, owner: Pubkey, txid: [u8; 32], vout: u32) -> Result<(), String> {
        if let Some(utxos) = self.utxos.get_mut(&owner) {
            if let Some(pos) = utxos.iter().position(|u| u.txid == txid && u.vout == vout) {
                utxos.remove(pos);
                Ok(())
            } else {
                Err("UTXO not found".to_string())
            }
        } else {
            Err("Owner has no UTXOs".to_string())
        }
    }

    pub fn validate_utxo(&self, owner: Pubkey, txid: [u8; 32], vout: u32) -> bool {
        if let Some(utxos) = self.utxos.get(&owner) {
            utxos.iter().any(|u| u.txid == txid && u.vout == vout)
        } else {
            false
        }
    }
}

type Balance = u64; // type definition
type Timestamp = u64; // type definition