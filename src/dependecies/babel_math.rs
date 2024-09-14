use num_bigint::BigUint;
use num_traits::{Zero, One};
use arch_sdk::{Utxo, UtxoSet, zkvm};

pub struct BabelMath;

impl BabelMath {
    const DECIMAL_PRECISION: u128 = 1e18 as u128;
    const NICR_PRECISION: u128 = 1e20 as u128;

    pub fn min(a: BigUint, b: BigUint) -> BigUint {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn max(a: BigUint, b: BigUint) -> BigUint {
        if a >= b {
            a
        } else {
            b
        }
    }

    pub fn dec_mul(x: BigUint, y: BigUint) -> BigUint {
        let prod_xy = &x * &y;
        let half_precision = BigUint::from(Self::DECIMAL_PRECISION / 2u128);
        (prod_xy + half_precision) / BigUint::from(Self::DECIMAL_PRECISION)
    }

    pub fn dec_pow(base: BigUint, minutes: u64) -> BigUint {
        let cap_minutes = 525600000;
        let mut n = if minutes > cap_minutes {
            cap_minutes
        } else {
            minutes
        };

        if n == 0 {
            return BigUint::from(Self::DECIMAL_PRECISION);
        }

        let mut y = BigUint::from(Self::DECIMAL_PRECISION);
        let mut x = base.clone();

        while n > 1 {
            if n % 2 == 0 {
                x = Self::dec_mul(x.clone(), x.clone());
                n /= 2;
            } else {
                y = Self::dec_mul(x.clone(), y.clone());
                x = Self::dec_mul(x.clone(), x.clone());
                n = (n - 1) / 2;
            }
        }

        Self::dec_mul(x, y)
    }

    pub fn get_absolute_difference(a: BigUint, b: BigUint) -> BigUint {
        if a >= b {
            a - b
        } else {
            b - a
        }
    }

    pub fn compute_nominal_cr(coll: BigUint, debt: BigUint) -> BigUint {
        if debt > BigUint::zero() {
            (&coll * BigUint::from(Self::NICR_PRECISION)) / debt
        } else {
            BigUint::from(u128::MAX) // Represents "infinite" CR
        }
    }

    pub fn compute_cr(coll: BigUint, debt: BigUint, price: BigUint) -> BigUint {
        if debt > BigUint::zero() {
            (&coll * &price) / debt
        } else {
            BigUint::from(u128::MAX) // Represents "infinite" CR
        }
    }

    pub fn compute_cr_without_price(coll: BigUint, debt: BigUint) -> BigUint {
        if debt > BigUint::zero() {
            &coll / debt
        } else {
            BigUint::from(u128::MAX) // Represents "infinite" CR
        }
    }
}

// Additional functions for UTXO handling and ZKVM integration can be added here.