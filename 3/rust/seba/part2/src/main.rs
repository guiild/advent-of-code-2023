use std::collections::HashSet;
use std::fs;

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u128 {
    let data = read_data(file_name);
    let referential = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // Mutable copy tf the table that we can dynamically modify
    // let mut table = referential.clone();
    let mut ratios = Vec::new();
    referential.iter().enumerate().for_each(|(i, row)| {
        for (j, c) in row.iter().enumerate() {
            if c != &'*' {
                continue;
            }
            let is_real_gear = check_gear(i, j, &referential);
            if is_real_gear {
                let gear_ratio = compute_gear_ratio(i, j, &referential);
                ratios.push(gear_ratio);
            }
        }
    });

    ratios.iter().sum()
}

fn compute_gear_ratio(i: usize, j: usize, referential: &Vec<Vec<char>>) -> u128 {
    // Reconstitute numbers next to the gear
    let mut table = referential.clone();
    for (ii, row) in referential.iter().enumerate() {
        for (jj, _c) in row.iter().enumerate() {
            if referential[ii][jj].is_ascii_digit() {
                let mut visited = HashSet::new();
                if !is_related_to_gear(i, j, ii, jj, referential, &mut visited) {
                    table[ii][jj] = '.';
                }
            }
        }
    }
    // Convert table to string
    let str_table = table
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("")
        .replace(|c: char| !c.is_ascii_digit(), " ");
    // Split string by space
    str_table
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u128>().unwrap())
        .product()
}

fn is_related_to_gear(
    i: usize,
    j: usize,
    ii: usize,
    jj: usize,
    referential: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> bool {
    if get_surrounding_full_cells(i, j, referential).contains(&(ii, jj)) {
        return true;
    }

    let neighbours = get_surrounding_full_cells(ii, jj, referential)
        .iter()
        .filter(|(i, j)| !visited.contains(&(*i, *j)))
        .cloned()
        .collect::<Vec<(usize, usize)>>();
    for neighbour in neighbours.iter() {
        visited.insert((ii, jj));
        if is_related_to_gear(i, j, neighbour.0, neighbour.1, referential, visited) {
            return true;
        }
    }

    false
}

fn check_gear(i: usize, j: usize, table: &Vec<Vec<char>>) -> bool {
    let neighbours = get_surrounding_full_cells(i, j, table)
        .iter()
        .filter(|(i, j)| table[*i][*j].is_ascii_digit())
        .cloned()
        .collect::<Vec<(usize, usize)>>();
    let mut fixed_neighbours = neighbours.clone();
    let mut removed: Vec<(usize, usize)> = Vec::new();

    for cell in neighbours.iter() {
        // continue if cell is in removed
        if removed.contains(cell) {
            continue;
        }
        for other_cell in neighbours.iter() {
            if same_number(cell, other_cell) {
                fixed_neighbours = fixed_neighbours
                    .iter()
                    .filter(|c| c != &other_cell)
                    .cloned()
                    .collect::<Vec<(usize, usize)>>();
                removed.push(*cell);
            }
        }
    }

    if fixed_neighbours.len() >= 2 {
        return true;
    }

    false
}

fn same_number(cell1: &(usize, usize), cell2: &(usize, usize)) -> bool {
    cell1.0 == cell2.0 && cell1.1 as i128 - cell2.1 as i128 == 1
}

fn get_surrounding_full_cells(i: usize, j: usize, table: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut all_neigbors = Vec::new();
    if i > 0 {
        all_neigbors.push((i - 1, j));
    }
    if i < table.len() - 1 {
        all_neigbors.push((i + 1, j));
    }
    if j > 0 {
        all_neigbors.push((i, j - 1));
    }
    if j < table[0].len() - 1 {
        all_neigbors.push((i, j + 1));
    }
    if i > 0 && j > 0 {
        all_neigbors.push((i - 1, j - 1));
    }
    if i > 0 && j < table[0].len() - 1 {
        all_neigbors.push((i - 1, j + 1));
    }
    if i < table.len() - 1 && j > 0 {
        all_neigbors.push((i + 1, j - 1));
    }
    if i < table.len() - 1 && j < table[0].len() - 1 {
        all_neigbors.push((i + 1, j + 1));
    }

    all_neigbors
        .into_iter()
        .filter(|(i, j)| table[*i][*j] != '.')
        .collect::<Vec<(usize, usize)>>()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(467835, solve_puzzle("../test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(79844424, solve_puzzle("../input"));
    }
}
