use std::io;
use rand::Rng;

pub fn get_random_num() {
    let random_num = rand::thread_rng().gen_range(0..10);

    println!("type in any number between 0 and 10");
    let mut num_as_string = String::with_capacity(2);
    io::stdin()
        .read_line(&mut num_as_string)
        .expect("failed to read number from console");
    num_as_string.pop();  // same as truncate truncate(num_as_string.len() - 1)
    println!("your number is '{}', generated num is '{}'", num_as_string, random_num)
}