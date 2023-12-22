use crate::utils::read_data;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = fix_data(read_data(file_name));
    data.lines()
        .map(|line| {
            let first_number = line.chars().find(|x| x.is_ascii_digit()).unwrap();
            let last_number = line.chars().rfind(|x| x.is_ascii_digit()).unwrap();
            format!("{}{}", first_number, last_number)
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn fix_data(data: String) -> String {
    // This is awkward...
    // How to handle better the "eightwo" case?
    HashMap::from([
        ("one", "o1ne"),
        ("two", "tw2o"),
        ("three", "th3ree"),
        ("four", "fo4ur"),
        ("five", "fi5ve"),
        ("six", "s6ix"),
        ("seven", "sev7en"),
        ("eight", "ei8ght"),
        ("nine", "ni9ne"),
    ])
    .iter()
    .fold(data, |acc, (key, value)| acc.replace(key, value))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(281, solve_puzzle("test_data2"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(54208, solve_puzzle("input"));
    }
}
