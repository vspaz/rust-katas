pub fn sort_in_place<T: Ord>(nums: &mut [T]) {
    for i in 0..len() {
        for j in 0..nums.len() - 1 - i {
            if nums[j] > nums[j + 1] {
            let temp = nums[j];
            nums[j] = nums[j+1];
            nums[j + 1] = temp;
            }
        }
    }
}