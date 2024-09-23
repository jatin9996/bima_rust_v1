use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::dependencies::babel_ownable::BabelOwnable;
use crate::dependencies::system_start::SystemStart;
use crate::dependencies::babel_math::BabelMath;
use crate::interfaces::stability_pool::IStabilityPool;
use bitcoin::{self, Transaction, OutPoint, Script}; // Import OutPoint and Script
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
    utxo::UtxoMeta, // Import UtxoMeta
};

// Arch SDK imports
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
    bitcoin::{self, Transaction},
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StabilityPool {
    deposits: HashMap<AccountId, Balance>,
    total_stablecoins: Balance,
    owner: AccountId,
    collaterals: HashMap<CollateralId, CollateralData>,
    debt_token: DebtToken,
    reward_rate: Balance,
    last_update: u64,
    period_finish: u64,
    bitcoin_transactions: Vec<Transaction>,
    utxos: HashMap<OutPoint, UtxoMeta>, // Add UTXO management
    depositor_snapshots: HashMap<AccountId, DepositorSnapshot>,
    P: Balance, // Added to store the product factor
    current_scale: u64, // Added to store the current scale
    G: Balance, // Added to store the Babel gain sum
}

impl StabilityPool {
    pub fn new(owner_id: AccountId, debt_token: DebtToken) -> Self {
        StabilityPool {
            deposits: HashMap::new(),
            total_stablecoins: 0,
            owner: owner_id,
            collaterals: HashMap::new(),
            debt_token,
            reward_rate: 0,
            last_update: 0,
            period_finish: 0,
            bitcoin_transactions: Vec::new(),
            utxos: HashMap::new(), // Initialize UTXO management
            depositor_snapshots: HashMap::new(),
            P: 1, // Initialize P to 1
            current_scale: 0, // Initialize current_scale to 0
            G: 0, // Initialize G to 0
        }
    }

    pub fn deposit(&mut self, caller: AccountId, amount: Balance) {
        self.only_owner(&caller);
        let current_deposit = self.deposits.entry(caller).or_insert(0);
        *current_deposit += amount;
        self.total_stablecoins += amount;

        msg!("Deposit: caller = {}, amount = {}", caller, amount);

        let mut tx = get_state_transition_tx(&[]);
        tx.instructions.push(Instruction {
            program_id: Pubkey::default(),
            accounts: vec![],
            data: vec![],
        });

        let tx_to_sign = TransactionToSign {
            tx_bytes: &bitcoin::consensus::serialize(&tx),
            inputs_to_sign: &[InputToSign {
                index: 0,
                signer: Pubkey::default(),
            }],
        };

        msg!("Transaction to sign: {:?}", tx_to_sign);
        set_transaction_to_sign(&[], tx_to_sign);
    }

    pub fn withdraw(&mut self, caller: AccountId, amount: Balance) -> bool {
        self.only_owner(&caller);
        if let Some(current_deposit) = self.deposits.get_mut(&caller) {
            if *current_deposit >= amount {
                *current_deposit -= amount;
                self.total_stablecoins -= amount;

                msg!("Withdraw: caller = {}, amount = {}", caller, amount);

                let mut tx = get_state_transition_tx(&[]);
                tx.instructions.push(Instruction {
                    program_id: Pubkey::default(),
                    accounts: vec![],
                    data: vec![],
                });

                let tx_to_sign = TransactionToSign {
                    tx_bytes: &bitcoin::consensus::serialize(&tx),
                    inputs_to_sign: &[InputToSign {
                        index: 0,
                        signer: Pubkey::default(),
                    }],
                };

                msg!("Transaction to sign: {:?}", tx_to_sign);
                set_transaction_to_sign(&[], tx_to_sign);

                return true;
            }
        }
        false
    }

    pub fn add_bitcoin_transaction(&mut self, tx: Transaction) {
        let tx_to_sign = TransactionToSign {
            tx_bytes: &bitcoin::consensus::serialize(&tx),
            inputs_to_sign: vec![],
        };

        self.bitcoin_transactions.push(tx);

        msg!("Added Bitcoin transaction: {:?}", tx);
        set_transaction_to_sign(&[], tx_to_sign);

        // Add UTXO management
        for (vout, output) in tx.output.iter().enumerate() {
            let outpoint = OutPoint::new(tx.txid(), vout as u32);
            let utxo_meta = UtxoMeta {
                txid: tx.txid(),
                vout: vout as u32,
                amount: output.value,
                script_pubkey: output.script_pubkey.clone(),
            };
            self.utxos.insert(outpoint, utxo_meta);
        }
    }

    // Function to calculate depositor collateral gain
    pub fn get_depositor_collateral_gain(&self, depositor: AccountId) -> Result<Balance, ProgramError> {
        let snapshot = self.depositor_snapshots.get(&depositor).ok_or(ProgramError::InvalidAccountData)?;
        let current_product_factor = self.calculate_current_product_factor(); // This needs to be implemented based on your system's specifics

        let gain = snapshot.last_deposit * (current_product_factor - snapshot.product_factor) / snapshot.product_factor;
        Ok(gain)
    }

    // Function to calculate the current product factor based on the pool's state
    fn calculate_current_product_factor(&self) -> Balance {
        // and are updated similarly to the Solidity version during liquidations or other events.
        let scale_factor = SCALE_FACTOR; //
        let current_p = self.P; // Assuming `P` is stored and updated in the struct

        // Calculate the compounded product factor based on the current scale
        let compounded_product_factor = if self.current_scale > 0 {
            current_p / scale_factor.pow(self.current_scale as u32)
        } else {
            current_p
        };

        compounded_product_factor
    }

    // Update depositor snapshot after each deposit or withdrawal
    pub fn update_depositor_snapshot(&mut self, depositor: AccountId, new_value: Balance) {
        if new_value == 0 {
            // Clear the snapshot if the new value is zero
            self.depositor_snapshots.remove(&depositor);
        } else {
            let current_product_factor = self.calculate_current_product_factor(); // P
            let babel_gain_sum = self.calculate_babel_gain_sum(); // G, needs implementation
            let collateral_gain_sum = self.calculate_collateral_gain_sum(); // S, needs implementation

            let snapshot = DepositorSnapshot {
                last_deposit: new_value,
                product_factor: current_product_factor,
                babel_gain: babel_gain_sum,
                collateral_gain: collateral_gain_sum,
                last_snapshot_time: self.get_current_time(),
                stored_pending_reward: 0, // Initialize stored_pending_reward to 0
            };
            self.depositor_snapshots.insert(depositor, snapshot);

            // Iterate through collateral tokens to update deposit sums, needs implementation
        }
    }

    // Function to calculate the Babel gain sum based on the pool's state
    fn calculate_babel_gain_sum(&self) -> Balance {
        
        let current_g = self.G; 

        // Calculate the total Babel gain sum based on the current epoch and scale
        let total_babel_gain_sum = if self.current_scale > 0 {
            // If there is a scale adjustment, calculate the adjusted gain sum
            current_g / SCALE_FACTOR.pow(self.current_scale as u32)
        } else {
            // If no scale adjustment is needed, return the current gain sum
            current_g
        };

        total_babel_gain_sum
    }

    // Function to calculate the collateral gain sum based on the pool's state
    fn calculate_collateral_gain_sum(&self) -> Balance {
      
        let scale_factor = SCALE_FACTOR; 
        let current_s = self.S; 

        // Calculate the total collateral gain sum based on the current epoch and scale
        let total_collateral_gain_sum = if self.current_scale > 0 {
            // If there is a scale adjustment, calculate the adjusted gain sum
            current_s / scale_factor.pow(self.current_scale as u32)
        } else {
            // If no scale adjustment is needed, return the current gain sum
            current_s
        };

        total_collateral_gain_sum
    }

    // Function to get the current time as a Unix timestamp
    fn get_current_time(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    // Function to calculate claimable Babel rewards for a depositor
    pub fn claimable_reward(&self, depositor: AccountId) -> Result<Balance, ProgramError> {
        let snapshot = self.depositor_snapshots.get(&depositor).ok_or(ProgramError::InvalidAccountData)?;
        let current_babel_gain = self.calculate_current_babel_gain(); // Ensure this function is implemented to calculate the current total Babel gain

        // Calculate the claimable Babel reward using the formula provided
        let babel_reward = snapshot.last_deposit * (current_babel_gain - snapshot.babel_gain) / snapshot.product_factor;
        Ok(babel_reward)
    }

    // Function to calculate the current Babel gain based on the pool's state
    fn calculate_current_babel_gain(&self) -> Balance {
       
        let current_g = self.G; 
        let current_babel_gain = if self.current_scale > 0 {
            current_g / SCALE_FACTOR.pow(self.current_scale as u32)
        } else {
            // If no scale adjustment is needed, return the current gain
            current_g
        };

        current_babel_gain
    }

    // Function to get the compounded deposit for a depositor
    pub fn get_compounded_debt_deposit(&self, depositor: AccountId) -> Result<Balance, ProgramError> {
        let snapshot = self.depositor_snapshots.get(&depositor).ok_or(ProgramError::InvalidAccountData)?;
        self._get_compounded_stake_from_snapshots(snapshot.last_deposit, snapshot)
    }

    // Internal function to calculate compounded stake from snapshots
    fn _get_compounded_stake_from_snapshots(&self, initial_stake: Balance, snapshots: &DepositorSnapshot) -> Result<Balance, ProgramError> {
        let snapshot_p = snapshots.product_factor;
        let current_p = self.calculate_current_product_factor(); // Ensure this function is implemented
        let scale_diff = self.current_scale() - snapshots.scale; // Assuming a function to get current scale

        let compounded_stake = if scale_diff == 0 {
            (initial_stake * current_p) / snapshot_p
        } else if scale_diff == 1 {
            (initial_stake * current_p) / (snapshot_p * SCALE_FACTOR)
        } else {
            // if scale_diff >= 2
            max(initial_stake / BILLION, (initial_stake * current_p) / (snapshot_p * SCALE_FACTOR.pow(scale_diff)))
        };

        Ok(compounded_stake)
    }

    // Function to claim collateral gains for a depositor
    pub fn claim_collateral_gains(&mut self, recipient: AccountId, collateral_indexes: Vec<CollateralId>) -> Result<(), ProgramError> {
        for index in collateral_indexes {
            let collateral_data = self.collaterals.get_mut(&index).ok_or(ProgramError::InvalidAccountData)?;
            // Transfer collateral to recipient and reset gains
            self._claim_collateral_gains(recipient, collateral_data)?;
        }
        Ok(())
    }

    // Internal function to perform the actual transfer of collateral gains
    fn _claim_collateral_gains(&mut self, recipient: AccountId, collateral_data: &mut CollateralData) -> Result<(), ProgramError> {
        // Assuming a method to transfer collateral to recipient
        collateral_data.transfer(recipient, collateral_data.amount)?;
        collateral_data.amount = 0; // Reset the collateral amount after transfer

        // Placeholder for event emission, implement as needed
        // msg!("Collateral gains transferred: recipient = {}, amount = {}", recipient, collateral_data.amount);

        Ok(())
    }

    // Internal function to calculate Babel gain from snapshots
    fn _get_babel_gain_from_snapshots(&self, initial_stake: Balance, snapshots: &DepositorSnapshot) -> Result<Balance, ProgramError> {
        let snapshot_p = snapshots.product_factor;
        let snapshot_g = snapshots.babel_gain;
        let current_scale = self.current_scale(); // Assuming a function to get current scale
        let scale_diff = current_scale - snapshots.scale;

        let babel_gain = if scale_diff == 0 {
            (initial_stake * (self.calculate_current_babel_gain()? - snapshot_g)) / snapshot_p
        } else if scale_diff == 1 {
            (initial_stake * (self.calculate_current_babel_gain()? - snapshot_g)) / snapshot_p / SCALE_FACTOR
        } else {
            // if scale_diff >= 2
            0
        };

        Ok(babel_gain)
    }

    // Function to accrue rewards for a depositor
    fn accrue_rewards(&mut self, depositor: AccountId) -> Result<(), ProgramError> {
        let reward = self.claimable_reward(depositor)?;
        let snapshot = self.depositor_snapshots.get_mut(&depositor).ok_or(ProgramError::InvalidAccountData)?;
        snapshot.stored_pending_reward += reward; // Assuming `stored_pending_reward` field in `DepositorSnapshot`
        Ok(())
    }

    // External function to claim rewards for a depositor
    pub fn claim_reward(&mut self, recipient: AccountId) -> Result<(), ProgramError> {
        let reward = self._claim_reward(recipient)?;
        // Assuming a method to transfer tokens from the vault to the recipient
        self.transfer_from_vault(recipient, reward)?;
        // Placeholder for event emission
        msg!("RewardClaimed: recipient = {}, reward = {}", recipient, reward);
        Ok(())
    }

    // Function to be called by the vault to claim rewards on behalf of a depositor
    pub fn vault_claim_reward(&mut self, claimant: AccountId, vault_id: AccountId) -> Result<(), ProgramError> {
        self.only_vault(vault_id)?;
        self.claim_reward(claimant)
    }

    // Internal function to perform the actual reward calculation and update
    fn _claim_reward(&mut self, account: AccountId) -> Result<Balance, ProgramError> {
        let snapshot = self.depositor_snapshots.get(&account).ok_or(ProgramError::InvalidAccountData)?;
        let initial_deposit = snapshot.last_deposit;
        let reward = self.calculate_current_babel_gain()? - snapshot.babel_gain;
        let compounded_deposit = self.get_compounded_debt_deposit(account)?;
        let final_reward = reward + compounded_deposit; // Simplified reward calculation

        // Update snapshot after claiming reward
        snapshot.babel_gain = self.calculate_current_babel_gain()?;
        snapshot.last_deposit = compounded_deposit;

        Ok(final_reward)
    }

    // Helper function to ensure only the vault can call certain methods
    fn only_vault(&self, vault_id: AccountId) -> Result<(), ProgramError> {
        if self.owner != vault_id {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    // Placeholder for a method to transfer tokens from the vault
    fn transfer_from_vault(&mut self, recipient: AccountId, amount: Balance) -> Result<(), ProgramError> {
        // Logic to transfer tokens from the vault to the recipient
        msg!("Transferred {} tokens from vault to {}", amount, recipient);
        Ok(())
    }
}

// Types for AccountId, Balance, CollateralId, DebtToken, and CollateralData would need to be defined or imported
type AccountId = u64; 
type Balance = u64; 
type CollateralId = u64; 

struct DebtToken;

impl DebtToken {
    fn transfer(&self, to: AccountId, amount: Balance) {
        let mut balances: HashMap<AccountId, Balance> = HashMap::new();
        let sender_balance = balances.entry(self.owner).or_insert(0);
        if *sender_balance < amount {
            panic!("Insufficient balance");
        }
        *sender_balance -= amount;
        let recipient_balance = balances.entry(to).or_insert(0);
        *recipient_balance += amount;
    }
}

struct CollateralData {
    amount: Balance,
    is_sunset: bool,
}

impl CollateralData {
    fn new() -> Self {
        CollateralData {
            amount: 0,
            is_sunset: false,
        }
    }

    fn start_sunset(&mut self) {
        self.is_sunset = true;
    }

    fn offset(&mut self, debt_to_offset: Balance, coll_to_add: Balance) {
        if !self.is_sunset {
            self.amount += coll_to_add;
            if self.amount >= debt_to_offset {
                self.amount -= debt_to_offset;
            } else {
                panic!("Insufficient collateral to offset the debt");
            }
        }
    }

    // Method to transfer collateral
    pub fn transfer(&mut self, recipient: AccountId, amount: Balance) -> Result<(), ProgramError> {
        // Logic to transfer collateral, adjust as per actual implementation
        // For example, updating balances in a ledger or database
        // Assuming a simple placeholder here
        msg!("Transferred {} collateral to {}", amount, recipient);
        Ok(())
    }
}

// Add to the StabilityPool struct to store snapshots
#[derive(BorshSerialize, BorshDeserialize)]
pub struct DepositorSnapshot {
    last_deposit: Balance,
    product_factor: Balance,
    last_snapshot_time: u64,
    babel_gain: Balance,
    stored_pending_reward: Balance, // Field to store pending rewards
}

impl StabilityPool {
    // Add a HashMap to store each depositor's snapshot
    depositor_snapshots: HashMap<AccountId, DepositorSnapshot>,
}