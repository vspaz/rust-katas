pub fn get_digit_count(num: u32) -> u32 {
    let mut count: u32 = 0;
    let mut remainder = num;
    while remainder > 0 {
        remainder /= 10;
        count += 1;
    }
    return count;
}