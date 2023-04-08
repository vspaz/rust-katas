pub fn search(query: &str, contents: &str) -> Vec<String> {
    let mut found_matches: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found_matches.push(line.to_string())
        }
    }
    found_matches
}
