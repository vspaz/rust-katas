use std::collections::HashMap;

pub fn get_occurrence(words: &Vec<&str>) -> HashMap<String, i32> {
    let mut word_to_count: HashMap<String, i32> = HashMap::new();
    for word in words {
        match word_to_count.get(*word) {
            Some(count) => word_to_count.insert(word.to_string(), count + 1),
            None => word_to_count.insert(word.to_string(), 1),
        };
    }
    word_to_count
}

#[cfg(test)]
mod tests {
    use crate::maps::get_occurrence;

    #[test]
    fn test_get_occurrence_ok() {
        let words = &vec!["foo", "foo", "bar", "baz", "bar", "barbaz"];
        let word_to_count = get_occurrence(words);
        assert_eq!(2, word_to_count["bar"]);
    }
}
