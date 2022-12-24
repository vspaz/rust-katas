use std::process::exit;

fn get_max(nums: Vec<i64>) -> i64 {
    if nums.len() == 0 {
        exit(-1);
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let mut max_num = nums[0];
    for i in 1..nums.len() {
        if nums[i] > max_num {
            max_num = nums[i];
        }
    }
    max_num
}

#[cfg(test)]
mod tests {
    use crate::arrays::get_max;

    #[test]
    fn test_get_max_with_size_one_ok() {
        let mut nums: Vec<i64> = Vec::new();
        nums.insert(0, 10);
        assert_eq!(get_max(nums), 10);
    }

    #[test]
    fn test_get_max_ok() {
        let mut nums: Vec<i64> = vec![1, 2, 3, -10, 40, 100, -80, 50, 140];
        nums.insert(0, 10);
        assert_eq!(get_max(nums), 140);
    }
}
