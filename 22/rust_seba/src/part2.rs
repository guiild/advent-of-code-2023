use std::collections::HashMap;
use std::collections::HashSet;

use crate::cube::Cube;
use crate::utils::read_data;
use itertools::Itertools;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut cubes = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once('~').unwrap();
            let (x, y, z) = start
                .splitn(3, ',')
                .map(|el| el.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (xx, yy, zz) = end
                .splitn(3, ',')
                .map(|el| el.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            Cube::new(i as u32, x, y, z, xx, yy, zz)
        })
        .collect::<Vec<Cube>>();

    cubes.sort();

    for i in 0..cubes.len() {
        let cubes_ref = cubes.clone();
        let cube = &mut cubes.get_mut(i).unwrap();
        cube.settle_at_next_support_level(&cubes_ref);
    }

    let dependencies = cubes
        .iter()
        .map(|cube| (cube.index, cube.dependencies(&cubes)))
        .collect::<HashMap<u32, Vec<u32>>>();

    cubes
        .iter()
        .map(|c| destroy_cube(c.index, &dependencies))
        .sum()
}

fn destroy_cube(cube: u32, dependencies: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut destroyed: HashSet<u32> = HashSet::new();
    let mut stack = vec![cube];

    while let Some(current_cube) = stack.pop() {
        let current_index = current_cube;
        destroyed.insert(current_cube);

        let dependent_indexes = dependencies
            .iter()
            .filter(|(_k, v)| v.contains(&current_index))
            .map(|(k, _v)| k)
            .collect::<Vec<&u32>>();

        for d in dependent_indexes {
            if dependencies
                .get(d)
                .unwrap()
                .iter()
                .all(|i| destroyed.contains(i))
            {
                stack.push(*d);
            }
        }
    }

    destroyed.len() as u32 - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(7, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(101541, solve_puzzle("input"));
    }
}
