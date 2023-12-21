use std::collections::{HashMap, HashSet, VecDeque};

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "20";

pub fn run(part: &str) {
    let result = match part {
        "1" => part1(loader::read_text_from_file(part, DAY).as_str()),
        "2" => part2(loader::read_text_from_file(part, DAY).as_str()),
        _ => unimplemented!(),
    };
    println!("Solution for part {} on day {} is:", part, DAY);
    println!("{:?}", result)
}

pub fn part1(input: &str) -> Solution {
    let mut machines: HashMap<&str, Module> = HashMap::new();

    for line in input.lines() {
        let (first_part, second_part) = line.split_once(" -> ").unwrap();
        match line.chars().nth(0).unwrap() {
            '%' => {
                let name = &first_part[1..];
                let outputs = second_part
                    .split(',')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let module = Module {
                    machine_type: ModuleType::FlipFlop {
                        status: Status::Off,
                    },
                    id: name,
                    observers: outputs,
                };
                machines.insert(name, module);
            }
            '&' => {
                let name = &first_part[1..];
                let outputs = second_part
                    .split(',')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let module = Module {
                    machine_type: ModuleType::Conjunction {
                        memory: HashMap::new(),
                    },
                    id: name,
                    observers: outputs,
                };

                machines.insert(name, module);
            }
            'b' => {
                let name = &first_part;
                let observers = second_part
                    .split(',')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let module = Module {
                    machine_type: ModuleType::Broadcast,
                    id: name,
                    observers,
                };

                machines.insert(name, module);
            }
            _ => panic!("What?! {}", line),
        }
    }
    dbg!(&machines);

    let conjunctions = machines
        .iter()
        .filter_map(|(id, machine)| match &machine.machine_type {
            ModuleType::Broadcast => None,
            ModuleType::FlipFlop { .. } => None,
            ModuleType::Conjunction { .. } => Some(*id),
        })
        .collect::<Vec<&str>>();
    let inputs = machines.iter().fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, (id, machine)| {
            for c in conjunctions.iter() {
                if machine.observers.contains(c) {
                    acc.entry(c)
                        .and_modify(|item| {
                            item.push(id);
                        })
                        .or_insert(vec![id]);
                }
            }
            acc
        },
    );
    inputs
        .into_iter()
        .for_each(|(conjunction, input_machines)| {
            machines.entry(conjunction).and_modify(|machine| {
                let ModuleType::Conjunction { memory, .. } = &mut machine.machine_type else {
                    unreachable!("has to exist");
                };
                *memory = input_machines
                    .into_iter()
                    .map(|id| (id, Signal::Low))
                    .collect();
            });
        });

    let button_pushes = 1000;
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _ in 0..button_pushes {
        low_pulses += 1;
        let mut inbox = VecDeque::<(String, String, Signal)>::from([(
            String::from("button"),
            String::from("broadcaster"),
            Signal::Low,
        )]);
        while let Some((from, id, signal)) = inbox.pop_front() {
            let output = machines
                .get_mut(id.as_str())
                .map(|m| m.process(from.clone(), &signal))
                .unwrap_or(vec![]);
            for (_, _, signal) in output.iter() {
                match signal {
                    Signal::High => {
                        high_pulses += 1;
                    }
                    Signal::Low => {
                        low_pulses += 1;
                    }
                }
            }
            inbox.extend(output);
        }
    }
    dbg!(&high_pulses, &low_pulses);
    let result = high_pulses * low_pulses;

    Solution::from(result)
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum Status {
    On,
    Off,
}
#[derive(Debug)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop { status: Status },
    Conjunction { memory: HashMap<&'a str, Signal> },
}
#[derive(Debug)]
struct Module<'a> {
    machine_type: ModuleType<'a>,
    id: &'a str,
    observers: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn process(
        &mut self,
        sending_machine_id: String,
        signal: &Signal,
    ) -> Vec<(String, String, Signal)> {
        match &mut self.machine_type {
            ModuleType::Broadcast => self
                .observers
                .iter()
                .map(|&id| (self.id.to_string(), id.to_string(), *signal))
                .collect::<Vec<(String, String, Signal)>>(),
            ModuleType::FlipFlop { ref mut status } => match (signal, &status) {
                (Signal::High, _) => vec![],
                (Signal::Low, Status::On) => {
                    *status = Status::Off;
                    self.observers
                        .iter()
                        .map(|&id| (self.id.to_string(), id.to_string(), Signal::Low))
                        .collect::<Vec<(String, String, Signal)>>()
                }
                (Signal::Low, Status::Off) => {
                    *status = Status::On;
                    self.observers
                        .iter()
                        .map(|&id| (self.id.to_string(), id.to_string(), Signal::High))
                        .collect::<Vec<(String, String, Signal)>>()
                }
            },
            ModuleType::Conjunction { memory } => {
                *memory.get_mut(sending_machine_id.as_str()).unwrap() = *signal;

                let new_signal = match memory.values().all(|s| s == &Signal::High) {
                    true => Signal::Low,
                    false => Signal::High,
                };
                self.observers
                    .iter()
                    .map(|id| (self.id.to_string(), id.to_string(), new_signal))
                    .collect::<Vec<(String, String, Signal)>>()
            }
        }
    }
}

pub fn part2(input: &str) -> Solution {
    let mut machines: HashMap<&str, Module> = HashMap::new();

    for line in input.lines() {
        let (first_part, second_part) = line.split_once(" -> ").unwrap();
        match line.chars().nth(0).unwrap() {
            '%' => {
                let name = &first_part[1..];
                let outputs = second_part
                    .split(',')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let module = Module {
                    machine_type: ModuleType::FlipFlop {
                        status: Status::Off,
                    },
                    id: name,
                    observers: outputs,
                };
                machines.insert(name, module);
            }
            '&' => {
                let name = &first_part[1..];
                let outputs = second_part
                    .split(',')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let module = Module {
                    machine_type: ModuleType::Conjunction {
                        memory: HashMap::new(),
                    },
                    id: name,
                    observers: outputs,
                };

                machines.insert(name, module);
            }
            'b' => {
                let name = &first_part;
                let observers = second_part
                    .split(',')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let module = Module {
                    machine_type: ModuleType::Broadcast,
                    id: name,
                    observers,
                };

                machines.insert(name, module);
            }
            _ => panic!("What?! {}", line),
        }
    }
    dbg!(&machines);

    let conjunctions = machines
        .iter()
        .filter_map(|(id, machine)| match &machine.machine_type {
            ModuleType::Broadcast => None,
            ModuleType::FlipFlop { .. } => None,
            ModuleType::Conjunction { .. } => Some(*id),
        })
        .collect::<Vec<&str>>();
    let inputs = machines.iter().fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, (id, machine)| {
            for c in conjunctions.iter() {
                if machine.observers.contains(c) {
                    acc.entry(c)
                        .and_modify(|item| {
                            item.push(id);
                        })
                        .or_insert(vec![id]);
                }
            }
            acc
        },
    );
    inputs
        .into_iter()
        .for_each(|(conjunction, input_machines)| {
            machines.entry(conjunction).and_modify(|machine| {
                let ModuleType::Conjunction { memory, .. } = &mut machine.machine_type else {
                    unreachable!("has to exist");
                };
                *memory = input_machines
                    .into_iter()
                    .map(|id| (id, Signal::Low))
                    .collect();
            });
        });

    let button_pushes = 1000;
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    'outer: for button_presses in 0.. {
        low_pulses += 1;
        let mut inbox = VecDeque::<(String, String, Signal)>::from([(
            String::from("button"),
            String::from("broadcaster"),
            Signal::Low,
        )]);
        while let Some((from, id, signal)) = inbox.pop_front() {
            if id == "pd" && signal == Signal::Low {
                println!("{}", button_presses);
                break 'outer;
            }
            let output = machines
                .get_mut(id.as_str())
                .map(|m| m.process(from.clone(), &signal))
                .unwrap_or(vec![]);
            for (_, _, signal) in output.iter() {
                match signal {
                    Signal::High => {
                        high_pulses += 1;
                    }
                    Signal::Low => {
                        low_pulses += 1;
                    }
                }
            }
            inbox.extend(output);
        }
    }
    dbg!(&high_pulses, &low_pulses);
    let result = high_pulses * low_pulses;

    Solution::from(result)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT_ONE: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST_INPUT_TWO: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[rstest]
    #[case(TEST_INPUT_ONE, 32000000_usize)]
    // #[case(TEST_INPUT_TWO, 11687500_usize)]
    fn test_part_1(#[case] input: &str, #[case] result: usize) {
        let fasit = Solution::from(result);
        let part_solution = part1(input);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    #[ignore = "Not on part 2 yet go away"]
    fn test_part_2() {
        let fasit = Solution::from(200);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
