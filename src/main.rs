mod math;
use math::digitcounter::get_digit_count;

fn main() {
   assert_eq!(5, get_digit_count(10303));
}
