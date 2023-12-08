use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut lines = data.lines();
    let directions = lines.next().unwrap();
    lines.next();

    let mut maps = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let fixed_line = line.replace(['(', ')'], "");
        let (source, destinations) = fixed_line.split_once(" = ").unwrap();
        let (dest_left, dest_right) = destinations.split_once(", ").unwrap();
        maps.insert(
            source.to_string(),
            (dest_left.to_string(), dest_right.to_string()),
        );
    }

    let mut position = "AAA";

    for (step, direction) in directions.chars().cycle().enumerate() {
        let (left, right) = maps.get(position).unwrap();
        if direction == 'L' {
            position = left;
        } else {
            position = right;
        }
        if position == "ZZZ" {
            return step as u32 + 1;
        }
    }

    0
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(2, solve_puzzle("test_data"));
    }

    #[test]
    fn test_example_data_2() {
        assert_eq!(6, solve_puzzle("test_data2"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(12083, solve_puzzle("../input"));
    }
}
