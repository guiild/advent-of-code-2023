use std::collections::HashMap;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u16 {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u16)
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();

    let exit_cell = (map.len() - 1, map[0].len() - 1);

    // i, j, direction, steps_count, heat
    let start = (0, 0, 'S', 0, 0);
    let mut min_value = u16::MAX;

    let mut visited = HashMap::new();
    let mut stack = vec![start];

    while !stack.is_empty() {
        stack.sort_by(|(_, _, _, _, heat1), (_, _, _, _, heat2)| heat2.cmp(heat1));

        let (i, j, dir, steps_count, heat) = stack.pop().unwrap();

        let next_points = get_next_points(&map, i, j, dir, steps_count, heat, &min_value);
        for (i, j, dir, steps_count, heat) in next_points {
            if (i, j) == exit_cell && steps_count >= 4 {
                min_value = min_value.min(heat);
            }
            if let Some(visited_heat) = visited.get(&(i, j, dir, steps_count)) {
                if heat >= *visited_heat {
                    continue;
                }
            }
            stack.push((i, j, dir, steps_count, heat));
            visited.insert((i, j, dir, steps_count), heat);
        }
    }

    min_value
}

fn get_next_points(
    map: &Vec<Vec<u16>>,
    i: usize,
    j: usize,
    dir: char,
    steps_count: u16,
    heat: u16,
    min_value: &u16,
) -> Vec<(usize, usize, char, u16, u16)> {
    let mut next_points = Vec::new();

    let can_turn = steps_count >= 4;
    let can_continue_straight = steps_count < 10;

    match dir {
        'R' => {
            if can_turn {
                if i > 0 {
                    let next_value = heat + map[i - 1][j];
                    if next_value < *min_value {
                        next_points.push((i - 1, j, 'U', 1, next_value));
                    }
                }
                if i < map.len() - 1 {
                    let next_value = heat + map[i + 1][j];
                    if next_value < *min_value {
                        next_points.push((i + 1, j, 'D', 1, next_value));
                    }
                }
            }
            if can_continue_straight && j < map[0].len() - 1 {
                let next_value = heat + map[i][j + 1];
                if next_value < *min_value {
                    next_points.push((i, j + 1, 'R', steps_count + 1, next_value));
                }
            }
        }
        'L' => {
            if can_continue_straight && j > 0 {
                let next_value = heat + map[i][j - 1];
                if next_value < *min_value {
                    next_points.push((i, j - 1, 'L', steps_count + 1, next_value));
                }
            }
            if can_turn {
                if i > 0 {
                    let next_value = heat + map[i - 1][j];
                    if next_value < *min_value {
                        next_points.push((i - 1, j, 'U', 1, next_value));
                    }
                }
                if i < map.len() - 1 {
                    let next_value = heat + map[i + 1][j];
                    if next_value < *min_value {
                        next_points.push((i + 1, j, 'D', 1, next_value));
                    }
                }
            }
        }
        'U' => {
            if can_continue_straight && i > 0 {
                let next_value = heat + map[i - 1][j];
                if next_value < *min_value {
                    next_points.push((i - 1, j, 'U', steps_count + 1, next_value));
                }
            }
            if can_turn {
                if j > 0 {
                    let next_value = heat + map[i][j - 1];
                    if next_value < *min_value {
                        next_points.push((i, j - 1, 'L', 1, next_value));
                    }
                }
                if j < map[0].len() - 1 {
                    let next_value = heat + map[i][j + 1];
                    if next_value < *min_value {
                        next_points.push((i, j + 1, 'R', 1, next_value));
                    }
                }
            }
        }
        'D' => {
            if can_turn {
                if j > 0 {
                    let next_value = heat + map[i][j - 1];
                    if next_value < *min_value {
                        next_points.push((i, j - 1, 'L', 1, next_value));
                    }
                }
                if j < map[0].len() - 1 {
                    let next_value = heat + map[i][j + 1];
                    if next_value < *min_value {
                        next_points.push((i, j + 1, 'R', 1, next_value));
                    }
                }
            }
            if can_continue_straight && i < map.len() - 1 {
                let next_value = heat + map[i + 1][j];
                if next_value < *min_value {
                    next_points.push((i + 1, j, 'D', steps_count + 1, next_value));
                }
            }
        }
        // Starting point
        'S' => {
            next_points.push((i, j + 1, 'R', 1, heat + map[i][j + 1]));
            next_points.push((i + 1, j, 'D', 1, heat + map[i + 1][j]));
        }
        _ => panic!("Unknown direction"),
    }

    next_points
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(94, solve_puzzle("test_data"));
    }

    #[test]
    fn test_example_data_2() {
        assert_eq!(71, solve_puzzle("test_data_2"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(1037, solve_puzzle("input"));
    }
}
