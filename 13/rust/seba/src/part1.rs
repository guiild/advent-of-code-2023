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
    let horizontal_mirror = check_mirror(&map, false);
    if horizontal_mirror.is_some() {
        *top_rows += horizontal_mirror.unwrap() + 1;
        return;
    }

    let vertical_mirror = check_mirror(&map, true);
    if vertical_mirror.is_some() {
        *left_columns += vertical_mirror.unwrap() + 1;
    }
}

fn check_mirror(map: &Vec<Vec<char>>, transpose_map: bool) -> Option<usize> {
    if transpose_map {
        return check_mirror(&transpose(map.clone()), false);
    }
    for i in 0..map.len() - 1 {
        let mut counter = 0;
        let mut is_reflection = true;
        while i >= counter && i + counter < map.len() - 1 {
            if map[i - counter] == map[i + 1 + counter] {
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
        assert_eq!(405, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(35232, solve_puzzle("input"));
    }
}
