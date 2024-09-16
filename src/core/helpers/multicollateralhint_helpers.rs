#![cfg_attr(not(feature = "std"), no_std)]

use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MultiCollateralHintHelpers {
    borrower_operations: Box<dyn IBorrowerOperations>,
}

pub trait ITroveManager {
    fn sorted_troves(&self) -> Vec<String>;
    fn mcr(&self) -> u128;
    fn current_icr(&self, trove: String, price: u128) -> u128;
    fn get_entire_debt_and_coll(&self, trove: String) -> (u128, u128);
    fn get_trove_owners_count(&self) -> usize;
    fn get_trove_from_trove_owners_array(&self, index: usize) -> String;
    fn get_nominal_icr(&self, trove: String) -> u128;
}

pub trait IBorrowerOperations {
    fn min_net_debt(&self) -> u128;
}

impl MultiCollateralHintHelpers {
    pub fn new(borrower_operations: Box<dyn IBorrowerOperations>) -> Self {
        Self { borrower_operations }
    }

    pub fn compute_nominal_cr(coll: u128, debt: u128) -> u128 {
        coll / debt
    }

    pub fn compute_cr(coll: u128, debt: u128, price: u128) -> u128 {
        (coll * price) / debt
    }

    pub fn get_redemption_hints(
        &self,
        trove_manager: &dyn ITroveManager,
        debt_amount: u128,
        price: u128,
        max_iterations: u128,
    ) -> (String, u128, u128) {
        let mut sorted_troves = trove_manager.sorted_troves();
        let mut remaining_debt = debt_amount;
        let mut current_trove = sorted_troves.last().cloned().unwrap_or_default();
        let mcr = trove_manager.mcr();

        while trove_manager.current_icr(current_trove.clone(), price) < mcr {
            current_trove = sorted_troves.pop().unwrap_or_default();
        }

        let first_redemption_hint = current_trove.clone();
        let min_net_debt = self.borrower_operations.min_net_debt();
        let mut partial_redemption_hint_nicr = 0;
        let mut truncated_debt_amount = 0;

        while !current_trove.is_empty() && remaining_debt > 0 && max_iterations > 0 {
            let (debt, coll) = trove_manager.get_entire_debt_and_coll(current_trove.clone());
            let net_debt = debt;

            if net_debt > remaining_debt {
                if net_debt > min_net_debt {
                    let max_redeemable_debt = std::cmp::min(remaining_debt, net_debt - min_net_debt);
                    let new_coll = coll - (max_redeemable_debt * price);
                    let new_debt = net_debt - max_redeemable_debt;
                    partial_redemption_hint_nicr = Self::compute_nominal_cr(new_coll, new_debt);

                    remaining_debt -= max_redeemable_debt;
                    truncated_debt_amount += max_redeemable_debt;
                }
                break;
            } else {
                remaining_debt -= net_debt;
                truncated_debt_amount += net_debt;
            }

            current_trove = sorted_troves.pop().unwrap_or_default();
            max_iterations -= 1;
        }

        (first_redemption_hint, partial_redemption_hint_nicr, truncated_debt_amount)
    }
}