use crate::utils::read_data;
use itertools::Itertools;

pub fn solve_puzzle(file_name: &str) -> isize {
    let data = read_data(file_name);

    data.lines().fold(0, |mut total, line| {
        total += extra_value(
            line.split(' ')
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
        );
        total
    })
}

fn extra_value(numbers: Vec<isize>) -> isize {
    if numbers.iter().all(|x| *x == 0) {
        return 0;
    }
    let new_sequence = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<isize>>();

    numbers.first().unwrap() - extra_value(new_sequence)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(2, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(1097, solve_puzzle("input"));
    }
}
