use std::fs;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(format!("Failed to read file: {}", file_path).as_str())
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord {
    pub i: i32,
    pub j: i32,
}

impl Coord {
    pub fn new(i: i32, j: i32) -> Self {
        Coord { i, j }
    }
}
