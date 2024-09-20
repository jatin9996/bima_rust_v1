use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::interfaces::trove_manager::TroveManager;
use bitcoin::{self, Transaction};
use arch_program::{
    account::AccountInfo,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    msg,
    program::{get_account_script_pubkey, get_caller_address, set_transaction_to_sign},
    pubkey::Pubkey,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Node {
    exists: bool,
    next_id: Option<u32>,
    prev_id: Option<u32>,
    nicr: u256,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SortedTroves {
    head: Option<u32>,
    tail: Option<u32>,
    size: u32,
    nodes: HashMap<u32, Node>,
    trove_manager: Option<TroveManager>,
    transactions: Vec<Transaction>,
    utxos: HashMap<OutPoint, UtxoMeta>,
}

impl SortedTroves {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
            nodes: HashMap::new(),
            trove_manager: None,
            transactions: Vec::new(),
            utxos: HashMap::new(),
        }
    }

    pub fn set_addresses(&mut self, trove_manager: TroveManager) {
        assert!(self.trove_manager.is_none(), "Already set");
        self.trove_manager = Some(trove_manager);
    }

    fn require_caller_is_trove_manager(&self) {
        let caller: Pubkey = get_caller_address();
        assert!(self.trove_manager.as_ref().map_or(false, |tm| tm.address == caller), "Caller is not the TroveManager");
    }

    pub fn insert(&mut self, id: u32, nicr: u256, prev_id: Option<u32>, next_id: Option<u32>) {
        self.require_caller_is_trove_manager();

        let node = Node {
            exists: true,
            next_id,
            prev_id,
            nicr,
        };

        if let Some(prev_id) = prev_id {
            if let Some(prev_node) = self.nodes.get_mut(&prev_id) {
                prev_node.next_id = Some(id);
            }
        } else {
            self.head = Some(id);
        }

        if let Some(next_id) = next_id {
            if let Some(next_node) = self.nodes.get_mut(&next_id) {
                next_node.prev_id = Some(id);
            }
        } else {
            self.tail = Some(id);
        }

        self.nodes.insert(id, node);
        self.size += 1;

        let mut tx = get_state_transition_tx(&self.nodes);
        msg!("State transition transaction created: {:?}", tx);

        let input_to_sign = InputToSign {
            index: 0,
            signer: get_caller_address(),
        };
        let tx_to_sign = TransactionToSign {
            tx_bytes: bitcoin::consensus::serialize(&tx),
            inputs_to_sign: vec![input_to_sign],
        };
        msg!("Transaction to sign: {:?}", tx_to_sign);

        set_transaction_to_sign(&self.nodes, tx_to_sign);
    }

    pub fn remove(&mut self, id: u32) {
        self.require_caller_is_trove_manager();

        if let Some(node) = self.nodes.remove(&id) {
            if let Some(prev_id) = node.prev_id {
                if let Some(prev_node) = self.nodes.get_mut(&prev_id) {
                    prev_node.next_id = node.next_id;
                }
            } else {
                self.head = node.next_id;
            }

            if let Some(next_id) = node.next_id {
                if let Some(next_node) = self.nodes.get_mut(&next_id) {
                    next_node.prev_id = node.prev_id;
                }
            } else {
                self.tail = node.prev_id;
            }

            self.size -= 1;

            let mut tx = get_state_transition_tx(&self.nodes);
            msg!("State transition transaction created: {:?}", tx);

            let input_to_sign = InputToSign {
                index: 0,
                signer: get_caller_address(),
            };
            let tx_to_sign = TransactionToSign {
                tx_bytes: bitcoin::consensus::serialize(&tx),
                inputs_to_sign: vec![input_to_sign],
            };
            msg!("Transaction to sign: {:?}", tx_to_sign);

            set_transaction_to_sign(&self.nodes, tx_to_sign);
        }
    }

    pub fn re_insert(&mut self, id: u32, new_nicr: u256, new_prev_id: Option<u32>, new_next_id: Option<u32>) {
        self.remove(id);
        self.insert(id, new_nicr, new_prev_id, new_next_id);
    }

    fn valid_insert_position(&self, nicr: u256, prev_id: Option<u32>, next_id: Option<u32>) -> bool {
        if let Some(prev_id) = prev_id {
            if let Some(prev_node) = self.nodes.get(&prev_id) {
                if prev_node.nicr > nicr {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(next_id) = next_id {
            if let Some(next_node) = self.nodes.get(&next_id) {
                if next_node.nicr < nicr {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().expect("Serialization failed")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        Self::try_from_slice(data).expect("Deserialization failed")
    }

    pub fn add_transaction(&mut self, tx: Transaction, inputs_to_sign: Vec<InputToSign>) {
        let tx_bytes = tx.serialize();
        let transaction_to_sign = self.create_transaction_to_sign(tx_bytes, inputs_to_sign);
        self.transactions.push(tx);
    }

    pub fn create_transaction_to_sign(&self, tx_bytes: Vec<u8>, inputs_to_sign: Vec<InputToSign>) -> TransactionToSign {
        TransactionToSign {
            tx_bytes,
            inputs_to_sign,
        }
    }

    pub fn add_utxo(&mut self, tx: &Transaction, vout: u32, value: u64, script_pubkey: Vec<u8>) {
        let outpoint = OutPoint::new(tx.txid(), vout);
        let utxo_meta = UtxoMeta {
            txid: tx.txid(),
            vout,
            value,
            script_pubkey,
        };
        self.utxos.insert(outpoint, utxo_meta);
    }

    pub fn spend_utxo(&mut self, outpoint: OutPoint) {
        self.utxos.remove(&outpoint);
    }
}