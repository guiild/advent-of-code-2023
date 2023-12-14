use crate::utils::read_data;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    data.lines().enumerate().map(|(_, l)| arrangements(l)).sum()
}

fn arrangements(line: &str) -> usize {
    let split_line = line.split_once(' ').unwrap();
    let map = split_line.0;
    let numbers = split_line
        .1
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    get_count(map, numbers)
}

fn get_count(map: &str, numbers: Vec<usize>) -> usize {
    // If not enough space on map, return 0
    if map.len() < numbers.iter().sum::<usize>() {
        return 0;
    }

    // If map is exactly the size of the last number, and only contains # or ?, return 1
    if map.len() == numbers[0] && map.chars().all(|c| c == '#' || c == '?') && numbers.len() == 1 {
        return 1;
    }

    let mut count = 0;
    let number = numbers[0];
    let mut next_maps = Vec::new();
    for i in 0..=map.len() - number {
        let target_zone = &map[i..i + number];
        if target_zone.chars().all(|c| c == '#' || c == '?') {
            let next_char = map.chars().nth(i + number);
            let previous_char = if i > 0 { map.chars().nth(i - 1) } else { None };
            if next_char == Some('#') || previous_char == Some('#') {
                if map.chars().nth(i) == Some('#') {
                    break;
                }
                continue;
            }
            if next_char.is_none() && numbers.len() == 1 {
                count += 1;
            }
            let next_map = if map.len() > i + number + 1 {
                &map[i + number + 1..]
            } else {
                ""
            };
            next_maps.push(next_map);
        }
        if map.chars().nth(i) == Some('#') {
            break;
        }
    }

    let empty_maps = next_maps
        .iter()
        .filter(|m| !m.chars().any(|c| c == '#'))
        .count();
    if numbers.len() == 1 {
        return empty_maps;
    }

    let unique_maps = next_maps.iter().collect::<HashSet<_>>();
    for next_map in unique_maps {
        count += get_count(next_map, numbers[1..].to_vec());
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(21, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(7090, solve_puzzle("input"));
    }

    #[test]
    fn test_already_solved() {
        assert_eq!(1, arrangements("????#???#.?..???? 1,1"));
    }

    #[test]
    fn test_example_1() {
        assert_eq!(1, arrangements("???.### 1,1,3"));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(4, arrangements(".??..??...?##. 1,1,3"));
    }

    #[test]
    fn test_example_3() {
        assert_eq!(1, arrangements("?#?#?#?#?#?#?#? 1,3,1,6"));
    }

    #[test]
    fn test_example_4() {
        assert_eq!(1, arrangements("????.#...#... 4,1,1"));
    }

    #[test]
    fn test_example_5() {
        assert_eq!(4, arrangements("????.######..#####. 1,6,5"));
    }

    #[test]
    fn test_example_6() {
        assert_eq!(10, arrangements("?###???????? 3,2,1"));
    }

    #[test]
    fn test_get_count_1() {
        assert_eq!(1, get_count("???.###", vec![1, 1, 3]));
    }

    #[test]
    fn test_get_count_empty() {
        assert_eq!(0, get_count("", vec![1]));
    }

    #[test]
    fn test_get_count_three() {
        assert_eq!(1, get_count("###", vec![3]));
    }

    #[test]
    fn test_get_count_three_out_of_four() {
        assert_eq!(2, get_count("????", vec![3]));
    }

    #[test]
    fn test_get_count_three_out_of_four_first_() {
        assert_eq!(1, get_count("#???", vec![3]));
    }

    #[test]
    fn test_get_count_three_out_of_four_first_sharp() {
        assert_eq!(2, get_count("?#??", vec![3]));
    }

    #[test]
    fn test_get_count_one_out_of_four() {
        assert_eq!(4, get_count("????", vec![1]));
    }

    #[test]
    fn test_long_arrangement() {
        assert_eq!(
            1,
            arrangements("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3")
        )
    }

    #[test]
    fn test_long_arrangement_2() {
        assert_eq!(506250, arrangements("?????????###??????????###??????????###??????????###???????? 2,1,3,2,1,3,2,1,3,2,1,3,2,1"));
        assert_eq!(506250, arrangements("?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1"))
    }

    #[test]
    fn test_long_arrangement_3() {
        assert_eq!(16, arrangements("????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#... 4,1,1,4,1,1,4,1,1,4,1,1,4,1,1"))
    }

    #[test]
    fn test_long_arrangement_4() {
        assert_eq!(2500, arrangements("????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####. 1,6,5,1,6,5,1,6,5,1,6,5,1,6,5"))
    }
}
