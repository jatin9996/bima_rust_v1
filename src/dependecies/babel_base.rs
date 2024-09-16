#![cfg_attr(not(feature = "std"), no_std)]

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelBase {
    debt_gas_compensation: u128,
}

impl BabelBase {
    pub const DECIMAL_PRECISION: u128 = 1e18 as u128;
    pub const CCR: u128 = 2.25e18 as u128; // 225%
    pub const PERCENT_DIVISOR: u128 = 200; // dividing by 200 yields 0.5%

    pub fn new(gas_compensation: u128) -> Self {
        Self {
            debt_gas_compensation: gas_compensation,
        }
    }

    // --- Gas compensation functions ---

    // Returns the composite debt (drawn debt + gas compensation) of a trove, for the purpose of ICR calculation
    pub fn get_composite_debt(&self, debt: u128) -> u128 {
        debt + self.debt_gas_compensation
    }

    pub fn get_net_debt(&self, debt: u128) -> u128 {
        debt - self.debt_gas_compensation
    }

    // Return the amount of collateral to be drawn from a trove's collateral and sent as gas compensation.
    pub fn get_coll_gas_compensation(&self, entire_coll: u128) -> u128 {
        entire_coll / Self::PERCENT_DIVISOR
    }

    pub fn require_user_accepts_fee(
        &self,
        fee: u128,
        amount: u128,
        max_fee_percentage: u128,
    ) -> Result<(), &'static str> {
        let fee_percentage = (fee * Self::DECIMAL_PRECISION) / amount;
        if fee_percentage > max_fee_percentage {
            return Err("Fee exceeded provided maximum");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_debt() {
        let contract = BabelBase::new(100);
        assert_eq!(contract.get_composite_debt(200), 300);
    }

    #[test]
    fn test_net_debt() {
        let contract = BabelBase::new(100);
        assert_eq!(contract.get_net_debt(200), 100);
    }

    #[test]
    fn test_coll_gas_compensation() {
        let contract = BabelBase::new(100);
        assert_eq!(contract.get_coll_gas_compensation(200), 1); // Since PERCENT_DIVISOR is 200
    }

    #[test]
    fn test_require_user_accepts_fee() {
        let contract = BabelBase::new(100);
        assert_eq!(contract.require_user_accepts_fee(50, 200, 1e18 as u128), Ok(()));
        assert_eq!(contract.require_user_accepts_fee(100, 200, 0.25e18 as u128), Err("Fee exceeded provided maximum"));
    }

    #[test]
    fn test_borsh_serialization() {
        let contract = BabelBase::new(100);
        let serialized = contract.try_to_vec().unwrap();
        let deserialized: BabelBase = BabelBase::try_from_slice(&serialized).unwrap();
        assert_eq!(contract.debt_gas_compensation, deserialized.debt_gas_compensation);
    }
}
