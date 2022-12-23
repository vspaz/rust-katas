pub fn reverse_digit(num: i64) -> i64 {
    let mut reversed_num = 0;
    let mut original_num = num;
    while original_num > 0 {
        reversed_num = reversed_num * 10 + original_num % 10;
        original_num /= 10;
    }
    reversed_num
}

#[cfg(test)]
mod tests {
    use crate::math::reversedigit::reverse_digit;

    #[test]
    fn test_reverse_digit_count_ok() {
        assert_eq!(54321, reverse_digit(12345));
    }
}
