use std::io;

fn main() {
    println!("pls, type in a word!");
    let mut word = String::with_capacity(32);
    io::stdin()
        .read_line(&mut word)
        .expect("failed to read the word");
    println!("{}", word)
}
