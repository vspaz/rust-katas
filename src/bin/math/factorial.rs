pub fn get_factorial(num: i64) -> i64 {
    let mut product = 1;
    for i in 1..num + 1 {
        product *= i;
    }
    product
}

#[cfg(test)]
mod tests {
    use crate::factorial::get_factorial;

    #[test]
    fn test_get_factorial_ok() {
        assert_eq!(120, get_factorial(5));
    }
}
