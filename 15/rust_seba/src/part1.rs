use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    data.split(',').map(hash).sum()
}

fn hash(input: &str) -> u32 {
    let mut result = 0;
    for c in input.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(1320, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(506869, solve_puzzle("input"));
    }
}
