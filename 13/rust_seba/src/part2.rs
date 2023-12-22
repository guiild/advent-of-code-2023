use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);
    let mut left_columns = 0;
    let mut top_rows = 0;

    for pattern in data.split("\n\n") {
        check_pattern(pattern, &mut top_rows, &mut left_columns);
    }

    top_rows * 100 + left_columns
}

fn check_pattern(pattern: &str, top_rows: &mut usize, left_columns: &mut usize) {
    let map: Vec<Vec<char>> = pattern.lines().map(|line| line.chars().collect()).collect();

    let initial_horizontal_mirror = check_mirrors(&map, false, None, false);
    let new_horizontal_mirror = check_mirrors(&map, true, initial_horizontal_mirror, false);
    if new_horizontal_mirror.is_some() {
        *top_rows += new_horizontal_mirror.unwrap() + 1;
        return;
    }

    let initial_vertical_mirror = check_mirrors(&map, false, None, true);
    let vertical_mirror = check_mirrors(&map, true, initial_vertical_mirror, true);
    if vertical_mirror.is_some() {
        *left_columns += vertical_mirror.unwrap() + 1;
    }
}

fn check_mirrors(
    map: &Vec<Vec<char>>,
    with_error: bool,
    skip_row: Option<usize>,
    transpose_map: bool,
) -> Option<usize> {
    if transpose_map {
        return check_mirrors(&transpose(map.clone()), with_error, skip_row, false);
    }
    for i in 0..map.len() - 1 {
        if with_error && skip_row.is_some() && skip_row.unwrap() == i {
            continue;
        }
        let mut error_found = false;
        let mut counter = 0;
        let mut is_reflection = true;
        while i >= counter && i + counter < map.len() - 1 {
            let difference_count = if with_error {
                map[i - counter]
                    .iter()
                    .zip(map[i + 1 + counter].iter())
                    .filter(|(a, b)| a != b)
                    .count()
            } else {
                0
            };

            if map[i - counter] == map[i + 1 + counter] {
                counter += 1;
            } else if with_error && !error_found && difference_count == 1 {
                error_found = true;
                counter += 1;
            } else {
                is_reflection = false;
                break;
            }
        }
        if is_reflection {
            return Some(i);
        }
    }

    None
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(400, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(37982, solve_puzzle("input"));
    }
}
