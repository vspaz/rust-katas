mod digitcounter;
mod factorial;
mod fibonacci;
mod reversedigit;
mod square;

fn main() {
    assert_eq!(5, digitcounter::get_digit_count(10303));
    assert_eq!(54321, reversedigit::reverse_digit(12345));
    assert_eq!(120, factorial::get_factorial(5));
    assert_eq!(55, fibonacci::fib(10));
    assert_eq!(20, square::Rectangle::new(5, 4).get_area());
}
