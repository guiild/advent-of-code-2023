use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);
    let mut lines = data.lines();
    let time = read_next_line(&mut lines);
    let distance = read_next_line(&mut lines);

    let first = (0..time + 1)
        .find(|push_time| {
            let travelled = (time - push_time) * push_time;
            travelled > distance
        })
        .unwrap();
    let last = time - first;
    last - first + 1
}

fn read_next_line(lines: &mut std::str::Lines) -> usize {
    lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(71503, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(35961505, solve_puzzle("input"));
    }
}
