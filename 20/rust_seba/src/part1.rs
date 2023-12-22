use crate::utils::read_data;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

struct Module {
    mod_type: char,
    name: String,
    on: bool,
    received_pulses: HashMap<String, char>,
    targets: Vec<String>,
    low_pulse_count: usize,
    high_pulse_count: usize,
}

impl Module {
    fn new(mod_type: char, name: String, targets: Vec<String>) -> Self {
        Module {
            mod_type,
            name,
            on: false,
            received_pulses: HashMap::new(),
            targets,
            low_pulse_count: 0,
            high_pulse_count: 0,
        }
    }

    fn toggle(&mut self) {
        self.on = !self.on;
    }

    fn is_off(&self) -> bool {
        !self.on
    }

    fn initiate_input_modules(&mut self, modules: &HashMap<String, Vec<String>>) {
        if self.mod_type != '&' {
            return;
        }
        for (name, targets) in modules.iter() {
            if targets.contains(&self.name) {
                self.receive_pulse('L', name);
            }
        }
    }

    fn receive_pulse(&mut self, pulse: char, sender: &str) {
        self.received_pulses.insert(sender.to_string(), pulse);
    }

    fn increase_count(&mut self, pulse: char) {
        if pulse == 'L' {
            self.low_pulse_count += 1;
        } else {
            self.high_pulse_count += 1;
        }
    }

    fn send_pulse(
        &mut self,
        pulse: char,
        target: String,
        stack: &mut VecDeque<(String, char)>,
        sent_pulses: &mut Vec<(String, char)>,
    ) {
        stack.push_back((target.to_string(), pulse));
        sent_pulses.push((target.to_string(), pulse));
        self.increase_count(pulse);
    }

    fn process_pulse(
        &mut self,
        stack: &mut VecDeque<(String, char)>,
        received_pulse: char,
    ) -> Vec<(String, char)> {
        let mut sent_pulses = Vec::<(String, char)>::new();
        if self.name == "button" {
            self.send_pulse('L', "broadcaster".to_string(), stack, &mut sent_pulses);
        } else if self.name == "broadcaster" {
            for target in self.targets.clone() {
                self.send_pulse(received_pulse, target.to_string(), stack, &mut sent_pulses);
            }
        } else if self.mod_type == '%' {
            if received_pulse == 'L' {
                for target in self.targets.clone() {
                    let next_pulse = if self.is_off() { 'H' } else { 'L' };
                    self.send_pulse(next_pulse, target.to_string(), stack, &mut sent_pulses);
                }
                self.toggle();
            }
        } else if self.mod_type == '&' {
            if self.received_pulses.values().all(|x| x == &'H') {
                for target in self.targets.clone() {
                    self.send_pulse('L', target.to_string(), stack, &mut sent_pulses);
                }
            } else {
                for target in self.targets.clone() {
                    self.send_pulse('H', target.to_string(), stack, &mut sent_pulses);
                }
            }
        }
        sent_pulses
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.mod_type, self.name, self.on)
    }
}

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    // Build modules list from input
    let mut modules = HashMap::<String, Module>::new();
    for line in data.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        let targets = b
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if a == "broadcaster" {
            modules.insert(a.to_string(), Module::new('B', a.to_string(), targets));
        } else {
            let (mod_type, name) = a.split_at(1);
            modules.insert(
                name.to_string(),
                Module::new(mod_type.chars().next().unwrap(), name.to_string(), targets),
            );
        }
    }
    // Add button in  modules
    modules.insert(
        "button".to_string(),
        Module::new('S', "button".to_string(), vec!["broadcaster".to_string()]),
    );

    // initiate received pulses for modules
    let mut modules_targets = HashMap::new();
    for module in modules.values() {
        modules_targets.insert(module.name.to_string(), module.targets.clone());
    }
    for module in modules.values_mut() {
        module.initiate_input_modules(&modules_targets);
    }

    // Initiate stack
    let mut stack = VecDeque::<(String, char)>::new();

    // Process stack 1000 times
    for _ in 0..1000 {
        stack.push_back(("button".to_string(), 'L'));
        while !stack.is_empty() {
            let (name, pulse) = stack.pop_front().unwrap();
            let module = &mut modules.get_mut(&name);
            match module {
                Some(module) => {
                    let sent_pulses = module.process_pulse(&mut stack, pulse);
                    for (target, pulse) in sent_pulses {
                        let target_module = modules.get_mut(&target);
                        if let Some(target_module) = target_module {
                            target_module.receive_pulse(pulse, &name);
                        }
                    }
                }
                None => {
                    println!("{} not found", name);
                }
            }
        }
    }

    // Counting results
    let high_pulses_count: usize = modules.values().map(|x| x.high_pulse_count).sum();
    let low_pulses_count: usize = modules.values().map(|x| x.low_pulse_count).sum();

    high_pulses_count * low_pulses_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(32000000, solve_puzzle("test_data_1"));
    }

    #[test]
    fn test_example_data_2() {
        assert_eq!(11687500, solve_puzzle("test_data_2"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(812609846, solve_puzzle("input"));
    }
}
