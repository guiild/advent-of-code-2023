use std::collections::HashMap;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for instruction in data.split(',') {
        if instruction.contains('=') {
            let mut split = instruction.split('=');
            let label = split.next().unwrap();
            let number = split.next().unwrap().parse::<u32>().unwrap();
            let box_number = hash(label);
            let tuple = (label.to_string(), number);
            let box_content = boxes.entry(box_number).or_default();
            let existing = box_content.iter().position(|x| x.0 == label);
            match existing {
                Some(index) => {
                    box_content[index] = tuple;
                }
                None => {
                    box_content.push(tuple);
                }
            }
        } else {
            let mut split = instruction.split('-');
            let label = split.next().unwrap();
            let box_number = hash(label);
            let box_content = boxes.entry(box_number).or_default();
            let existing = box_content.iter().position(|x| x.0 == label);
            if let Some(index) = existing {
                box_content.remove(index);
            }
        }
    }

    boxes
        .into_iter()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .map(|(i, lens)| (i + 1) as u32 * lens.1 * (1 + k))
                .sum::<u32>()
        })
        .sum()
}

fn hash(input: &str) -> u32 {
    let mut result = 0;
    for c in input.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(145, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(271384, solve_puzzle("input"));
    }
}
