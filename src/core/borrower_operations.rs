use crate::interfaces::trove_manager::{TroveManager, Trove};
use crate::interfaces::debt_token::DebtToken;
use crate::dependecies::babel_base::BabelBase;
use crate::dependecies::babel_math::BabelMath;
use crate::dependecies::babel_ownable::BabelOwnable;
use crate::dependecies::delegated_ops::DelegatedOps;

struct BorrowerOperationsState {
    min_net_debt: u64,
    trove_manager: TroveManager,
    debt_token: DebtToken,
    babel_base: BabelBase,
    babel_ownable: BabelOwnable,
    delegated_ops: DelegatedOps,
    // Additional state variables can be added here
}

impl BorrowerOperationsState {
    pub fn new(min_net_debt: u64, trove_manager: TroveManager, debt_token: DebtToken, babel_base: BabelBase, babel_ownable: BabelOwnable, delegated_ops: DelegatedOps) -> Self {
        Self { min_net_debt, trove_manager, debt_token, babel_base, babel_ownable, delegated_ops }
    }

    pub fn adjust_trove(&mut self, user_id: String, coll_change: i64, debt_change: i64) {
        self.babel_ownable.only_owner(); // Ensure only the owner can adjust the trove
        self.delegated_ops.ensure_caller_or_delegated(user_id.clone()); // Or a delegated user

        self.trove_manager.adjust_trove(user_id.clone(), coll_change, debt_change);
        if debt_change > 0 {
            self.debt_token.issue(debt_change as u64);
        } else {
            self.debt_token.burn((-debt_change) as u64);
        }

        println!("Trove adjusted for user: {}, Collateral change: {}, Debt change: {}", user_id, coll_change, debt_change);
    }
}

fn main() {
    // Example instantiation and usage
    let trove_manager = TroveManager::new();
    let debt_token = DebtToken::new();
    let babel_base = BabelBase::new(100); // Example gas compensation
    let babel_ownable = BabelOwnable::new("owner_account_id".to_string()); // Example owner account
    let delegated_ops = DelegatedOps::new();

    let mut borrower_ops = BorrowerOperationsState::new(1000, trove_manager, debt_token, babel_base, babel_ownable, delegated_ops);
    borrower_ops.adjust_trove("user123".to_string(), 500, -100); 
}