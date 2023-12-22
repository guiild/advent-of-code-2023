use std::collections::HashMap;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let (part1, part2) = data
        .split_once("\n\n")
        .map(|(x, y)| (x.to_string(), y.to_string()))
        .unwrap();
    let workflows = format_workflows(&part1);

    let accepted: Vec<HashMap<char, u32>> = part2
        .lines()
        .filter_map(|l| {
            let part = format_part(l);
            if is_accepted(&part, &workflows) {
                Some(part)
            } else {
                None
            }
        })
        .collect();

    accepted.iter().map(|x| x.values().sum::<u32>()).sum()
}

fn format_part(part: &str) -> HashMap<char, u32> {
    part[1..part.len() - 1]
        .split(',')
        .map(|x| {
            let (key, value) = x.split_once('=').unwrap();
            (key.chars().next().unwrap(), value.parse::<u32>().unwrap())
        })
        .collect()
}

fn format_workflows(workflows: &str) -> HashMap<String, Vec<String>> {
    workflows
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            chars.next_back();
            let l = chars.as_str();
            let (name, conditions) = l.split_once('{').unwrap();
            (
                name.to_string(),
                conditions.split(',').map(|x| x.to_string()).collect(),
            )
        })
        .collect()
}

fn is_accepted(part: &HashMap<char, u32>, workflows: &HashMap<String, Vec<String>>) -> bool {
    apply_rule(part, workflows, String::from("in"))
}

fn apply_rule(
    part: &HashMap<char, u32>,
    workflows: &HashMap<String, Vec<String>>,
    name: String,
) -> bool {
    if name == "R" {
        return false;
    }
    if name == "A" {
        return true;
    }
    let rule = workflows.get(&name).unwrap().clone();
    for condition in rule.iter() {
        if condition == "R" {
            return false;
        }
        if condition == "A" {
            return true;
        }
        if condition.contains('>') {
            let (key, value) = condition
                .split_once('>')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .unwrap();
            if part.get(&key).unwrap() > &value {
                return apply_rule(
                    part,
                    workflows,
                    condition.split_once(':').unwrap().1.to_string(),
                );
            } else {
                continue;
            }
        }
        if condition.contains('<') {
            let (key, value) = condition
                .split_once('<')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .unwrap();
            if part.get(&key).unwrap() < &value {
                return apply_rule(
                    part,
                    workflows,
                    condition.split_once(':').unwrap().1.to_string(),
                );
            } else {
                continue;
            }
        }

        return apply_rule(part, workflows, condition.to_string());
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(19114, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(368523, solve_puzzle("input"));
    }
}
