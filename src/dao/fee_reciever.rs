use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use bitcoin::{self, Transaction, OutPoint, Script};
use archnetwork::transaction_to_sign::TransactionToSign;
use archnetwork::Pubkey;
use arch_program::{
    get_account_script_pubkey,
    get_bitcoin_tx,
    set_transaction_to_sign,
    validate_utxo_ownership,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    instruction::Instruction,
    msg,
    utxo::UtxoMeta,
};

#[derive(BorshSerialize, BorshDeserialize)]
struct Token {
    balances: HashMap<Pubkey, Balance>,
    allowances: HashMap<(Pubkey, Pubkey), Balance>,
}

impl Token {
    fn new() -> Self {
        Self {
            balances: HashMap::new(),
            allowances: HashMap::new(),
        }
    }

    fn transfer(&mut self, from: Pubkey, to: Pubkey, amount: Balance) -> Result<(), String> {
        let from_balance = *self.balances.get(&from).unwrap_or(&0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.balances.insert(from, from_balance - amount);
        let to_balance = *self.balances.get(&to).unwrap_or(&0);
        self.balances.insert(to, to_balance + amount);
        Ok(())
    }

    fn approve(&mut self, owner: Pubkey, spender: Pubkey, amount: Balance) {
        self.allowances.insert((owner, spender), amount);
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct FeeReceiver {
    owner: Pubkey,
    tokens: HashMap<String, Token>,
    bitcoin_transactions: HashMap<String, Transaction>,
    utxos: HashMap<OutPoint, UtxoMeta>,
}

impl FeeReceiver {
    fn new(owner: Pubkey) -> Self {
        Self {
            owner,
            tokens: HashMap::new(),
            bitcoin_transactions: HashMap::new(),
            utxos: HashMap::new(),
        }
    }

    fn transfer_ownership(&mut self, new_owner: Pubkey) -> Result<(), String> {
        self.only_owner()?;
        self.owner = new_owner;
        Ok(())
    }

    fn transfer_token(&mut self, token_id: String, receiver: Pubkey, amount: Balance) -> Result<(), String> {
        self.only_owner()?;
        let token = self.tokens.get_mut(&token_id).ok_or("Token not found".to_string())?;
        let from_balance = *token.balances.get(&self.owner).unwrap_or(&0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        let to_balance = *token.balances.get(&receiver).unwrap_or(&0);
        token.balances.insert(self.owner, from_balance - amount);
        token.balances.insert(receiver, to_balance + amount);
        Ok(())
    }

    fn set_token_approval(&mut self, token_id: String, spender: Pubkey, amount: Balance) -> Result<(), String> {
        self.only_owner()?;
        let token = self.tokens.get_mut(&token_id).ok_or("Token not found".to_string())?;
        token.allowances.insert((self.owner, spender), amount);
        Ok(())
    }

    fn process_bitcoin_transaction(&mut self, tx_id: String, transaction: Transaction) -> Result<(), String> {
        self.only_owner()?;
        
        let tx_bytes = self.create_transaction_bytes(&transaction);
        let inputs_to_sign = vec![self.owner.clone()];
        let transaction_to_sign = TransactionToSign::new(tx_bytes, inputs_to_sign);

        self.sign_transaction(&transaction_to_sign);

        self.bitcoin_transactions.insert(tx_id, transaction);
        Ok(())
    }

    fn create_transaction_bytes(&self, transaction: &Transaction) -> Vec<u8> {
        bincode::serialize(transaction).unwrap_or_default()
    }

    fn sign_transaction(&self, transaction: &TransactionToSign) {
        println!("Signing transaction with inputs: {:?}", transaction.inputs_to_sign);
    }

    fn only_owner(&self) -> Result<(), String> {
        if self.env_caller() != self.owner {
            return Err("Caller is not owner".to_string());
        }
        Ok(())
    }

    fn env_caller(&self) -> Pubkey {
        self.owner
    }

    fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: Script) {
        let outpoint = OutPoint::new(tx.txid(), vout);
        let utxo_meta = UtxoMeta {
            txid: tx.txid(),
            vout,
            value,
            script_pubkey,
        };
        self.utxos.insert(outpoint, utxo_meta);
    }

    fn spend_utxo(&mut self, outpoint: OutPoint) -> Result<(), String> {
        self.only_owner()?;
        self.utxos.remove(&outpoint).ok_or("UTXO not found".to_string())?;
        Ok(())
    }

    fn process_arch_transaction(&mut self, tx_id: String, transaction: Transaction) -> Result<(), String> {
        self.only_owner()?;
        
        msg!("Processing Arch transaction with ID: {}", tx_id);

        let mut state_tx = get_state_transition_tx(&[self.owner.clone()]);
        state_tx.input.push(transaction.input[0].clone());

        let inputs_to_sign = vec![InputToSign {
            index: 0,
            signer: self.owner.clone(),
        }];

        let tx_to_sign = TransactionToSign {
            tx_bytes: &bitcoin::consensus::serialize(&state_tx),
            inputs_to_sign: &inputs_to_sign,
        };

        msg!("Transaction to sign: {:?}", tx_to_sign);

        set_transaction_to_sign(&[self.owner.clone()], tx_to_sign);

        self.bitcoin_transactions.insert(tx_id, transaction);
        Ok(())
    }
}

type Balance = u32;