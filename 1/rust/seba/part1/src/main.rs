use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
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

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(142, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(54940, solve_puzzle("input"));
    }
}
