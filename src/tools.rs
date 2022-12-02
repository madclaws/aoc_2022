use std::fs;
pub fn load_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Unable to load the file")
}