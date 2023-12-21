use std::collections::HashMap;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let (part1, _) = data
        .split_once("\n\n")
        .map(|(x, y)| (x.to_string(), y.to_string()))
        .unwrap();
    let workflows = format_workflows(&part1);

    let mut ranges = HashMap::new();
    ranges.insert('a', (1, 4001));
    ranges.insert('m', (1, 4001));
    ranges.insert('s', (1, 4001));
    ranges.insert('x', (1, 4001));

    let mut accepted_ranges: Vec<HashMap<char, (usize, usize)>> = Vec::new();

    process_ranges(
        &workflows,
        &mut ranges,
        "in".to_string(),
        &mut accepted_ranges,
    );

    accepted_ranges
        .iter()
        .map(get_combinations_count)
        .sum()
}

fn get_combinations_count(ranges: &HashMap<char, (usize, usize)>) -> usize {
    ranges.values().map(|range| range.1 - range.0).product()
}

fn process_ranges(
    workflows: &HashMap<String, Vec<String>>,
    ranges: &mut HashMap<char, (usize, usize)>,
    name: String,
    accepted_ranges: &mut Vec<HashMap<char, (usize, usize)>>,
) {
    if name == "R" {
        return;
    }
    if name == "A" {
        accepted_ranges.push(ranges.clone());
        return;
    }
    let conditions = workflows.get(&name).unwrap();
    // println!("conditions {:?}", conditions);
    for condition in conditions {
        if condition == "R" {
            continue;
        }
        if condition == "A" {
            accepted_ranges.push(ranges.clone());
            continue;
        }
        if !condition.contains('>') && !condition.contains('<') {
            process_ranges(workflows, ranges, condition.to_string(), accepted_ranges);
        }

        if condition.contains('<') {
            let (key, value) = condition
                .split_once('<')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .unwrap();
            let current_range_for_char = ranges.get(&key).unwrap();
            let range_in_condition = (current_range_for_char.0, value);
            let remaining_range = (value, current_range_for_char.1);

            let mut new_ranges = ranges.clone();
            new_ranges.insert(key, range_in_condition);
            process_ranges(
                workflows,
                &mut new_ranges,
                condition.split_once(':').unwrap().1.to_string(),
                accepted_ranges,
            );
            ranges.insert(key, remaining_range);
        }

        if condition.contains('>') {
            let (key, value) = condition
                .split_once('>')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .unwrap();
            let current_range_for_char = ranges.get(&key).unwrap();
            let range_in_condition = (value + 1, current_range_for_char.1);
            let remaining_range = (current_range_for_char.0, value + 1);

            let mut new_ranges = ranges.clone();
            new_ranges.insert(key, range_in_condition);
            process_ranges(
                workflows,
                &mut new_ranges,
                condition.split_once(':').unwrap().1.to_string(),
                accepted_ranges,
            );
            ranges.insert(key, remaining_range);
        }
    }
}

fn format_workflows(workflows: &str) -> HashMap<String, Vec<String>> {
    let mut all_workflows: HashMap<String, Vec<String>> = HashMap::new();
    for mut workflow in workflows.lines() {
        let mut chars = workflow.chars();
        chars.next_back();
        workflow = chars.as_str();
        let (name, conditions) = workflow.split_once('{').unwrap();
        all_workflows.insert(
            name.to_string(),
            conditions.split(',').map(|x| x.to_string()).collect(),
        );
    }
    all_workflows
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(167409079868000, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(124167549767307, solve_puzzle("input"));
    }
}
