mod binary_search;

fn main() {
    let nums: Vec<i64> = vec![-5, 0, 143, 800, 14800];
    let num_to_look_for = 143;
    let num_index = binary_search::bin_search(nums, num_to_look_for);

    match num_index {
        Some(idx) => println!("num {} found at index {}", num_to_look_for, idx),
        None => println!("num {} not found", num_to_look_for ),
    }
}
