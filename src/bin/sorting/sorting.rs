pub fn bubble_sort<T: Ord>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() - (1 + i) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting;

    #[test]
    fn test_bubble_sort_ok() {
        let mut nums = [5, 4, 3, 2, 1];
        sorting::bubble_sort(&mut nums);
        assert_eq!([1, 2, 3, 4, 5], nums);
    }
}
