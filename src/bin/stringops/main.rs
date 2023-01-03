mod lettercounter;
mod reverse;

fn main() {
    assert_eq!(2, lettercounter::count_letters("foo")[&'o']);
    assert_eq!(2, lettercounter::count_letters_2("foo")[&'o']);
    assert_eq!("12345", reverse::reverse_acii_string("54321"));
}
