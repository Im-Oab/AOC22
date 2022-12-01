use std::fs;

pub struct FileHandler {}

impl FileHandler {
    pub fn read(file_path: &str) -> String {
        let contents = fs::read_to_string(file_path).expect("");

        contents
    }
}
