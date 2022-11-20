pub fn get_digits(num: u128) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn get_number(digits: &[u32]) -> u128 {
    if digits.is_empty() {
        return 0;
    }

    let mut num: u128 = u128::from(digits[0]);

    for i in 1..digits.len() {
        num *= 10;
        num += u128::from(digits[i]);
    }
    num
}

#[cfg(test)]
mod tests {
    use super::get_digits;
    use super::get_number;

    #[test]
    fn get_digits_tests() {
        assert_eq!(get_digits(0), vec![0]);
        assert_eq!(get_digits(123), vec![1, 2, 3]);
        assert_eq!(get_digits(5032), vec![5, 0, 3, 2]);
        assert_eq!(get_digits(9), vec![9]);
    }

    #[test]
    fn get_number_tests() {
        assert_eq!(get_number(&vec![0]), 0);
        assert_eq!(get_number(&vec![1, 2, 3]), 123);
        assert_eq!(get_number(&vec![5, 0, 3, 2]), 5032);
        assert_eq!(get_number(&vec![9]), 9);
    }
}
