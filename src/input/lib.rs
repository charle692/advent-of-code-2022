use std::fs;

/// Read advent of code problem input
pub fn read_input(file_name: &str) -> String {
    let file_input = fs::read_to_string(file_name).expect("Unable to read file");

    return file_input;
}
