#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod borrower_operations {
    use super::*;

    #[ink(storage)]
    pub struct BorrowerOperationsState {
        min_net_debt: u64,
        trove_manager: TroveManager,
        debt_token: DebtToken,
        babel_base: BabelBase,
        babel_ownable: BabelOwnable,
        delegated_ops: DelegatedOps,
    }

    impl BorrowerOperationsState {
        #[ink(constructor)]
        pub fn new(min_net_debt: u64, trove_manager: TroveManager, debt_token: DebtToken, babel_base: BabelBase, babel_ownable: BabelOwnable, delegated_ops: DelegatedOps) -> Self {
            Self { min_net_debt, trove_manager, debt_token, babel_base, babel_ownable, delegated_ops }
        }

        #[ink(message)]
        pub fn adjust_trove(&mut self, user_id: AccountId, coll_change: i64, debt_change: i64) {
            self.babel_ownable.only_owner();
            self.delegated_ops.ensure_caller_or_delegated(user_id);

            self.trove_manager.adjust_trove(user_id, coll_change, debt_change);
            if debt_change > 0 {
                self.debt_token.issue(debt_change as u64);
            } else {
                self.debt_token.burn((-debt_change) as u64);
            }
        }
    }
}