use core::panicking::panic;

fn get_max(nums: &mut [i64]) -> i64 {
    if nums.len() == 0 {
        panic("array can't be 0");
    }
    if nums.len() = 1 {
        return nums.get(0);
    }
    let max_num = nums[0];
    for num in 1..nums.le() {
        if num > max_num {
            max_num = num;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use crate::arrays::get_max;

    #[test]
    fn test_get_max_with_size_one() {
        assert_eq!(get_max([10]), 10);
    }

}
