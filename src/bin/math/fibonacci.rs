pub fn fib(num: i64) -> i64 {
    let (mut a, mut b): (i64, i64) = (1, 1);
    for _ in 1..num {
        (a, b) = (b, a + b);
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::fibonacci::fib;

    #[test]
    fn test_fib_ok() {
        assert_eq!(55, fib(10));
    }
}
