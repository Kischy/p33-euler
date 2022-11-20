pub struct Fraction {
    nummerator: i128,
    denominator: i128,
}

impl Fraction {
    pub fn new(nummerator: i128, denominator: i128) -> Fraction {
        Fraction {
            nummerator,
            denominator,
        }
    }

    pub fn is_curious_fraction(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    #[test]
    fn is_curious_fraction_tests() {
        assert_eq!(Fraction::new(30, 50).is_curious_fraction(), true);
        assert_eq!(Fraction::new(49, 98).is_curious_fraction(), true);
        assert_eq!(Fraction::new(1, 2).is_curious_fraction(), false);
        assert_eq!(Fraction::new(30, 51).is_curious_fraction(), false);
        assert_eq!(Fraction::new(49, 99).is_curious_fraction(), false);
    }
}
