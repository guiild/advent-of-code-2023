use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let totals = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    data.lines()
        .enumerate()
        .map(|(i, game)| index_if_game_possible(i, game, &totals))
        .sum::<u32>()
}

fn index_if_game_possible(i: usize, game: &str, totals: &HashMap<&str, u32>) -> u32 {
    for subgame in game.split(": ").nth(1).unwrap().split("; ") {
        for cube in subgame.split(", ") {
            let mut test = cube.split(' ');
            let count = test.next().unwrap().parse::<u32>().unwrap();
            let color = test.next().unwrap();
            if totals.get(color).unwrap() < &count {
                return 0;
            }
        }
    }
    i as u32 + 1
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(8, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(2685, solve_puzzle("../input"));
    }
}
