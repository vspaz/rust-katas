pub fn get_digit_count(num: u32) -> u32 {
    let mut count: u32 = 0;
    let mut remainder = num;
    while remainder > 0 {
        remainder /= 10;
        count += 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use crate::math::digitcounter::get_digit_count;

    #[test]
    fn test_get_digit_count_ok() {
        assert_eq!(get_digit_count(13033), 5);
    }

    #[test]
    fn test_get_digit_count_with_zero() {
        assert_eq!(get_digit_count(0), 0);
    }
}