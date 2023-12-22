use crate::utils::read_data;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> u128 {
    let data = read_data(file_name);
    let mut split_data = data.split("\n\n");

    let seeds = split_data.next().unwrap();

    let mut maps: Vec<HashMap<(u128, u128), i128>> = Vec::new();

    split_data.for_each(|x| {
        let mut map = HashMap::new();
        x.split('\n').skip(1).for_each(|y| {
            let numbers = y
                .split(' ')
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            let destination_range_start = numbers[0];
            let source_range_start = numbers[1];
            let length = numbers[2];

            map.insert(
                (source_range_start, source_range_start + length - 1),
                destination_range_start as i128 - source_range_start as i128,
            );
        });
        maps.push(map);
    });

    // VERY NAIVE SOLUTION looping over aaaaaall millions of seeds
    let mut min: u128 = 10000000000;
    seeds
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>()
        .chunks(2)
        .for_each(|range| {
            let start = range[0];
            let count = range[1];
            for i in start..start + count {
                let mut location = i;
                for map in &maps {
                    location = get_from_source_to_destination(map, location);
                }
                if location < min {
                    println!("{}", location);
                    min = location;
                }
            }
        });

    min
}

fn get_from_source_to_destination(map: &HashMap<(u128, u128), i128>, source: u128) -> u128 {
    let range = map
        .keys()
        .find(|(min, max)| min <= &source && max >= &source);
    let result = match range {
        None => source,
        Some(range) => (source as i128 + map.get(range).unwrap()) as u128,
    };

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(46, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(51399228, solve_puzzle("input"));
    }
}
