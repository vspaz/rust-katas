use std::collections::HashMap;

pub fn get_occurrence(words: &Vec<String>) -> HashMap<String, i32> {
    let mut word_to_count: HashMap<String, i32> = HashMap::new();
    for word in words {
         match word_to_count.get(word)  {
             Some(count) => word_to_count[word] += 1,
             None => word_to_count[word] = 1
         }
    }
    word_to_count
}