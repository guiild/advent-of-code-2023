use crate::utils::read_data;
use std::collections::VecDeque;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let mut vertices: Vec<(isize, isize)> = data.lines().fold(Vec::new(), |mut acc, line| {
        let hexa = line.split(' ').last().unwrap();
        let distance = isize::from_str_radix(&hexa[2..7], 16).unwrap();
        let direction = &hexa[7..8];

        acc.push(get_vertices(&acc, direction, distance));
        acc
    });

    let xs = vertices.iter().map(|(x, _)| x).collect::<VecDeque<_>>();
    let ys = vertices.iter().map(|(_, y)| y).collect::<VecDeque<_>>();

    let mut ys_rotated = ys.clone();
    ys_rotated.rotate_right(1);

    let mut xs_rotated = xs.clone();
    xs_rotated.rotate_right(1);

    let first_sum: isize = xs
        .iter()
        .zip(ys_rotated.iter())
        .map(|(x, y)| **x * **y)
        .sum();
    let second_sum: isize = ys
        .iter()
        .zip(xs_rotated.iter())
        .map(|(y, x)| **y * **x)
        .sum();

    let result = 0.5 * ((first_sum - second_sum).abs() as f64); // Shoelace formula

    vertices.insert(0, (0, 0));

    let perimeter = vertices
        .iter()
        .zip(vertices.iter().skip(1))
        .map(|(s, e)| (s.0 - e.0).abs() + (s.1 - e.1).abs())
        .sum::<isize>();

    result as usize + (perimeter as f64 / 2.0) as usize + 1
}

fn get_vertices(
    vertices: &Vec<(isize, isize)>,
    direction: &str,
    distance: isize,
) -> (isize, isize) {
    let last_angle = if vertices.is_empty() {
        (0_isize, 0_isize)
    } else {
        *vertices.last().unwrap()
    };

    match direction {
        "0" => (last_angle.0, last_angle.1 + distance), // R
        "1" => (last_angle.0 + distance, last_angle.1), // D
        "2" => (last_angle.0, last_angle.1 - distance), // L
        "3" => (last_angle.0 - distance, last_angle.1), // U
        _ => panic!("Unknown direction: {}", direction),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(952408144115, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(96556251590677, solve_puzzle("input"));
    }
}
