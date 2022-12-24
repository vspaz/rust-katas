mod arrays;

fn main() {
    let mut nums: Vec<i64> = vec![1, 2, 3, -10, 40, 100, -80, 50, 140];
    nums.insert(0, 10);
    assert_eq!(arrays::get_max(nums), 140);
}
