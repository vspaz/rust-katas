pub fn get_digit_count(mut num: u32) -> u32 {
    let mut count: u32 = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::digitcounter::get_digit_count;

    #[test]
    fn test_get_digit_count_ok() {
        assert_eq!(get_digit_count(13033), 5);
    }

    #[test]
    fn test_get_digit_count_with_zero() {
        assert_eq!(get_digit_count(0), 0);
    }

    #[test]
    fn test_get_digit_count_with_one() {
        assert_eq!(get_digit_count(1), 1);
    }
}
