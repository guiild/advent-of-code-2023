use std::fs;

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut score: u32 = 0;
    for line in data.lines() {
        let card_data = line.split(": ").nth(1).unwrap();
        let mut values = card_data.split(" | ");
        let winnings = values
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let my_numbers = values
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut card_score: u32 = 0;
        for my_number in my_numbers {
            if winnings.contains(&my_number) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        score += card_score;
    }

    score
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(13, solve_puzzle("../test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(24706, solve_puzzle("../input"));
    }
}
