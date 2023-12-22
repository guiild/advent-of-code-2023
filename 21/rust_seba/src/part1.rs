use std::collections::HashSet;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str, steps: u32) -> u32 {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start: (usize, usize) = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(
                |(x, cell)| {
                    if *cell == 'S' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .unwrap();

    let mut starts: HashSet<(usize, usize)> = HashSet::from([start]);

    for _ in 0..steps {
        let mut next_starts = HashSet::new();
        for start in &starts {
            let next_cells = get_next_cells(&map, *start);
            next_starts.extend(next_cells);
        }
        starts = next_starts;
    }

    starts.len() as u32
}

fn get_next_cells(map: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut next_cells: Vec<(usize, usize)> = Vec::new();

    let (i, j) = start;

    if i > 0 && map[i - 1][j] != '#' {
        next_cells.push((i - 1, j));
    }
    if i < map.len() - 1 && map[i + 1][j] != '#' {
        next_cells.push((i + 1, j));
    }
    if j > 0 && map[i][j - 1] != '#' {
        next_cells.push((i, j - 1));
    }
    if j < map.len() - 1 && map[i][j + 1] != '#' {
        next_cells.push((i, j + 1));
    }

    next_cells
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(16, solve_puzzle("test_data", 6));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(3658, solve_puzzle("input", 64));
    }
}
