use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    data.lines().map(get_the_powaa).sum::<u32>()
}

fn get_the_powaa(game: &str) -> u32 {
    let mut mins = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for subgame in game.split(": ").nth(1).unwrap().split("; ") {
        for cube in subgame.split(", ") {
            let mut test = cube.split(' ');
            let count = test.next().unwrap().parse::<u32>().unwrap();
            let color = test.next().unwrap();
            if mins.get(color).unwrap() < &count {
                mins.insert(color, count);
            }
        }
    }
    mins.values().product::<u32>()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(2286, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(83707, solve_puzzle("../input"));
    }
}
