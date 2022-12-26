mod arrays;

fn main() {
    let mut nums: Vec<i64> = vec![1, 2, 3, -10, 40, 100, -80, 50, 140];
    nums.insert(0, 10);
    assert_eq!(arrays::get_max(nums), 140);
    assert_eq!(10, arrays::get_total(vec![2, 2, 2, 2, 2]));
    assert_eq!(3.0, arrays::get_average(vec![2, 3, 4]))
}
