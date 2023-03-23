use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn compare_nums() {

    let random_num = rand::thread_rng().gen_range(0..=10);

    loop {
        let mut num_as_text = String::with_capacity(2);
        println!("Please type in a number");
        io::stdin()
            .read_line(&mut num_as_text)
            .expect("failed to read input");
        let num: u32 = match num_as_text.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match num.cmp(&random_num) {
            Ordering::Less => println!("{} is less", num),
            Ordering::Greater => println!("{} is greater", num),
            Ordering::Equal => {
                println!("{} is equal", num);
                break;
            }
        }
    }
}
