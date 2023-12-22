use std::collections::HashMap;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);
    let mut visited = HashMap::new();

    let mut plateform: Vec<Vec<char>> = data
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut copy = plateform.clone();
    let mut cycle_length = 0;
    let mut cycle_start = 0;

    for i in 0..1000000000 {
        copy = run_cycle(copy.clone());
        let cycle_number = i;
        if visited.contains_key(&copy) {
            cycle_length = cycle_number - visited.get(&copy).unwrap();
            cycle_start = *visited.get(&copy).unwrap();
            break;
        }
        visited.insert(copy.clone(), i);
    }
    for _ in 0..1000000000 % cycle_length + (cycle_start / cycle_length + 1) * cycle_length {
        plateform = run_cycle(plateform.clone());
    }

    plateform
        .iter()
        .rev()
        .enumerate()
        .map(|(y, line)| line.iter().filter(|c| c == &&'O').count() * (y + 1))
        .sum()
}

fn run_cycle(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let tilted_north = tilt_north(plateform);
    let tilted_west = titl_west(tilted_north);
    let tilted_south = tilt_south(tilted_west);
    tilt_east(tilted_south)
}

fn tilt_north(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    tilt(plateform)
}

fn tilt_south(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let reversed: Vec<Vec<char>> = plateform.iter().rev().map(|row| row.to_vec()).collect();
    let tilted = tilt(reversed);
    let reversed_back: Vec<Vec<char>> = tilted.iter().rev().map(|row| row.to_vec()).collect();
    reversed_back
}

fn rotate_right(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows_count = plateform.len();
    let cols_count = plateform[0].len();
    let mut rotated = vec![vec!['.'; cols_count]; rows_count];
    for (i, row) in plateform.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            rotated[j][plateform[0].len() - i - 1] = *char;
        }
    }
    rotated
}

fn rotate_left(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows_count = plateform.len();
    let cols_count = plateform[0].len();
    let mut rotated = vec![vec!['.'; cols_count]; rows_count];
    for (i, row) in plateform.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            rotated[plateform.len() - j - 1][i] = *char;
        }
    }
    rotated
}

fn titl_west(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rotated = rotate_right(plateform);
    let tilted = tilt(rotated);
    rotate_left(tilted)
}

fn tilt_east(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rotated = rotate_left(plateform);
    let tilted = tilt(rotated);
    rotate_right(tilted)
}

fn tilt(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    //  Vec of same dimensions with only points
    let rows_count = plateform.len();
    let cols_count = plateform[0].len();
    let mut tilted_north = vec![vec!['.'; cols_count]; rows_count];

    for (i, row) in plateform.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'O' {
                let new_position = get_new_position(&plateform, i, j);
                tilted_north[new_position.0][new_position.1] = 'O';
            } else if char == &'#' {
                tilted_north[i][j] = '#';
            }
        }
    }
    tilted_north
}

fn get_new_position(plateform: &[Vec<char>], i: usize, j: usize) -> (usize, usize) {
    let current_col = plateform.iter().map(|row| row[j]).collect::<Vec<char>>();
    let closest_cube = current_col[0..i]
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'#')
        .map(|(ix, _)| ix)
        .max();
    match closest_cube {
        Some(pos) => {
            let rounded_below = current_col[pos..i].iter().filter(|c| c == &&'O').count();
            (pos + rounded_below + 1, j)
        }
        None => {
            let rounded_below = current_col[0..i].iter().filter(|c| c == &&'O').count();
            (rounded_below, j)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(64, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(105008, solve_puzzle("input"));
    }
}
