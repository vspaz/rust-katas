mod arrays;

fn main() {
    let mut nums: Vec<i64> = vec![1, 2, 3, -10, 40, 100, -80, 50, 140];
    nums.insert(0, 10);
    assert_eq!(arrays::get_max(nums), 140);
    assert_eq!(10, arrays::get_total(vec![2, 2, 2, 2, 2]));
    assert_eq!(3.0, arrays::get_average(vec![2, 3, 4]));
    let mut nums = [5, 4, 3, 2, 1];
    arrays::reverse_array_in_place(&mut nums);
    assert_eq!([1, 2, 3, 4, 5], nums);
    let nums = vec![0, 10, 20, 30, 40, -10];
    assert_eq!(vec![10, 20, 30, 40, 50, 0], arrays::increment_by(10, nums));
}
