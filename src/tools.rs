use std::fs;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(format!("Failed to read file: {}", file_path).as_str())
}
