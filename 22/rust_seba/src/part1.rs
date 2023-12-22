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

    let cubes_copy = cubes.clone();
    cubes.iter().filter(|c| c.can_destroy(&cubes_copy)).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(5, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(515, solve_puzzle("input"));
    }
}
