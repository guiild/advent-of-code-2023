use crate::utils::read_data;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = (0_usize, map[0].iter().position(|c| c == &'.').unwrap());
    let exit = (
        map.len() - 1,
        map[map.len() - 1].iter().position(|c| c == &'.').unwrap(),
    );

    let mut visited = HashMap::new();
    visited.insert(start, 0_usize);

    let mut result = 0;

    let mut stack = Vec::new();
    stack.push(vec![start]);
    while let Some(path) = stack.pop() {
        let next_paths = get_next_paths(&map, &path);
        for next_path in next_paths {
            if next_path[next_path.len() - 1] == exit {
                result = result.max(next_path.len());
                continue;
            }
            if let Some(current_max_for_cell) = visited.get(&next_path[next_path.len() - 1]) {
                if &next_path.len() < current_max_for_cell {
                    continue;
                }
            }
            visited.insert(next_path[next_path.len() - 1], next_path.len());
            stack.push(next_path);
        }
    }

    result - 1
}

fn get_next_paths(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let (i, j) = path[path.len() - 1];
    let mut next_paths = Vec::new();
    if i > 0 {
        let next_position = (i - 1, j);
        if !path.contains(&next_position) {
            let next_value = map[i - 1][j];
            if next_value == '.' || next_value == '^' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }
    if i < map.len() - 1 {
        let next_position = (i + 1, j);
        if !path.contains(&next_position) {
            let next_value = map[i + 1][j];
            if next_value == '.' || next_value == 'v' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }
    if j > 0 {
        let next_position = (i, j - 1);
        if !path.contains(&next_position) {
            let next_value = map[i][j - 1];
            if next_value == '.' || next_value == '<' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }
    if j < map[0].len() - 1 {
        let next_position = (i, j + 1);
        if !path.contains(&next_position) {
            let next_value = map[i][j + 1];
            if next_value == '.' || next_value == '>' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }

    next_paths
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(94, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(2050, solve_puzzle("input"));
    }
}
