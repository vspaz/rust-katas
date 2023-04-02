use std::process::exit;

pub fn get_max(nums: Vec<i64>) -> i64 {
    if nums.is_empty() {
        eprintln!("size can't be 0");
        exit(-1);
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let mut max_num = nums[0];
    for num in nums.iter().skip(1) {
        if *num > max_num {
            max_num = *num;
        }
    }
    max_num
}

pub fn get_total(nums: Vec<i64>) -> i64 {
    let mut total = 0;
    for num in nums.iter() {
        total += num;
    }
    total
}

pub fn get_average(nums: Vec<i64>) -> f64 {
    let mut total = 0;
    for num in nums.iter() {
        total += num
    }
    total as f64 / nums.len() as f64
}

pub fn reverse_array_in_place<T: Ord>(nums: &mut [T]) {
    let mut start = 0;
    let mut end = nums.len() - 1;
    while start < end {
        nums.swap(start, end);
        start += 1;
        end -= 1;
    }
}

pub fn increment_by(value: i32, mut nums: Vec<i32>) -> Vec<i32> {
    for num in &mut nums {
        *num += value;
    }
    nums
}

#[cfg(test)]
mod tests {
    use crate::arrays::{get_average, get_max, get_total, increment_by, reverse_array_in_place};

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

    #[test]
    fn test_get_total_ok() {
        assert_eq!(10, get_total(vec![2, 2, 2, 2, 2]));
    }

    #[test]
    fn test_get_total_one_element_ok() {
        assert_eq!(2, get_total(vec![2]));
    }

    #[test]
    fn test_get_average_ok() {
        assert_eq!(1.0, get_average(vec![1, 2, 0, 0, 2]))
    }

    #[test]
    fn test_reverse_ok() {
        let mut nums = [5, 4, 3, 2, 1];
        reverse_array_in_place(&mut nums);
        assert_eq!([1, 2, 3, 4, 5], nums);
    }

    #[test]
    fn test_reverse_idiomatic_ok() {
        let mut nums = [5, 4, 3, 2, 1];
        let mut nums_2 = nums.clone();
        reverse_array_in_place(&mut nums);
        nums_2.reverse();
        assert_eq!([1, 2, 3, 4, 5], nums_2);
    }

    #[test]
    fn test_increment_by_ok() {
        let nums = vec![0, 10, 20, 30, 40, -10];
        assert_eq!(vec![10, 20, 30, 40, 50, 0], increment_by(10, nums));
    }
}
