#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod babel_base {
    #[ink(storage)]
    pub struct BabelBase {
        debt_gas_compensation: Balance,
    }

    impl BabelBase {
        pub const DECIMAL_PRECISION: u128 = 1e18 as u128;
        pub const CCR: u128 = 2.25e18 as u128; // 225%
        pub const PERCENT_DIVISOR: u128 = 200; // dividing by 200 yields 0.5%

        #[ink(constructor)]
        pub fn new(gas_compensation: Balance) -> Self {
            Self {
                debt_gas_compensation: gas_compensation,
            }
        }

        // --- Gas compensation functions ---

        // Returns the composite debt (drawn debt + gas compensation) of a trove, for the purpose of ICR calculation
        #[ink(message)]
        pub fn get_composite_debt(&self, debt: Balance) -> Balance {
            debt + self.debt_gas_compensation
        }

        #[ink(message)]
        pub fn get_net_debt(&self, debt: Balance) -> Balance {
            debt - self.debt_gas_compensation
        }

        // Return the amount of collateral to be drawn from a trove's collateral and sent as gas compensation.
        #[ink(message)]
        pub fn get_coll_gas_compensation(&self, entire_coll: Balance) -> Balance {
            entire_coll / Self::PERCENT_DIVISOR
        }

        #[ink(message)]
        pub fn require_user_accepts_fee(
            &self,
            fee: Balance,
            amount: Balance,
            max_fee_percentage: Balance,
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

        #[ink::test]
        fn test_composite_debt() {
            let contract = BabelBase::new(100);
            assert_eq!(contract.get_composite_debt(200), 300);
        }

        #[ink::test]
        fn test_net_debt() {
            let contract = BabelBase::new(100);
            assert_eq!(contract.get_net_debt(200), 100);
        }

        #[ink::test]
        fn test_coll_gas_compensation() {
            let contract = BabelBase::new(100);
            assert_eq!(contract.get_coll_gas_compensation(200), 1); // Since PERCENT_DIVISOR is 200
        }

        #[ink::test]
        fn test_require_user_accepts_fee() {
            let contract = BabelBase::new(100);
            assert_eq!(contract.require_user_accepts_fee(50, 200, 1e18 as Balance), Ok(()));
            assert_eq!(contract.require_user_accepts_fee(100, 200, 0.25e18 as Balance), Err("Fee exceeded provided maximum"));
        }
    }
}
