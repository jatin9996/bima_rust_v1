use arch_program::utxo::UtxoMeta;
use arch_program::entrypoint::ProgramResult;

pub trait BorrowerOperationsInterface {
    fn add_liquidity(&mut self, utxo: &UtxoMeta, amount: u64) -> ProgramResult;
    fn remove_liquidity(&mut self, utxo: &UtxoMeta, amount: u64) -> ProgramResult;
}
