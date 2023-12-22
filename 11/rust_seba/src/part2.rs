use crate::utils::read_data;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    // Create Universe
    let mut universe: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        if !line.contains('#') {
            universe.push((0..line.len()).map(|_| '*').collect());
        } else {
            universe.push(line.chars().collect());
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
            row[*j] = '*';
        }
    }

    for row in universe.iter_mut() {
        println!("{}", row.iter().collect::<String>());
    }

    // Get galaxies
    let mut galaxies = HashSet::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value == &'#' {
                galaxies.insert((i, j));
            }
        }
    }

    // PRINT MAP
    // for galaxy in galaxies.iter() {
    //     println!("{:?}", galaxy);
    // }

    //
    let pairs = galaxies.iter().combinations(2).collect::<Vec<_>>();

    pairs.iter().fold(0, |mut acc, p| {
        acc += distance(p[0], p[1], &universe);
        acc
    })
}

fn distance(p1: &(usize, usize), p2: &(usize, usize), universe: &[Vec<char>]) -> usize {
    let mut distance = 0;
    let start_v = p1.0.min(p2.0);
    let end_v = p1.0.max(p2.0);
    for row in universe.iter().take(end_v).skip(start_v) {
        if row[p1.1] == '*' {
            distance += 1000000;
        } else {
            distance += 1;
        }
    }

    let start_h = p1.1.min(p2.1);
    let end_h = p1.1.max(p2.1);
    for j in start_h..end_h {
        if universe[p1.0][j] == '*' {
            distance += 1000000;
        } else {
            distance += 1;
        }
    }

    distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(746207878188, solve_puzzle("input"));
    }
}
