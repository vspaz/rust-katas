use crate::lettercounter::count_letters;
use crate::lettercounter::count_letters_2;

fn main() {
    assert_eq!(2, count_letters("foo")[&'o']);
    assert_eq!(2, count_letters_2("foo")[&'o']);
}
