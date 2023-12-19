use std::collections::HashMap;
use std::env::current_exe;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "19";

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
    let (rules_input, data_input) = input.split_once("\n\n").unwrap();

    let parts = data_input
        .lines()
        .map(|x| {
            let numbers = x
                .split(',')
                .map(|x| {
                    let number = x
                        .chars()
                        .filter(|x| x.is_numeric())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    number
                })
                .collect::<Vec<_>>();

            Part {
                x: numbers[0],
                m: numbers[1],
                a: numbers[2],
                s: numbers[3],
            }
        })
        .collect::<Vec<Part>>();

    let rules = rules_input
        .lines()
        .map(|line| {
            let rule_name = line.chars().take_while(|x| *x != '{').collect::<String>();
            let rule_paths = &line[rule_name.len() + 1..line.len() - 1];
            let sub_rules = rule_paths
                .split(',')
                .map(|x| {
                    if x.chars().all(|x| x.is_alphabetic()) {
                        return SubRule::Next(x.to_string());
                    }

                    let var_to_check = x.chars().nth(0).unwrap();
                    let less_or_more = x.chars().nth(1).unwrap();
                    let number = x
                        .chars()
                        .filter(|x| x.is_numeric())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    let next = x.split_once(':').unwrap().1.to_string();
                    SubRule::SubRule(var_to_check, less_or_more, number, next)
                })
                .collect::<Vec<SubRule>>();
            (rule_name, sub_rules)
        })
        .collect::<HashMap<String, Vec<SubRule>>>();

    let accepted_parts = parts
        .iter()
        .filter(|x| accepted_part(x, &"in".to_string(), &rules))
        .collect::<Vec<&Part>>();

    let result = accepted_parts.iter().fold(0, |acc, part| {
        let part_sum = part.x + part.m + part.a + part.s;
        acc + part_sum
    });

    Solution::from(result)
}

fn accepted_part(
    part: &Part,
    current_rule_name: &String,
    rule_map: &HashMap<String, Vec<SubRule>>,
) -> bool {
    let rule = rule_map.get(current_rule_name).unwrap();
    let mut result = false;

    for sub_rule in rule {
        match sub_rule {
            SubRule::Next(next) => {
                match next.as_str() {
                    "A" => result = true,
                    "R" => result = false,
                    _ => result = accepted_part(part, next, rule_map),
                }
                break;
            }
            SubRule::SubRule(check_var, less_or_greater, value, next) => match check_var {
                'x' => match less_or_greater {
                    '<' => {
                        if part.x < *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    '>' => {
                        if part.x > *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    _ => panic!("This again?! {}", less_or_greater),
                },
                'm' => match less_or_greater {
                    '<' => {
                        if part.m < *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    '>' => {
                        if part.m > *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    _ => panic!("This again?! {}", less_or_greater),
                },
                'a' => match less_or_greater {
                    '<' => {
                        if part.a < *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    '>' => {
                        if part.a > *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    _ => panic!("This again?! {}", less_or_greater),
                },
                's' => match less_or_greater {
                    '<' => {
                        if part.s < *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    '>' => {
                        if part.s > *value {
                            match next.as_str() {
                                "A" => result = true,
                                "R" => result = false,
                                _ => result = accepted_part(part, next, rule_map),
                            }
                            break;
                        }
                    }
                    _ => panic!("This again?! {}", less_or_greater),
                },
                _ => panic!("what is this?!{}", check_var),
            },
        }
    }

    result
}

#[derive(Debug)]
enum SubRule {
    Next(String),
    SubRule(char, char, usize, String),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

pub fn part2(input: &str) -> Solution {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    const TEST_INPUT_TWO: &str = r"";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(19114_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(200);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
