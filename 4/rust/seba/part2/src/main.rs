use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut cards_count = HashMap::new();

    for (i, line) in data.lines().enumerate() {
        let card_number = (i + 1) as u32;

        let current_value = cards_count.get(&card_number).cloned().unwrap_or(0_u32);
        cards_count.insert(card_number, current_value + 1);

        let card_data = line.split(": ").nth(1).unwrap();
        let mut values = card_data.split(" | ");
        let wins = values
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let numbers = values
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let matching_count = numbers.iter().filter(|x| wins.contains(x)).count();
        for n in card_number + 1..card_number + matching_count as u32 + 1 {
            let current_value = cards_count.get(&n).cloned().unwrap_or(0_u32);
            let increment = cards_count.get(&card_number).unwrap();
            cards_count.insert(n, current_value + increment);
        }
    }

    cards_count.values().sum()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(30, solve_puzzle("../test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(13114317, solve_puzzle("../input"));
    }
}
