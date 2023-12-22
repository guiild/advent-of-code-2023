use crate::utils::read_data;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    // Create Universe
    let mut universe: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        universe.push(line.chars().collect());
        if !line.contains('#') {
            universe.push((0..line.len()).map(|_| '.').collect());
        }
    }
    let mut empty_cols = Vec::new();
    for j in 0..universe[0].len() {
        let column = universe.iter().map(|row| row[j]).collect::<String>();
        if !column.contains('#') {
            empty_cols.push(j);
        }
    }
    for j in empty_cols.iter().rev() {
        for row in universe.iter_mut() {
            row.insert(*j, '.');
        }
    }
    // PRINT MAP
    // for row in universe.iter_mut() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // Get galaxies
    let mut galaxies = HashSet::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value == &'#' {
                galaxies.insert((i, j));
            }
        }
    }

    let pairs = galaxies.iter().combinations(2).collect::<Vec<_>>();

    pairs.iter().fold(0, |mut acc, p| {
        acc += distance(p[0], p[1]);
        acc
    })
}

fn distance(p1: &(usize, usize), p2: &(usize, usize)) -> u32 {
    let x1 = p1.0 as i32;
    let y1 = p1.1 as i32;
    let x2 = p2.0 as i32;
    let y2 = p2.1 as i32;
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(374, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(9370588, solve_puzzle("input"));
    }
}
