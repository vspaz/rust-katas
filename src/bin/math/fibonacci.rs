pub fn fib(num: i64) -> i64 {
    let (mut a,mut b): (i64, i64) = (1, 1);
    for i in 1..num {
        (a, b) = (b, a + b);
    }
    a
}