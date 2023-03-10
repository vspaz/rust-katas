use std::collections::HashMap;

pub fn count_letters(text: &str) -> HashMap<char, i32> {
    let mut letter_to_count: HashMap<char, i32> = HashMap::with_capacity(text.len());
    for letter in text.chars() {
        letter_to_count.entry(letter).or_insert(0);
        letter_to_count.insert(letter, letter_to_count[&letter] + 1);
    }
    letter_to_count
}

pub fn count_letters_2(text: &str) -> HashMap<char, i32> {
    let mut letter_to_count: HashMap<char, i32> = HashMap::with_capacity(text.len());
    for letter in text.chars() {
        letter_to_count
            .entry(letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    letter_to_count
}

#[cfg(test)]
mod tests {
    use crate::lettercounter;

    #[test]
    fn test_count_letters_ok() {
        let letter_to_count = lettercounter::count_letters("foobarbaz");
        assert_eq!(2, letter_to_count[&'o']);
        assert_eq!(2, letter_to_count[&'b']);
    }

    #[test]
    fn test_count_letters_2_ok() {
        let letter_to_count = lettercounter::count_letters_2("foobarbaz");
        assert_eq!(2, letter_to_count[&'o']);
        assert_eq!(2, letter_to_count[&'b']);
    }

    #[test]
    fn test_count_letters_equal() {
        let text = "foobarbaz";
        assert_eq!(
            lettercounter::count_letters(text),
            lettercounter::count_letters_2(text)
        );
    }
}
