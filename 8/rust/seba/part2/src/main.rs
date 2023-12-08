use num::integer::lcm;
use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> usize {
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
    let mut positions = maps.keys().filter(|x| x.ends_with('A')).collect::<Vec<_>>();

    let mut cycles: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..positions.len() {
        cycles.insert(i, vec![]);
    }

    for (step, direction) in directions.chars().cycle().enumerate() {
        positions = positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                let (left, right) = maps.get(*position).unwrap();
                let next_position = if direction == 'L' { left } else { right };
                if next_position.ends_with('Z') {
                    cycles.get_mut(&index).unwrap().push(step);
                }
                next_position
            })
            .collect::<Vec<&String>>();

        if cycles.values().all(|x| x.len() >= 2) {
            break;
        }
    }

    cycles
        .values()
        .map(|x| x[1] - x[0])
        .fold(0, |acc, x| if acc == 0 { x } else { lcm(acc, x) })
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(6, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(13385272668829, solve_puzzle("../input"));
    }
}
