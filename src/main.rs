mod math;
use math::digitcounter::get_digit_count;
use math::reversedigit::reverse_digit;

mod stringops;
use stringops::lettercounter;

fn main() {
    assert_eq!(5, get_digit_count(10303));
    assert_eq!(54321, reverse_digit(12345));
    assert_eq!(2, lettercounter::count_letters("foo")[&'o']);
    assert_eq!(2, lettercounter::count_letters_2("foo")[&'o']);
}
