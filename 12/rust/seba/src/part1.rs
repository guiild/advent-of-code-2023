use crate::utils::read_data;
use regex::Regex;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    data.lines().enumerate().map(|(n,l)| arrangements(l, n)).sum()
}

fn arrangements(line: &str, index: usize) -> usize {
    println!("Line {}", index);
    println!("Line {}", line);
    let split_line = line.split_once(' ').unwrap();
    let map = split_line.0;
    let numbers = split_line.1.split(',').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let expected_count = numbers.iter().sum::<usize>();
    let mut complete: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();


    if is_complete(&map.to_string(), expected_count)  {
        return 1;
    }

    // BUILD REGEX
    // Start with
    let mut regex = String::from(r"^(\.|\?)*");
    for (i, number) in numbers.iter().enumerate() {
        regex.push_str(&format!("{}{}", r"(#|\?)", String::from("{") +  &number.to_string() + "}"));
        if i == numbers.len() - 1 {
            regex.push_str(r"(\.|\?)*$");
        } else {
            regex.push_str(r"(\.|\?)+");
        }
    }
    println!("Regex: {}", regex);

    stack.push(String::from(map));

    while !stack.is_empty() {
        let candidate = stack.pop().unwrap();
        let diese_count = candidate.chars().filter(|c| c == &'#').count();
        if diese_count >= expected_count {
            continue;
        }
        // For each '?' create a new candidate with a # in that position
        let indexes = candidate.chars().enumerate().filter(|(_, x)|  x == &'?').map(|(i, _)| i).collect::<Vec<usize>>();
        for index in indexes {
            // Replace this index with a #
            let mut new_candidate = String::from(&candidate);
            new_candidate.replace_range(index..index+1, "#");
            if visited.contains(&new_candidate) {
                continue;
            }
            let possible = is_possible(&new_candidate, &regex);
            visited.insert(new_candidate.clone());
            if is_complete(&new_candidate, expected_count) && possible {
                complete.insert(new_candidate.clone());
            } else if possible {
                stack.push(new_candidate.clone());
            }
        }
    }
    println!("Complete: {:?}", complete);
    println!("Complete count: {}", complete.len());
    if  complete.len() == 0 {
        panic!("No complete arrangements found");
    }
    complete.len()

}

fn is_possible(candidate: &String, regex: &String) -> bool {
    let re = Regex::new(&regex).unwrap();
    re.is_match(&candidate)
}

fn is_complete(candidate: &String, expected_count: usize) -> bool {
    candidate.chars().filter(|c| c == &'#').count() == expected_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(21, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(7090, solve_puzzle("input"));
    }

    #[test]
    fn test_already_solved() {
        assert_eq!(1, arrangements("????#???#.?..???? 1,1", 0));
    }
}
