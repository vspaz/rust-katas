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

pub fn reverse_array_in_place(nums: &mut [i32; 5]) {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut temp;
    while start < end {
        temp = nums[start];
        nums[start] = nums[end];
        nums[end] = temp;
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::{get_average, get_max, get_total, reverse_array_in_place};

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
}
