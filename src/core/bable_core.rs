use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use borsh::{BorshDeserialize, BorshSerialize};
use borsh::maybestd::io::{Error, ErrorKind};
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

const OWNERSHIP_TRANSFER_DELAY: u64 = 86400 * 3; // 3 days

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelCore {
    utxos: HashMap<(Vec<u8>, u32), UTXO>,
    fee_receiver: String,
    price_feed: String,
    owner: Pubkey,
    pending_owner: Option<Pubkey>,
    ownership_transfer_deadline: Option<u64>,
    guardian: Pubkey,
    paused: bool,
    start_time: u64,
}

impl BabelCore {
    pub fn new(owner: Pubkey, guardian: Pubkey, price_feed: String, fee_receiver: String) -> Self {
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            utxos: HashMap::default(),
            fee_receiver,
            price_feed,
            owner,
            pending_owner: None,
            ownership_transfer_deadline: None,
            guardian,
            paused: false,
            start_time: start_time - (start_time % (7 * 86400)), // Rounded down to the nearest week
        }
    }

    pub fn set_fee_receiver(&mut self, new_fee_receiver: String) {
        self.fee_receiver = new_fee_receiver;
        msg!("FeeReceiverSet: {}", new_fee_receiver); // Event-like log
    }

    pub fn set_price_feed(&mut self, new_price_feed: String) {
        self.price_feed = new_price_feed;
        msg!("PriceFeedSet: {}", new_price_feed); // Event-like log
    }

    pub fn set_guardian(&mut self, new_guardian: Pubkey) {
        msg!("GuardianSet: Changed from {:?} to {:?}", self.guardian, new_guardian);
        self.guardian = new_guardian;
    }

    pub fn set_paused(&mut self, new_paused: bool) -> Result<(), ProgramError> {
        // Allow both the guardian and the owner to pause the system
        if new_paused && (self.guardian != self.owner) && (self.owner != get_caller()) {
            return Err(ProgramError::Unauthorized);
        }
        self.paused = new_paused;
        if new_paused {
            msg!("Paused"); // Event-like log for pausing
        } else {
            msg!("Unpaused"); // Event-like log for unpausing
        }
        Ok(())
    }
  
    pub fn commit_transfer_ownership(&mut self, caller: String, new_owner: String, accounts: &[AccountInfo]) -> Result<(), ProgramError>{
        if self.is_owner(&caller) {
            self.pending_owner = Some(new_owner.clone());
            self.ownership_transfer_deadline = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + OWNERSHIP_TRANSFER_DELAY);
            msg!("NewOwnerCommitted: Committed by {}, New owner pending: {}, Deadline: {}", caller, new_owner, self.ownership_transfer_deadline.unwrap());
            // Additional logging for ownership commitment
            let tx_bytes = self.serialize()?; // Serialize the current state
            let inputs_to_sign = vec![InputToSign::new(caller.to_string(), tx_bytes.clone())]; // Create inputs to sign
            let transaction_to_sign = TransactionToSign::new(tx_bytes, inputs_to_sign); // Create the transaction to sign
            set_transaction_to_sign(accounts, transaction_to_sign)?; // Set the transaction to sign
            Ok(())
        } else {
            Err(ProgramError::Unauthorized)
        }
    }

    pub fn accept_transfer_ownership(&mut self, caller: String) -> Result<(), ProgramError> {
        if let Some(ref pending_owner) = self.pending_owner {
            if caller == *pending_owner && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() >= self.ownership_transfer_deadline.unwrap() {
                msg!("NewOwnerAccepted: Ownership transferred from {} to {}", self.owner, pending_owner);
                self.owner = pending_owner.clone();
                self.pending_owner = None;
                self.ownership_transfer_deadline = None;
                msg!("NewOwnerAccepted: Ownership accepted by {}", caller);
                Ok(())
            } else {
                Err(ProgramError::Unauthorized)
            }
        } else {
            Err(ProgramError::InvalidArgument)
        }
    }

    pub fn revoke_transfer_ownership(&mut self) {
        msg!("NewOwnerRevoked: Revoked by {}, Pending owner was {}", self.owner, self.pending_owner.unwrap_or_default());
        self.pending_owner = None;
        self.ownership_transfer_deadline = None;
    }

    pub fn transfer_utxo(&mut self, input_utxos: Vec<(Vec<u8>, u32)>, output_utxos: Vec<UTXO>, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
        let mut input_value = 0;
        for (txid, vout) in input_utxos.iter() {
            let utxo = self.utxos.get(&(*txid, *vout)).expect("UTXO not found");
            input_value += utxo.value;
            self.utxos.remove(&(*txid, *vout));
        }

        let mut output_value = 0;
        for utxo in output_utxos.iter() {
            output_value += utxo.value;
            let txid = utxo.txid.clone();
            let vout = utxo.vout;
            self.utxos.insert((txid, vout), utxo.clone());
        }

        if input_value != output_value {
            return Err(ProgramError::Custom(502)); // Custom error for value mismatch
        }

        // Validate UTXO ownership
        for (txid, vout) in input_utxos.iter() {
            validate_utxo_ownership(accounts, txid, *vout)?;
        }

        // Create state transition transaction
        let mut tx = get_state_transition_tx(accounts);
        for (txid, vout) in input_utxos.iter() {
            let utxo = self.utxos.get(&(*txid, *vout)).expect("UTXO not found");
            tx.input.push(utxo.clone());
        }

        let tx_bytes = bitcoin::consensus::serialize(&tx);
        let inputs_to_sign = input_utxos.iter().enumerate().map(|(i, (txid, vout))| {
            InputToSign::new(i as u32, tx_bytes.clone())
        }).collect::<Vec<_>>();

        let transaction_to_sign = TransactionToSign::new(tx_bytes, inputs_to_sign);
        set_transaction_to_sign(accounts, transaction_to_sign)?;

        Ok(())
    }

    pub fn adjust_trove(&mut self, user: String, adjustment: i64) {
        println!("Adjusting trove for user: {}, adjustment: {}", user, adjustment);
    }

    pub fn trigger_emergency(&mut self, caller: Pubkey) {
        if caller == self.guardian {
            self.paused = true;
            println!("Emergency triggered, system paused by {}", caller);
        } else {
            panic!("Unauthorized attempt to trigger emergency by {}", caller);
        }
    }

    pub fn admin_vote(&mut self, proposal_id: u32, vote: bool) {
        println!("Admin voting on proposal: {}, vote: {}", proposal_id, vote);
    }

    fn is_owner(&self, caller: &Pubkey) -> bool {
        &self.owner == caller
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        self.try_to_vec().map_err(|e| Error::new(ErrorKind::Other, e))
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, Error> {
        Self::try_from_slice(data).map_err(|e| Error::new(ErrorKind::Other, e))
    }
}