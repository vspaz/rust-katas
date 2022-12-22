use std::collections::HashMap;

pub fn count_letters(text: String) -> HashMap<&str, i32> {
    let mut letter_to_count = HashMap::with_capacity(text.len());
    for letter in text.chars() {
        if !letter_to_count.contains_key(letter) {
            letter_to_count.insert(letter, 0)
        }
        letter_to_count.insert(letter, letter_to_count.get(letter) + 1);
    }
    return letter_to_count
}