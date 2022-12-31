pub fn bin_search(nums: Vec<i64>, num: i64) -> Option<i64> {
    let mut lower_bound = 0;
    let mut upper_bound = nums.len() - 1;

    while lower_bound <= upper_bound {
        mid = (lower_bound + upper_bound) / 2;
        let guess = nums[mid];
        if guess == num {
            return Some(mid);
        }
        if guess > num {
            upper_bound = mid - 1;
        } else {
            lower_bound = mid + 1;
        }
    }
    return None;
}