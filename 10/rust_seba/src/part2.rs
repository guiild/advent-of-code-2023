use crate::utils::read_data;
use core::panic;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let map = data.lines().collect::<Vec<&str>>();

    let animal = get_animal(&map);

    let mut main_loop: HashSet<(usize, usize)> = HashSet::new();
    build_loop(&map, animal.0, animal.1, &mut main_loop);

    let new_map = get_new_map(map, main_loop);

    let mut outside_points: HashSet<(usize, usize)> = HashSet::new();
    let mut inside_points: HashSet<(usize, usize)> = HashSet::new();

    (0..new_map.len())
        .flat_map(|i| (0..new_map[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| is_inside_ground(&new_map, i, j, &mut outside_points, &mut inside_points))
        .count() as u32
}

fn get_new_map(map: Vec<&str>, main_loop: HashSet<(usize, usize)>) -> Vec<String> {
    let mut new_map: Vec<String> = Vec::new();
    for i in 0..map.len() {
        let target_row = i * 2;
        new_map.push(String::new()); // Current line with extra columns
        for j in 0..map[i].len() {
            let current_value = if main_loop.contains(&(i, j)) {
                map[i].chars().nth(j).unwrap()
            } else {
                '.'
            };

            // Put current value
            new_map[target_row].push(current_value);

            // Insert new value to the right
            if j < map[i].len() {
                let right_value = map[i].chars().nth(j + 1).unwrap_or(' ');
                match current_value {
                    'F' | 'S' | '-' | 'L' => {
                        if ['7', 'J', '-', 'S'].contains(&right_value) {
                            new_map[target_row].push('-');
                        } else {
                            new_map[target_row].push(' ');
                        }
                    }
                    '|' | '.' | 'J' | '7' => {
                        new_map[target_row].push(' ');
                    }
                    _ => panic!("Unknown combination: {} - {}", current_value, right_value),
                }
            }

            if i == map.len() - 1 {
                continue;
            }
            new_map.push(String::new()); // Line with new values

            // Insert new value to the bottom
            let bottom_value = map[i + 1].chars().nth(j).unwrap_or(' ');
            match current_value {
                'F' | 'S' | '|' | '7' => {
                    if ['L', '|', 'J', 'S'].contains(&bottom_value) {
                        new_map[target_row + 1].push('|');
                        new_map[target_row + 1].push(' ');
                    } else {
                        new_map[target_row + 1].push(' ');
                        new_map[target_row + 1].push(' ');
                    }
                }
                '-' | '.' | 'L' | 'J' => {
                    new_map[target_row + 1].push(' ');
                    new_map[target_row + 1].push(' ');
                }
                _ => panic!("Unknown combination: {} - {}", current_value, bottom_value),
            }
        }
    }

    new_map
}

fn is_inside_ground(
    new_map: &[String],
    i: usize,
    j: usize,
    outside_points: &mut HashSet<(usize, usize)>,
    inside_points: &mut HashSet<(usize, usize)>,
) -> bool {
    let value = new_map[i].chars().nth(j).unwrap();

    if value != '.'
        || i == 0
        || i == new_map.len() - 1
        || j == 0
        || j == new_map[i].len() - 1
        || outside_points.contains(&(i, j))
        || can_reach_border(new_map, i, j, outside_points, inside_points)
    {
        return false;
    }

    true
}

fn can_reach_border(
    new_map: &[String],
    i: usize,
    j: usize,
    outside_points: &mut HashSet<(usize, usize)>,
    inside_points: &mut HashSet<(usize, usize)>,
) -> bool {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((i, j));

    loop {
        if stack.is_empty() {
            inside_points.extend(visited);

            return false;
        }

        let current = stack.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        let neighbors = get_empty_neighbors(new_map, current.0, current.1, &visited);
        for neighbor in neighbors {
            if inside_points.contains(&neighbor) {
                return false;
            }

            if outside_points.contains(&neighbor)
                || neighbor.0 == 0
                || neighbor.0 == new_map.len() - 1
                || neighbor.1 == 0
                || neighbor.1 == new_map[neighbor.0].len() - 1
            {
                outside_points.extend(visited);
                outside_points.insert(neighbor);
                return true;
            }
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
}

fn get_empty_neighbors(
    new_map: &[String],
    i: usize,
    j: usize,
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    Vec::from([(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)])
        .iter()
        .filter(|(i, j)| {
            i < &new_map.len()
                && j < &new_map[*i].len()
                && ['.', ' '].contains(&new_map[*i].chars().nth(*j).unwrap())
                && !visited.contains(&(*i, *j))
        })
        .map(|(i, j)| (*i, *j))
        .collect()
}

fn build_loop(map: &[&str], i: usize, j: usize, main_loop: &mut HashSet<(usize, usize)>) {
    let neighbors = get_neighbors(map, i, j);
    for neighbor in neighbors {
        if !main_loop.contains(&neighbor) {
            main_loop.insert(neighbor);
            build_loop(map, neighbor.0, neighbor.1, main_loop);
        }
    }
}

fn get_animal(map: &[&str]) -> (usize, usize) {
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No animal found");
}

fn get_neighbors(map: &[&str], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let current = map[i].chars().nth(j).unwrap();
    if i > 0 && ['S', 'L', 'J', '|'].contains(&current) {
        let top = map[i - 1].chars().nth(j).unwrap();
        if ['F', '|', '7', 'S'].contains(&top) {
            neighbors.push((i - 1, j));
        }
    }
    if j < map[i].len() - 1 && ['S', 'F', 'L', '-'].contains(&current) {
        let right = map[i].chars().nth(j + 1).unwrap();
        if ['J', '-', '7', 'S'].contains(&right) {
            neighbors.push((i, j + 1));
        }
    }
    if i < map.len() - 1 && ['S', 'F', '7', '|'].contains(&current) {
        let bottom = map[i + 1].chars().nth(j).unwrap();
        if ['L', '|', 'J', 'S'].contains(&bottom) {
            neighbors.push((i + 1, j));
        }
    }
    if j > 0 && ['S', '7', 'J', '-'].contains(&current) {
        let left = map[i].chars().nth(j - 1).unwrap();
        if ['F', '-', 'L', 'S'].contains(&left) {
            neighbors.push((i, j - 1));
        }
    }
    if neighbors.len() != 2 {
        panic!("There is only two directions in a pipe!");
    }
    neighbors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(10, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(413, solve_puzzle("input"));
    }
}

// Test initial map
// FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L

// test new map
//               11111111112222222222333333333
//     012345678901234567890123456789012345678
// 00  . F-7 F-S F-7 F-7 F-7 F-7 F-7 F-------7
// 01    | | | | | | | | | | | | | | |       |
// 02  . | L-J | | | | | | | | | | | | F-----J
// 03    |     | | | | | | | | | | | | |
// 04  . L---7 L-J L-J | | | | | | L-J L---7 .
// 05        |         | | | | | |         |
// 06  F-----J F-----7 | | L-J L-J . F-7 F-J .
// 07  |       |     | | |           | | |
// 08  L-------J F---J L-J . . . . F-J L-J . .
// 09            |                 |
// 10  . . . F---J F-------7 . . . L-7 . . . .
// 11        |     |       |         |
// 12  . . F-J F-7 L-7 F---J F-7 . . L-------7
// 13      |   | |   | |     | |             |
// 14  . . L---J L-7 | | F-7 | L-7 F---7 F-7 |
// 15              | | | | | |   | |   | | | |
// 16  . . . . . F-J | | | | | F-J L-7 | | L-J
// 17            |   | | | | | |     | | |
// 18  . . . . . L---J L-J L-J L-----J L-J . .
