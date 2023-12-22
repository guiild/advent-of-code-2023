use crate::utils::read_data;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> i128 {
    let data = read_data(file_name);
    let mut split_data = data.split("\n\n");

    // Create seed ranges
    let seed_ranges: Vec<(i128, i128)> = split_data
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>()
        .chunks(2)
        .map(|range| {
            let start = range[0];
            let count = range[1];
            (start, start + count)
        })
        .collect();

    // Create successive maps
    let mut maps: Vec<HashMap<(i128, i128), i128>> = Vec::new();
    split_data.for_each(|x| {
        let mut map = HashMap::new();
        x.split('\n').skip(1).for_each(|y| {
            let numbers = y
                .split(' ')
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            let destination_range_start = numbers[0];
            let source_range_start = numbers[1];
            let length = numbers[2];

            map.insert(
                (source_range_start, source_range_start + length),
                destination_range_start - source_range_start,
            );
        });
        maps.push(map);
    });

    let mut current_ranges = seed_ranges;
    for map in &maps {
        current_ranges = process_ranges(&current_ranges, map);
    }

    *current_ranges.iter().map(|(min, _)| min).min().unwrap()
}

fn process_ranges(
    ranges: &Vec<(i128, i128)>,
    map: &HashMap<(i128, i128), i128>,
) -> Vec<(i128, i128)> {
    let mut new_ranges: Vec<(i128, i128)> = Vec::new();

    for range in ranges {
        let processed_ranges = process_range(range, map);
        new_ranges.extend(processed_ranges);
    }

    new_ranges
}

fn process_range(range: &(i128, i128), map: &HashMap<(i128, i128), i128>) -> Vec<(i128, i128)> {
    let new_ranges = split_range(range, map);

    transform_ranges(&new_ranges, map)
}

fn split_range(range: &(i128, i128), map: &HashMap<(i128, i128), i128>) -> Vec<(i128, i128)> {
    let start = range.0;
    let end = range.1;
    let mut included_limits = map.keys().fold(Vec::new(), |mut acc, (min, max)| {
        if min >= &start && min <= &end {
            acc.push(*min);
        };
        if max >= &start && max <= &end {
            acc.push(*max);
        };
        acc
    });

    included_limits.sort();

    let mut new_ranges: Vec<(i128, i128)> = Vec::new();
    let mut previous_limit = start;
    for limit in included_limits {
        new_ranges.push((previous_limit, limit));
        previous_limit = limit;
    }
    new_ranges.push((previous_limit, end));
    new_ranges
        .iter()
        .filter(|(min, max)| min != max)
        .map(|(min, max)| (*min, *max))
        .collect()
}

fn transform_ranges(
    ranges: &Vec<(i128, i128)>,
    map: &HashMap<(i128, i128), i128>,
) -> Vec<(i128, i128)> {
    let mut new_ranges: Vec<(i128, i128)> = Vec::new();

    for range in ranges {
        let containing_range = find_containing_range(range, map);
        let value = match containing_range {
            None => 0_i128,
            Some(range) => *map.get(range).unwrap(),
        };

        let new_range = (range.0 + value, range.1 + value);
        new_ranges.push(new_range);
    }

    new_ranges
}

fn find_containing_range<'a>(
    range: &'a (i128, i128),
    map: &'a HashMap<(i128, i128), i128>,
) -> Option<&'a (i128, i128)> {
    map.keys()
        .find(|(min, max)| min <= &range.0 && max >= &range.1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(46, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(51399228, solve_puzzle("input"));
    }

    #[test]
    fn test_transform_ranges_included() {
        let mut map = HashMap::new();
        map.insert((10, 20), 5);
        let ranges = vec![(15, 17), (18, 19)];
        let result = transform_ranges(&ranges, &map);
        assert_eq!(vec![(20, 22), (23, 24)], result);
    }

    #[test]
    fn test_transform_ranges_excluded() {
        let mut map = HashMap::new();
        map.insert((10, 20), 5);
        let ranges = vec![(2, 5), (25, 30)];
        let result = transform_ranges(&ranges, &map);
        assert_eq!(vec![(2, 5), (25, 30)], result);
    }

    #[test]
    fn test_transform_ranges_overlapping() {
        let mut map = HashMap::new();
        map.insert((10, 20), 5);
        let ranges = vec![(5, 15)];
        let result = transform_ranges(&ranges, &map);
        assert_eq!(vec![(5, 15)], result);
    }

    #[test]
    fn test_transform_negative() {
        let mut map = HashMap::new();
        map.insert((10, 20), -5);
        let ranges = vec![(12, 15)];
        let result: Vec<(i128, i128)> = transform_ranges(&ranges, &map);
        assert_eq!(vec![(7, 10)], result);
    }

    #[test]
    fn test_find_containing_range_true() {
        let mut map = HashMap::new();
        map.insert((10, 20), 1);
        let range = (15, 16);
        let result = find_containing_range(&range, &map);
        assert_eq!(Some(&(10, 20)), result);
    }

    #[test]
    fn test_find_containing_range_true_long_map() {
        let mut map = HashMap::new();
        map.insert((30, 40), 1);
        map.insert((2, 3), 1);
        map.insert((10, 20), 1);

        let range = (15, 16);
        let result = find_containing_range(&range, &map);
        assert_eq!(Some(&(10, 20)), result);
    }

    #[test]
    fn test_find_containing_range_false_1() {
        let mut map = HashMap::new();
        map.insert((10, 14), 1);
        let range = (15, 16);
        let result = find_containing_range(&range, &map);
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_containing_range_false_2() {
        let mut map = HashMap::new();
        map.insert((18, 25), 1);
        let range = (15, 16);
        let result = find_containing_range(&range, &map);
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_containing_range_false_intersect_1() {
        let mut map = HashMap::new();
        map.insert((14, 17), 1);
        let range = (15, 20);
        let result = find_containing_range(&range, &map);
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_containing_range_false_intersect_2() {
        let mut map = HashMap::new();
        map.insert((17, 25), 1);
        let range = (15, 20);
        let result = find_containing_range(&range, &map);
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_containing_range_false_included() {
        let mut map = HashMap::new();
        map.insert((16, 17), 1);
        let range = (15, 20);
        let result = find_containing_range(&range, &map);
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_containing_range_identical() {
        let mut map = HashMap::new();
        map.insert((15, 20), 1);
        let range = (15, 20);
        let result = find_containing_range(&range, &map);
        assert_eq!(Some(&(15, 20)), result);
    }

    #[test]
    fn test_find_containing_range_identical_min() {
        let mut map = HashMap::new();
        map.insert((15, 25), 1);
        let range = (15, 20);
        let result = find_containing_range(&range, &map);
        assert_eq!(Some(&(15, 25)), result);
    }

    #[test]
    fn test_find_containing_range_identical_max() {
        let mut map = HashMap::new();
        map.insert((12, 20), 1);
        let range = (15, 20);
        let result = find_containing_range(&range, &map);
        assert_eq!(Some(&(12, 20)), result);
    }

    #[test]
    fn test_split_range_no_split() {
        let mut map = HashMap::new();
        map.insert((0, 200), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 20)], result);
    }

    #[test]
    fn test_split_range_3_ranges() {
        let mut map = HashMap::new();
        map.insert((12, 15), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 12), (12, 15), (15, 20)], result);
    }

    #[test]
    fn test_split_range_similar() {
        let mut map = HashMap::new();
        map.insert((10, 20), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 20)], result);
    }

    #[test]
    fn test_split_range_excluded_1() {
        let mut map = HashMap::new();
        map.insert((1, 5), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 20)], result);
    }

    #[test]
    fn test_split_range_excluded_2() {
        let mut map = HashMap::new();
        map.insert((25, 30), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 20)], result);
    }

    #[test]
    fn test_split_range_overlap_right() {
        let mut map: HashMap<(i128, i128), i128> = HashMap::new();
        map.insert((5, 15), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 15), (15, 20)], result);
    }

    #[test]
    fn test_split_range_touch_right() {
        let mut map: HashMap<(i128, i128), i128> = HashMap::new();
        map.insert((5, 10), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 20)], result);
    }

    #[test]
    fn test_split_range_touch_left() {
        let mut map: HashMap<(i128, i128), i128> = HashMap::new();
        map.insert((20, 30), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 20)], result);
    }

    #[test]
    fn test_split_range_overlap_left() {
        let mut map: HashMap<(i128, i128), i128> = HashMap::new();
        map.insert((15, 25), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(vec![(10, 15), (15, 20)], result);
    }

    #[test]
    fn test_split_range_complex_case() {
        let mut map: HashMap<(i128, i128), i128> = HashMap::new();
        map.insert((5, 12), 1);
        map.insert((13, 14), 1);
        map.insert((16, 18), 1);
        map.insert((18, 28), 1);
        let range = (10, 20);
        let result = split_range(&range, &map);
        assert_eq!(
            vec![(10, 12), (12, 13), (13, 14), (14, 16), (16, 18), (18, 20)],
            result
        );
    }
}

// 13202542
