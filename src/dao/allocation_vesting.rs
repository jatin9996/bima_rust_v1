#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use crate::interfaces::token_locker::{ITokenLocker, LockData};
use crate::dependencies::babel_ownable::BabelOwnable;
use borsh::{BorshDeserialize, BorshSerialize};
use bitcoin::{self, Transaction};  // Importing bitcoin crate
use archnetwork::transaction_to_sign::TransactionToSign;  // Importing TransactionToSign
use archnetwork::Pubkey;  // Importing Pubkey
use archnetwork::utxo::UtxoMeta;  // Import UtxoMeta
use arch_program::{
    get_account_script_pubkey, get_bitcoin_tx, set_transaction_to_sign, validate_utxo_ownership,
    transaction_to_sign::TransactionToSign, input_to_sign::InputToSign, msg,
};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AllocationSplit {
    recipient: Pubkey,  // Using Pubkey to represent AccountId
    points: u32,
    number_of_weeks: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AllocationState {
    points: u32,
    number_of_weeks: u8,
    claimed: u128,  // Using u128 to represent Balance for simplicity
    preclaimed: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AllocationVesting {
    allocations: HashMap<Pubkey, AllocationState>,
    max_total_preclaim_pct: u32,
    total_allocation: u128,
    vesting_start: Option<u64>,  // Using u64 to represent Timestamp for simplicity
    owner: Pubkey,
    token: Box<dyn ERC20Trait>,
    token_locker: Box<dyn ITokenLocker>,
    utxos: HashMap<OutPoint, UtxoMeta>,  // Add UTXO management
}

#[derive(Debug)]
pub enum ContractError {
    NothingToClaim,
    CannotLock,
    WrongMaxTotalPreclaimPct,
    PreclaimTooLarge,
    AllocationsMismatch,
    ZeroTotalAllocation,
    ZeroAllocation,
    ZeroNumberOfWeeks,
    DuplicateAllocation,
    InsufficientPoints,
    LockedAllocation,
    IllegalVestingStart,
    VestingAlreadyStarted,
    IncompatibleVestingPeriod,
    UtxoNotFound,
    InvalidUtxo,
}

impl AllocationVesting {
    pub fn new(total_allocation: u128, max_total_preclaim_pct: u32, owner: Pubkey, token: Box<dyn ERC20Trait>, token_locker: Box<dyn ITokenLocker>) -> Self {
        assert!(total_allocation > 0);
        assert!(max_total_preclaim_pct <= 20);
        Self {
            allocations: HashMap::new(),
            max_total_preclaim_pct,
            total_allocation,
            vesting_start: None,
            owner,
            token,
            token_locker,
            utxos: HashMap::new(),
        }
    }

    pub fn set_allocations(&mut self, allocation_splits: Vec<AllocationSplit>, vesting_start: u64) -> Result<(), ContractError> {
        if self.vesting_start.is_some() {
            return Err(ContractError::VestingAlreadyStarted);
        }
        self.vesting_start = Some(vesting_start);

        let mut total_points = 0;
        for split in &allocation_splits {
            if split.points == 0 {
                return Err(ContractError::ZeroAllocation);
            }
            if split.number_of_weeks == 0 {
                return Err(ContractError::ZeroNumberOfWeeks);
            }
            if self.allocations.contains_key(&split.recipient) {
                return Err(ContractError::DuplicateAllocation);
            }
            total_points += split.points;
        }

        if total_points == 0 {
            return Err(ContractError::InsufficientPoints);
        }

        for split in allocation_splits {
            self.allocations.insert(
                split.recipient.clone(),
                AllocationState {
                    points: split.points,
                    number_of_weeks: split.number_of_weeks,
                    claimed: 0,
                    preclaimed: 0,
                },
            );
        }

        Ok(())
    }

    pub fn transfer_points(&mut self, from: &Pubkey, to: &Pubkey, points: u32) -> Result<(), ContractError> {
        let from_allocation = self.allocations.get(from).ok_or(ContractError::NothingToClaim)?;
        let to_allocation = self.allocations.get(to).unwrap_or(&AllocationState {
            points: 0,
            number_of_weeks: 0,
            claimed: 0,
            preclaimed: 0,
        });

        if to_allocation.number_of_weeks != 0 && to_allocation.number_of_weeks != from_allocation.number_of_weeks {
            return Err(ContractError::IncompatibleVestingPeriod);
        }

        let total_vested = self.vested_at(self.current_timestamp(), from_allocation.points, from_allocation.number_of_weeks);
        if total_vested < from_allocation.claimed {
            return Err(ContractError::LockedAllocation);
        }

        if points == 0 {
            return Err(ContractError::ZeroAllocation);
        }

        if from_allocation.points < points {
            return Err(ContractError::InsufficientPoints);
        }

        let claimed = self.claim(from)?;

        let claimed_adjustment = (claimed * points as u128) / from_allocation.points as u128;

        self.allocations.get_mut(from).unwrap().points -= points;
        self.allocations.get_mut(from).unwrap().claimed -= claimed_adjustment;

        let to_allocation = self.allocations.entry(to.clone()).or_insert(AllocationState {
            points: 0,
            number_of_weeks: from_allocation.number_of_weeks,
            claimed: 0,
            preclaimed: 0,
        });

        to_allocation.points += points;
        to_allocation.claimed += claimed_adjustment;

        Ok(())
    }

    pub fn lock_future_claims(&mut self, account: &Pubkey, amount: u128) -> Result<(), ContractError> {
        self.lock_future_claims_with_receiver(account, account, amount)
    }

    pub fn lock_future_claims_with_receiver(&mut self, account: &Pubkey, receiver: &Pubkey, amount: u128) -> Result<(), ContractError> {
        let allocation = self.allocations.get(account).ok_or(ContractError::CannotLock)?;
        if allocation.points == 0 || self.vesting_start.is_none() {
            return Err(ContractError::CannotLock);
        }

        let claimed_updated = if self.claimable_at(self.current_timestamp(), allocation.points, allocation.claimed, allocation.number_of_weeks) > 0 {
            self.claim(account)?
        } else {
            allocation.claimed
        };

        let user_allocation = (allocation.points as u128 * self.total_allocation) / TOTAL_POINTS;
        let unclaimed = user_allocation - claimed_updated;
        let preclaimed = allocation.preclaimed;
        let max_total_preclaim = (self.max_total_preclaim_pct as u128 * user_allocation) / 100;
        let left_to_preclaim = max_total_preclaim - preclaimed;

        let amount = if amount == 0 {
            left_to_preclaim.min(unclaimed)
        } else {
            if preclaimed + amount > max_total_preclaim || amount > unclaimed {
                return Err(ContractError::PreclaimTooLarge);
            }
            amount
        };

        let amount = (amount / self.token_locker.lock_to_token_ratio()) * self.token_locker.lock_to_token_ratio();

        self.allocations.get_mut(account).unwrap().claimed += amount;
        self.allocations.get_mut(account).unwrap().preclaimed += amount;

        self.token.transfer_from(&self.vault, &self.token_locker, amount)?;
        self.token_locker.lock(receiver, amount / self.token_locker.lock_to_token_ratio(), 52);

        Ok(())
    }

    pub fn claim(&mut self, account: &Pubkey) -> Result<u128, ContractError> {
        let allocation = self.allocations.get(account).ok_or(ContractError::NothingToClaim)?;
        self._claim(account, allocation.points, allocation.claimed, allocation.number_of_weeks)
    }

    fn _claim(&mut self, account: &Pubkey, points: u32, claimed: u128, number_of_weeks: u8) -> Result<u128, ContractError> {
        if points == 0 {
            return Err(ContractError::NothingToClaim);
        }

        let claimable = self.claimable_at(self.current_timestamp(), points, claimed, number_of_weeks);
        if claimable == 0 {
            return Err(ContractError::NothingToClaim);
        }

        let claimed_updated = claimed + claimable;
        self.allocations.get_mut(account).unwrap().claimed = claimed_updated;

        self.token.transfer_from(&self.vault, account, claimable)?;

        Ok(claimed_updated)
    }

    fn claimable_at(&self, when: u64, points: u32, claimed: u128, number_of_weeks: u8) -> u128 {
        let total_vested = self.vested_at(when, points, number_of_weeks);
        if total_vested > claimed {
            total_vested - claimed
        } else {
            0
        }
    }

    fn vested_at(&self, when: u64, points: u32, number_of_weeks: u8) -> u128 {
        if self.vesting_start.is_none() || number_of_weeks == 0 {
            return 0;
        }

        let vesting_start = self.vesting_start.unwrap();
        let vesting_weeks = number_of_weeks as u64 * 1_000_000; // assuming 1 week = 1_000_000 for simplicity
        let vesting_end = vesting_start + vesting_weeks;
        let end_time = if when >= vesting_end { vesting_end } else { when };
        let time_since_start = end_time - vesting_start;

        (self.total_allocation * time_since_start as u128 * points as u128) / (TOTAL_POINTS as u128 * vesting_weeks as u128)
    }

    fn current_timestamp(&self) -> u64 {
        // Implement a method to get the current timestamp
        0
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization should not fail")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization should not fail")
    }

    // Add methods to handle Bitcoin transactions
    pub fn create_bitcoin_transaction(&self, inputs: Vec<bitcoin::TxIn>, outputs: Vec<bitcoin::TxOut>) -> TransactionToSign {
        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: inputs.clone(),
            output: outputs.clone(),
        };
        let tx_bytes = bitcoin::consensus::encode::serialize(&tx);
        let tx_to_sign = TransactionToSign {
            tx_bytes,
            inputs_to_sign: inputs.iter().enumerate().map(|(i, _)| InputToSign {
                index: i as u32,
                signer: get_account_script_pubkey(&self.owner),
            }).collect(),
        };

        msg!("Transaction to sign: {:?}", tx_to_sign);
        tx_to_sign
    }

    pub fn parse_bitcoin_transaction(&self, raw_tx: &[u8]) -> Result<Transaction, bitcoin::consensus::encode::Error> {
        get_bitcoin_tx(raw_tx)
    }

    pub fn verify_bitcoin_transaction(&self, tx: &Transaction) -> bool {
        // Implement verification logic using validate_utxo_ownership
        for input in &tx.input {
            if !validate_utxo_ownership(&input.previous_output) {
                return false;
            }
        }
        true
    }

    pub fn set_transaction_to_sign(&self, tx_to_sign: TransactionToSign) {
        set_transaction_to_sign(&[self.owner], tx_to_sign);
    }

    pub fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: Script) {
        let outpoint = OutPoint::new(tx.txid(), vout);
        let utxo = UtxoMeta::new(outpoint, value, script_pubkey);
        self.utxos.insert(outpoint, utxo);
    }

    pub fn spend_utxo(&mut self, outpoint: OutPoint) -> Result<(), ContractError> {
        if self.utxos.remove(&outpoint).is_none() {
            return Err(ContractError::UtxoNotFound);
        }
        Ok(())
    }

    pub fn validate_utxo(&self, outpoint: &OutPoint) -> bool {
        self.utxos.contains_key(outpoint)
    }

    // ... existing code ...
}

// Implement ERC20Trait and other necessary traits and methods

fn main() {
    let owner_pubkey = Pubkey::new_unique();
    let mut vesting = AllocationVesting::new(1000000, 20, owner_pubkey, Box::new(ERC20Trait::default()), Box::new(ITokenLocker::default()));
    println!("{:?}", vesting);
}