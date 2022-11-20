mod digits;

use std::ops::{Mul, MulAssign};

pub struct Fraction {
    nummerator: u128,
    denominator: u128,
}

fn get_cloned_vec(vec: &Vec<u32>, excluded_index: usize) -> Vec<u32> {
    let mut new_vec = Vec::with_capacity(vec.len() - 1);
    new_vec.extend_from_slice(&vec[0..excluded_index]);
    new_vec.extend_from_slice(&vec[excluded_index + 1..]);
    new_vec
}

impl Fraction {
    pub fn get_denominator(&self) -> u128 {
        self.denominator
    }

    pub fn new(nummerator: u128, denominator: u128) -> Fraction {
        Fraction {
            nummerator,
            denominator,
        }
    }

    pub fn is_curious_non_trival_fraction(&self) -> bool {
        if self.nummerator == self.denominator {
            return false;
        }

        let nummerator_digits = digits::get_digits(self.nummerator);
        let denominator_digits = digits::get_digits(self.denominator);

        for i in 0..nummerator_digits.len() {
            let num_digit = nummerator_digits[i];

            if num_digit == 0 {
                continue;
            }

            for j in 0..denominator_digits.len() {
                let den_digit = denominator_digits[j];

                if den_digit == 0 {
                    continue;
                }

                if num_digit == den_digit {
                    let new_frac = Fraction::new(
                        digits::get_number(&get_cloned_vec(&nummerator_digits, i)),
                        digits::get_number(&get_cloned_vec(&denominator_digits, j)),
                    );
                    if self.eq(&new_frac) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_reduced_fraction(&self) -> Fraction {
        let mut gcd = num::integer::gcd(self.nummerator, self.denominator);
        if gcd == 0 {
            gcd = 1;
        }
        Fraction::new(self.nummerator / gcd, self.denominator / gcd)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.get_reduced_fraction();
        let rhs = other.get_reduced_fraction();

        lhs.nummerator == rhs.nummerator && lhs.denominator == rhs.denominator
    }
}

impl Mul for &Fraction {
    type Output = Fraction;
    fn mul(self, rhs: Self) -> Fraction {
        Fraction::new(
            self.nummerator * rhs.nummerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl MulAssign<&Fraction> for Fraction {
    fn mul_assign(&mut self, rhs: &Self) {
        self.nummerator *= rhs.nummerator;
        self.denominator *= rhs.denominator;
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    #[test]
    fn is_curious_fraction_tests() {
        assert_eq!(
            Fraction::new(30, 50).is_curious_non_trival_fraction(),
            false
        );
        assert_eq!(Fraction::new(49, 98).is_curious_non_trival_fraction(), true);
        assert_eq!(Fraction::new(1, 2).is_curious_non_trival_fraction(), false);
        assert_eq!(
            Fraction::new(30, 51).is_curious_non_trival_fraction(),
            false
        );
        assert_eq!(
            Fraction::new(49, 99).is_curious_non_trival_fraction(),
            false
        );
    }

    #[test]
    fn get_reduced_fraction_test() {
        let mut frac = Fraction::new(30, 50).get_reduced_fraction();
        assert_eq!(frac.nummerator, 3);
        assert_eq!(frac.denominator, 5);
        frac = Fraction::new(49, 98).get_reduced_fraction();
        assert_eq!(frac.nummerator, 1);
        assert_eq!(frac.denominator, 2);
        frac = Fraction::new(49, 99).get_reduced_fraction();
        assert_eq!(frac.nummerator, 49);
        assert_eq!(frac.denominator, 99);
    }

    #[test]
    fn test_equality_operator() {
        assert_eq!(Fraction::new(30, 50) == Fraction::new(30, 50), true);
        assert_eq!(Fraction::new(30, 50) == Fraction::new(30, 51), false);
        assert_eq!(Fraction::new(3, 5) == Fraction::new(30, 50), true);
        assert_eq!(Fraction::new(49, 98) == Fraction::new(4, 8), true);
        assert_eq!(Fraction::new(49, 99) == Fraction::new(4, 8), false);
    }
}
