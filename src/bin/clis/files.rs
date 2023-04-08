use std::fs;

pub fn from_file(file_name: String) -> std::io::Result<String> {
    return  fs::read_to_string(file_name).expect("failed to read from file");
}