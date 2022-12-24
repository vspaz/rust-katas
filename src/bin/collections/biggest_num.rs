use core::panicking::panic;
use std::process::exit;

fn get_max_num_in_array(nums: &mut [i64]) -> i64 {
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