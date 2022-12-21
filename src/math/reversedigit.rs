pub fn reverse_digit(num: i64) -> i64 {
    let mut reversed_num = 0;
    let mut original_num = num;
    while  original_num > 0 {
        reversed_num = reversed_num * 10 + original_num % 10;
        original_num /= 10;
    }
    return reversed_num;
}