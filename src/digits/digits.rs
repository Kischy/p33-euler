pub fn get_digits(num: u128) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::get_digits;

    #[test]
    fn get_digits_tests() {
        assert_eq!(get_digits(0), vec![0]);
        assert_eq!(get_digits(123), vec![1, 2, 3]);
        assert_eq!(get_digits(5032), vec![5, 0, 3, 2]);
        assert_eq!(get_digits(9), vec![9]);
    }
}
