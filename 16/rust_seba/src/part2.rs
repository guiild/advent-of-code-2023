use crate::utils::read_data;
use std::{collections::HashSet, vec};

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let contraption = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut entries = Vec::new();
    for i in 1..contraption.len() - 1 {
        entries.push((i, 0, '>'));
        entries.push((i, contraption[0].len() - 1, '<'));
    }
    for j in 1..contraption[0].len() - 1 {
        entries.push((0, j, 'v'));
        entries.push((contraption.len() - 1, j, '^'));
    }
    // Angles
    ['>', 'v']
        .iter()
        .for_each(|entry| entries.push((0, 0, *entry)));
    ['<', 'v']
        .iter()
        .for_each(|entry| entries.push((0, contraption[0].len() - 1, *entry)));
    ['>', '^']
        .iter()
        .for_each(|entry| entries.push((contraption.len() - 1, 0, *entry)));
    ['<', '^']
        .iter()
        .for_each(|entry| entries.push((contraption.len() - 1, contraption[0].len() - 1, *entry)));

    let mut result = 0;
    for entry in entries {
        let mut energized: HashSet<(usize, usize)> = HashSet::new();
        let mut visited = HashSet::new();
        let mut beams: Vec<(usize, usize, char)> = Vec::new(); // Next pos row,  next pos col, next pos direction
        beams.push(entry);
        visited.insert(entry);

        while let Some(beam) = beams.pop() {
            energized.insert((beam.0, beam.1));
            let next_cells = get_next_cells(beam.0, beam.1, beam.2, &contraption);
            for next_cell in next_cells {
                if visited.contains(&next_cell) {
                    continue;
                }

                visited.insert(next_cell);
                beams.push(next_cell);
            }
        }

        result = result.max(energized.len() as u32);
    }
    result
}

fn get_next_cells(
    i: usize,
    j: usize,
    entry_direction: char,
    contraption: &Vec<Vec<char>>,
) -> Vec<(usize, usize, char)> {
    let current_cell = contraption[i][j];
    let exit_directions = get_exit_directions(current_cell, entry_direction);
    let mut next_cells: Vec<(usize, usize, char)> = Vec::new();
    for exit_direction in exit_directions {
        if let Some(next_cell) = get_next_cell(i, j, exit_direction, contraption) {
            next_cells.push(next_cell);
        }
    }
    next_cells
}

fn get_next_cell(
    i: usize,
    j: usize,
    exit_direction: char,
    contraption: &Vec<Vec<char>>,
) -> Option<(usize, usize, char)> {
    match exit_direction {
        '>' => {
            if j < contraption[i].len() - 1 {
                Some((i, j + 1, exit_direction))
            } else {
                None
            }
        }
        '<' => {
            if j > 0 {
                Some((i, j - 1, exit_direction))
            } else {
                None
            }
        }
        '^' => {
            if i > 0 {
                Some((i - 1, j, exit_direction))
            } else {
                None
            }
        }
        'v' => {
            if i < contraption.len() - 1 {
                Some((i + 1, j, exit_direction))
            } else {
                None
            }
        }
        _ => panic!("Invalid exit_direction: {}", exit_direction),
    }
}

fn get_exit_directions(current_cell: char, entry_direction: char) -> Vec<char> {
    match entry_direction {
        '>' => match current_cell {
            '/' => vec!['^'],
            '\\' => vec!['v'],
            '|' => vec!['^', 'v'],
            _ => vec!['>'],
        },
        '<' => match current_cell {
            '/' => vec!['v'],
            '\\' => vec!['^'],
            '|' => vec!['^', 'v'],
            _ => vec!['<'],
        },
        '^' => match current_cell {
            '/' => vec!['>'],
            '\\' => vec!['<'],
            '-' => vec!['<', '>'],
            _ => vec!['^'],
        },
        'v' => match current_cell {
            '/' => vec!['<'],
            '\\' => vec!['>'],
            '-' => vec!['<', '>'],
            _ => vec!['v'],
        },
        _ => panic!("Invalid entry_direction: {}", entry_direction),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(51, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(8148, solve_puzzle("input"));
    }
}
