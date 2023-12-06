use std::fs;

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut lines = data.lines();
    let times = read_next_line(&mut lines);
    let distances = read_next_line(&mut lines);

    times
        .iter()
        .enumerate()
        .map(|(i, race_time)| {
            let first = (0..race_time + 1)
                .find(|push_time| {
                    let travelled = (race_time - push_time) * push_time;
                    travelled > distances[i]
                })
                .unwrap();
            let last = race_time - first;
            last - first + 1
        })
        .product()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

fn read_next_line(lines: &mut std::str::Lines) -> Vec<u32> {
    lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(288, solve_puzzle("../test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(1155175, solve_puzzle("../input"));
    }
}
