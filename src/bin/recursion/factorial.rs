pub fn get_factorial(num: i64) -> i64 {
    if num == 1 {
        return 1;
    }
    return num * get_factorial(num - 1);
}