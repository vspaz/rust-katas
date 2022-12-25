pub fn get_factorial(num: i64) -> i64 {
    if num == 1 {
        return 1;
    }
    return num * get_factorial(num - 1);
}

#[cfg(test)]
mod tests {
    use crate::factorial::get_factorial;

    #[test]
    fn test_get_factorial_ok() {
        assert_eq!(120, get_factorial(5));
    }
}