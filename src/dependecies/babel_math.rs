use num_bigint::BigUint;
use num_traits::{One, Zero};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelMath;

impl BabelMath {
    const DECIMAL_PRECISION: u128 = 1e18 as u128;
    const NICR_PRECISION: u128 = 1e20 as u128;

    pub fn min(a: Balance, b: Balance) -> Balance {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn max(a: Balance, b: Balance) -> Balance {
        if a >= b {
            a
        } else {
            b
        }
    }

    pub fn dec_mul(x: Balance, y: Balance) -> Balance {
        let prod_xy = x * y;
        let half_precision = Self::DECIMAL_PRECISION / 2;
        (prod_xy + half_precision) / Self::DECIMAL_PRECISION
    }

    pub fn dec_pow(base: Balance, minutes: u64) -> Balance {
        let cap_minutes = 525600000;
        let mut n = if minutes > cap_minutes {
            cap_minutes
        } else {
            minutes
        };

        if n == 0 {
            return Self::DECIMAL_PRECISION;
        }

        let mut y = Self::DECIMAL_PRECISION;
        let mut x = base;

        while n > 1 {
            if n % 2 == 0 {
                x = Self::dec_mul(x, x);
                n /= 2;
            } else {
                y = Self::dec_mul(x, y);
                x = Self::dec_mul(x, x);
                n = (n - 1) / 2;
            }
        }

        Self::dec_mul(x, y)
    }

    pub fn get_absolute_difference(a: Balance, b: Balance) -> Balance {
        if a >= b {
            a - b
        } else {
            b - a
        }
    }

    pub fn compute_nominal_cr(coll: Balance, debt: Balance) -> Balance {
        if debt > 0 {
            (coll * Self::NICR_PRECISION) / debt
        } else {
            u128::MAX
        }
    }

    pub fn compute_cr(coll: Balance, debt: Balance, price: Balance) -> Balance {
        if debt > 0 {
            (coll * price) / debt
        } else {
            u128::MAX
        }
    }

    pub fn compute_cr_without_price(coll: Balance, debt: Balance) -> Balance {
        if debt > 0 {
            coll / debt
        } else {
            u128::MAX
        }
    }
}

type Balance = u64; // Placeholder type definition

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(BabelMath::min(5, 10), 5);
        assert_eq!(BabelMath::min(10, 5), 5);
        assert_eq!(BabelMath::min(5, 5), 5);
    }

    #[test]
    fn test_max() {
        assert_eq!(BabelMath::max(5, 10), 10);
        assert_eq!(BabelMath::max(10, 5), 10);
        assert_eq!(BabelMath::max(5, 5), 5);
    }

    #[test]
    fn test_dec_mul() {
        assert_eq!(BabelMath::dec_mul(2 * BabelMath::DECIMAL_PRECISION, 3 * BabelMath::DECIMAL_PRECISION), 6 * BabelMath::DECIMAL_PRECISION);
    }

    #[test]
    fn test_dec_pow() {
        assert_eq!(BabelMath::dec_pow(2 * BabelMath::DECIMAL_PRECISION, 3), 8 * BabelMath::DECIMAL_PRECISION);
    }

    #[test]
    fn test_get_absolute_difference() {
        assert_eq!(BabelMath::get_absolute_difference(10, 5), 5);
        assert_eq!(BabelMath::get_absolute_difference(5, 10), 5);
    }

    #[test]
    fn test_compute_nominal_cr() {
        assert_eq!(BabelMath::compute_nominal_cr(100, 50), 2 * BabelMath::NICR_PRECISION);
        assert_eq!(BabelMath::compute_nominal_cr(100, 0), u128::MAX);
    }

    #[test]
    fn test_compute_cr() {
        assert_eq!(BabelMath::compute_cr(100, 50, BabelMath::DECIMAL_PRECISION), 2 * BabelMath::DECIMAL_PRECISION);
        assert_eq!(BabelMath::compute_cr(100, 0, BabelMath::DECIMAL_PRECISION), u128::MAX);
    }

    #[test]
    fn test_compute_cr_without_price() {
        assert_eq!(BabelMath::compute_cr_without_price(100, 50), 2);
        assert_eq!(BabelMath::compute_cr_without_price(100, 0), u128::MAX);
    }
}