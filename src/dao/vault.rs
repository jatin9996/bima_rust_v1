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
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
    bitcoin::{self, Transaction},
};

#[derive(BorshSerialize, BorshDeserialize)]
struct Vault {
    babel_token: String,
    emission_schedule: String,
    token_locker: String,
    boost_calculator: String,
    incentive_voting: String,
    babel_ownable: BabelOwnable,
    system_start: SystemStart,
    unallocated_total: u128,
    weekly_emissions: HashMap<u64, u128>,
    allocated: HashMap<String, u128>,
    receivers: HashMap<u64, Receiver>,
    boost_delegation: HashMap<String, BoostDelegation>,
}

impl Vault {
    pub fn new(
        babel_token: String,
        emission_schedule: String,
        token_locker: String,
        boost_calculator: String,
        incentive_voting: String,
        babel_ownable: BabelOwnable,
        system_start: SystemStart,
    ) -> Self {
        Self {
            babel_token,
            emission_schedule,
            token_locker,
            boost_calculator,
            incentive_voting,
            babel_ownable,
            system_start,
            unallocated_total: 0,
            weekly_emissions: HashMap::new(),
            allocated: HashMap::new(),
            receivers: HashMap::new(),
            boost_delegation: HashMap::new(),
        }
    }

    pub fn set_weekly_emission(&mut self, week: u64, amount: u128) {
        let total_emissions = self.get_total_weekly_emissions(week); 
        self.weekly_emissions.insert(week, total_emissions);
        self.unallocated_total -= total_emissions;
        self.lock_tokens(amount, 52); // Lock for 1 year
    }

    pub fn transfer_tokens(&mut self, receiver: String, amount: u128) {
        if self.unallocated_total >= amount {
            // Transfer logic here
            self.unallocated_total -= amount;
            // Use Arch SDK to transfer tokens
            let receiver_pubkey = Pubkey::new(&receiver.as_bytes());
            let tx = get_bitcoin_tx(&self.babel_token, &receiver_pubkey, amount);
            invoke(&tx);
        } else {
            panic!("Insufficient unallocated tokens for transfer");
        }
    }

    pub fn increase_unallocated_supply(&mut self, amount: u128) {
        self.unallocated_total += amount;
        // Use Arch SDK to increase allowance
        let tx = get_state_transition_tx(&self.babel_token, amount);
        invoke(&tx);
    }

    fn get_total_weekly_emissions(&self, week: u64) -> u128 {
        // Calculation logic here
        1000 // Dummy value
    }

    fn lock_tokens(&self, amount: u128, duration: u64) {
        // Use Arch SDK to lock tokens
        let tx = get_state_transition_tx(&self.token_locker, amount);
        invoke(&tx);
    }

    pub fn register_receiver(&mut self, id: u64, account: String) -> bool {
        let receiver = Receiver {
            account,
            is_active: true,
        };
        self.receivers.insert(id, receiver);
        true
    }

    pub fn set_receiver_is_active(&mut self, id: u64, is_active: bool) -> bool {
        if let Some(receiver) = self.receivers.get_mut(&id) {
            receiver.is_active = is_active;
            true
        } else {
            false
        }
    }

    pub fn set_boost_delegation_params(&mut self, sender: String, is_enabled: bool, fee_pct: u16, callback: Option<String>) -> bool {
        let delegation = BoostDelegation {
            is_enabled,
            fee_pct,
            callback,
        };
        self.boost_delegation.insert(sender, delegation);
        true
    }

    pub fn claim_boost_delegation_fees(&mut self, claimant: String) -> u128 {
        let amount = self.stored_pending_reward.get(&claimant).cloned().unwrap_or(0);
        if amount >= self.lock_to_token_ratio {
            self.stored_pending_reward.insert(claimant.clone(), 0);
            self.transfer_or_lock(amount, claimant.clone());
            amount
        } else {
            0
        }
    }

    fn transfer_or_lock(&mut self, amount: u128, receiver: String) {
        if self.lock_weeks == 0 {
            // If no lock duration is specified, transfer the tokens directly
            let receiver_pubkey = Pubkey::new(&receiver.as_bytes());
            let tx = get_bitcoin_tx(&self.babel_token, &receiver_pubkey, amount);
            invoke(&tx);
        } else {
            // Calculate the amount to lock based on the lock-to-token ratio
            let lock_amount = amount / self.lock_to_token_ratio;
            
            // Store the remaining amount as pending reward
            self.stored_pending_reward.insert(receiver.clone(), amount - lock_amount * self.lock_to_token_ratio);
            
            if lock_amount > 0 {
                // Lock the calculated amount for the specified duration
                let tx = get_state_transition_tx(&self.token_locker, lock_amount);
                invoke(&tx);
            }
        }
    }

    pub fn batch_claim_rewards(&mut self, claimant: String, rewards: Vec<u64>) -> bool {
        let mut total_claimed = 0;
        for reward_id in rewards {
            if let Some(reward) = self.receivers.get(&reward_id) {
                if reward.is_active {
                    total_claimed += self.claim_boost_delegation_fees(claimant.clone());
                }
            }
        }
        total_claimed > 0
    }

    pub fn claimable_reward_after_boost(&self, account: String, reward_contract: String) -> (u128, u128) {
        let base_reward = self.get_base_reward(&account, &reward_contract);
        let boost_factor = self.get_boost_factor(&account);
        let boosted_reward = base_reward * boost_factor / 100;
        (base_reward, boosted_reward)
    }

    fn get_base_reward(&self, account: &String, reward_contract: &String) -> u128 {
        // Logic to get the base reward from the reward contract
        1000 // Dummy value for base reward
    }

    fn get_boost_factor(&self, account: &String) -> u128 {
        //  logic to calculate the boost factor for the account
        let base_boost = 100; // Base boost factor (100%)
        let additional_boost = if let Some(delegation) = self.boost_delegation.get(account) {
            if delegation.is_enabled {
                delegation.fee_pct as u128 //  use fee percentage as additional boost
            } else {
                0
            }
        } else {
            0
        };
        base_boost + additional_boost
    }

    fn allocate_total_weekly(&mut self, current_week: u64) {
        // Retrieve the total emissions for the current week
        let total_emissions = self.weekly_emissions.get(&current_week).cloned().unwrap_or(0);

        // Calculate the total allocation for all receivers
        let mut total_allocated = 0;
        for receiver in self.receivers.values() {
            if receiver.is_active {
                //  allocation logic: equally distribute emissions among active receivers
                let allocation = total_emissions / self.receivers.len() as u128;
                total_allocated += allocation;
                self.allocated.insert(receiver.account.clone(), allocation);
            }
        }

        // Update the unallocated total
        self.unallocated_total = self.unallocated_total.saturating_sub(total_allocated);
    }

    fn transfer_allocated(&mut self, amount: u128, receiver: String) {
        // Check if the receiver has enough allocated tokens
        if let Some(allocated_amount) = self.allocated.get_mut(&receiver) {
            if *allocated_amount >= amount {
                // Deduct the amount from the allocated tokens
                *allocated_amount -= amount;
                
                // Transfer the tokens to the receiver
                let receiver_pubkey = Pubkey::new(&receiver.as_bytes());
                let tx = get_bitcoin_tx(&self.babel_token, &receiver_pubkey, amount);
                invoke(&tx);
            } else {
                // Handle the case where the allocated amount is insufficient
                panic!("Insufficient allocated tokens for transfer");
            }
        } else {
            // Handle the case where the receiver is not found in the allocated map
            panic!("Receiver not found in allocated tokens");
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Failed to serialize Vault")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Failed to deserialize Vault")
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Receiver {
    account: String,
    is_active: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct BoostDelegation {
    is_enabled: bool,
    fee_pct: u16,
    callback: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct BabelOwnable;

#[derive(BorshSerialize, BorshDeserialize)]
struct SystemStart;