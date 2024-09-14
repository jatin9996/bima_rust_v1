use crate::models::{TokenBalance, GaugeWeightVote, Utxo, AuthorityMessage};
use anyhow::Result;
use sdk::utxo::{UtxoSet, UtxoRef};
use sdk::zkvm::{ZkProgram, ZkProof};

pub struct GaugeWeightVote {
    pub gauge: String,
    pub weight: u128,
}

pub struct TokenBalance {
    pub token: String,
    pub amount: u128,
}

pub trait ICurveProxy {
    /// Adds liquidity to the pool, creating a new UTXO representing the updated state.
    fn add_liquidity(&mut self, utxos: &UtxoSet, liquidity_amount: u128, authority: &AuthorityMessage) -> Result<Utxo>;

    /// Removes liquidity from the pool, updating the UTXO state accordingly.
    fn remove_liquidity(&mut self, utxos: &UtxoSet, liquidity_amount: u128, authority: &AuthorityMessage) -> Result<Utxo>;

    /// Votes for gauge weights using a zero-knowledge proof to validate the vote without revealing voter identity.
    fn vote_for_gauge_weights(&mut self, utxos: &UtxoSet, votes: &[GaugeWeightVote], authority: &AuthorityMessage) -> Result<ZkProof>;

    /// Executes arbitrary functions within the contract in a zero-knowledge environment.
    fn execute(&mut self, utxos: &UtxoSet, data: &[u8], authority: &AuthorityMessage) -> Result<ZkProof>;

    /// Claims fees accumulated from the liquidity pool, returning a new UTXO with updated balances.
    fn claim_fees(&mut self, utxos: &UtxoSet, authority: &AuthorityMessage) -> Result<Utxo>;

    /// Transfers tokens between addresses, updating UTXOs accordingly.
    fn transfer_tokens(&mut self, utxos: &UtxoSet, transfers: &[TokenBalance], authority: &AuthorityMessage) -> Result<Vec<Utxo>>;

    /// Deploys the contract to the network, compiling it into an ELF file and using RPC to interact with it.
    fn deploy(&self) -> Result<()>;

    /// Queries the state UTXOs and liquidity pool data.
    fn query_state(&self, utxo_ref: &UtxoRef) -> Result<Utxo>;
}
