use crate::digitcounter::get_digit_count;
use crate::reversedigit::reverse_digit;

fn main() {
    assert_eq!(5, get_digit_count(10303));
    assert_eq!(54321, reverse_digit(12345));
}
