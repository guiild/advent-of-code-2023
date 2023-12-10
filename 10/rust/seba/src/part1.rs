use crate::utils::read_data;
use core::panic;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let map = data.lines().collect::<Vec<&str>>();

    let animal = get_animal(&map);
    let mut neighbors = get_neighbors(&map, animal.0, animal.1);
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    distances.insert(animal, 0);

    update_neighbors(&map, &mut neighbors, &mut distances, 0);

    *distances.values().max().unwrap() as u32
}

fn update_neighbors(
    map: &[&str],
    neighbors: &mut Vec<(usize, usize)>,
    distances: &mut HashMap<(usize, usize), usize>,
    current_distance: usize,
) {
    let mut new_neighbors = Vec::new();
    for neighbor in neighbors {
        if !distances.contains_key(neighbor) {
            distances.insert(*neighbor, current_distance + 1);
            new_neighbors.append(&mut get_neighbors(map, neighbor.0, neighbor.1));
        }
    }
    if !new_neighbors.is_empty() {
        update_neighbors(map, &mut new_neighbors, distances, current_distance + 1);
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
    // #[ignore]
    fn test_solution() {
        assert_eq!(6968, solve_puzzle("input"));
    }
}
