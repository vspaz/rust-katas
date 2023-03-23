use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn compare_nums() {
    let mut guess = String::with_capacity(2);
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read input");

    let guess: u32 = guess.trim().parse().expect("please type in a number");
    let random_num = rand::thread_rng().gen_range(0..=10);

    match guess.cmp(&random_num) {
        Ordering::Less => println!("{} < {}", guess, random_num),
        Ordering::Greater => println!("{} > {}", guess, random_num),
        Ordering::Equal => println!("{} == {}", guess, random_num),
    }
}
