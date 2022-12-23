mod digitcounter;
mod reversedigit;

fn main() {
    assert_eq!(5, digitcounter::get_digit_count(10303));
    assert_eq!(54321, reversedigit::reverse_digit(12345));
}
