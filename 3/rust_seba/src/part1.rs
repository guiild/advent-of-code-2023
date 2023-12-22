use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    // Imutable needed to iterate over the table
    let referential = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // Mutable copy tf the table that we can dynamically modify
    let mut table = referential.clone();

    referential.iter().enumerate().for_each(|(i, row)| {
        for (j, c) in row.iter().enumerate() {
            if !c.is_ascii_digit() {
                continue;
            }
            if !has_distant_symbols(i, j, &table, (0, 0)) {
                table[i][j] = '.';
            }
        }
    });

    // Convert table to string
    let str_table = table
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("")
        .replace(|c: char| !c.is_ascii_digit(), " ");

    // Split string by space
    str_table
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .sum()
}

fn has_distant_symbols(
    i: usize,
    j: usize,
    table: &Vec<Vec<char>>,
    previous: (usize, usize),
) -> bool {
    if has_adjacent_symbols(i, j, table) {
        return true;
    }

    let surrounding = get_surrounding_full_cells(i, j, table);
    for neighbor in surrounding.iter().filter(|coord| coord != &&previous) {
        if has_distant_symbols(neighbor.0, neighbor.1, table, (i, j)) {
            return true;
        }
    }

    false
}

fn has_adjacent_symbols(i: usize, j: usize, table: &Vec<Vec<char>>) -> bool {
    let surrounding = get_surrounding_full_cells(i, j, table);
    surrounding.iter().any(|(i, j)| {
        let c = table[*i][*j];
        !c.is_ascii_digit() && c != '.'
    })
}

fn get_surrounding_full_cells(i: usize, j: usize, table: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut all_neigbors = Vec::new();
    if i > 0 {
        all_neigbors.push((i - 1, j));
    }
    if i < table.len() - 1 {
        all_neigbors.push((i + 1, j));
    }
    if j > 0 {
        all_neigbors.push((i, j - 1));
    }
    if j < table[0].len() - 1 {
        all_neigbors.push((i, j + 1));
    }
    if i > 0 && j > 0 {
        all_neigbors.push((i - 1, j - 1));
    }
    if i > 0 && j < table[0].len() - 1 {
        all_neigbors.push((i - 1, j + 1));
    }
    if i < table.len() - 1 && j > 0 {
        all_neigbors.push((i + 1, j - 1));
    }
    if i < table.len() - 1 && j < table[0].len() - 1 {
        all_neigbors.push((i + 1, j + 1));
    }

    all_neigbors
        .into_iter()
        .filter(|(i, j)| table[*i][*j] != '.')
        .collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(4361, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(535235, solve_puzzle("input"));
    }
}
