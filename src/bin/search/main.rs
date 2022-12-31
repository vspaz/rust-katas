mod binary_search;

fn main() {
    let nums: Vec<i64> = vec![-5, 0, 143, 800, 14800];
    let num_index = binary_search::bin_search(nums, 143);
}
