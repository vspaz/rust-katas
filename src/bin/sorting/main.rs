mod sorting;

fn main() {
    let mut nums = [5, 4, 3, 2, 1];
    sorting::bubble_sort(&mut nums);
    assert_eq!([1, 2, 3, 4, 5], nums);
}
