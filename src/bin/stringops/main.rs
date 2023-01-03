mod lettercounter;


fn main() {
    assert_eq!(2, lettercounter::count_letters("foo")[&'o']);
    assert_eq!(2, lettercounter::count_letters_2("foo")[&'o']);
}
