use std::fs;

pub fn read_data(file_name: &str) -> String {
  fs::read_to_string(file_name).expect("Error")
}