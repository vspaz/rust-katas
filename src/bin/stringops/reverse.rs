pub fn reverse_acii_string(text: &str) -> String {
    let mut chars = text.as_bytes().to_owned();
    let mut start = 0;
    let mut end = chars.len() - 1;
    while start < end {
        chars.swap(start, end);
        start += 1;
        end -= 1;
    }
    std::str::from_utf8(&chars).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::reverse::reverse_acii_string;

    #[test]
    fn reverse_ascii_string_ok() {
        assert_eq!("abcde", reverse_acii_string("edcba"));
    }

    #[test]
    fn reverse_ascii_string_ok_2() {
        assert_eq!("abcde".chars().rev().collect::<String>(), reverse_acii_string("abcde"));
    }
}