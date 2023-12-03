use std::collections::VecDeque;
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
    let mut number_cells: Vec<VecDeque<(usize, usize)>> = Vec::new();
    let numeric_neighbours = get_surrounding_full_cells(i, j, referential)
        .iter()
        .filter(|(i, j)| referential[*i][*j].is_ascii_digit())
        .cloned()
        .collect::<Vec<(usize, usize)>>();

    for neighbour in numeric_neighbours.iter() {
        let number_vec = reconstitute_number(neighbour, referential);
        if !number_cells.contains(&number_vec) {
            number_cells.push(number_vec);
        }
    }

    number_cells
        .iter()
        .map(|d| {
            d.iter()
                .map(|cell| referential[cell.0][cell.1])
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .product()
}

fn reconstitute_number(
    cell: &(usize, usize),
    referential: &[Vec<char>],
) -> VecDeque<(usize, usize)> {
    let mut number_vec: VecDeque<(usize, usize)> = VecDeque::from([cell.to_owned()]);
    let mut counter = 0;
    loop {
        if cell.1 + counter > referential[0].len() - 1 {
            break;
        }
        let next_right = referential[cell.0][cell.1 + counter];
        if next_right.is_ascii_digit() {
            if !number_vec.contains(&(cell.0, cell.1 + counter)) {
                number_vec.push_back((cell.0, cell.1 + counter));
            }
            counter += 1;
        } else {
            break;
        }
    }
    counter = 0;
    loop {
        if counter > cell.1 {
            break;
        }
        let next_left = referential[cell.0][cell.1 - counter];
        if next_left.is_ascii_digit() {
            if !number_vec.contains(&(cell.0, cell.1 - counter)) {
                number_vec.push_front((cell.0, cell.1 - counter));
            }
            counter += 1;
        } else {
            break;
        }
    }
    number_vec
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
