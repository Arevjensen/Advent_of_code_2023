use std::collections::HashMap;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "8";

pub fn run(part: &str) {
    let result = match part {
        "1" => part1(loader::read_text_from_file(part, DAY).as_str()),
        "2" => part2(loader::read_text_from_file(part, DAY).as_str()),
        _ => unimplemented!(),
    };
    println!("Solution for part {} on day {} is:", part, DAY);
    println!("{:?}", result)
}

#[derive(Debug)]
struct LocationRoutes {
    left: String,
    right: String,
}

pub fn part1(input: &str) -> Solution {
    let (instructions, location_data) = input.split_once("\n\n").unwrap();
    let goal = "ZZZ".to_string();

    let location_traversal = location_data
        .lines()
        .map(|x| {
            let (key, paths) = x.split_once(" = ").unwrap();
            let binding = paths.replace(['(', ')'], "");
            let (left, rigth) = binding.trim().split_once(", ").unwrap();
            let location_routes = LocationRoutes {
                left: left.to_string(),
                right: rigth.to_string(),
            };
            (key.to_string(), location_routes)
        })
        .collect::<HashMap<String, LocationRoutes>>();

    let mut current_location = "AAA".to_string();
    let mut steps = 0;

    for (idx, x) in instructions.chars().cycle().enumerate() {
        if current_location == "ZZZ" {
            steps = idx;
            break;
        }
        let current_available_routes = location_traversal.get(&current_location).unwrap();
        current_location = match x {
            'L' => current_available_routes.left.clone(),
            'R' => current_available_routes.right.clone(),
            _ => unreachable!(),
        };
    }

    Solution::from(steps)
}

pub fn part2(input: &str) -> Solution {
    let (instructions, location_data) = input.split_once("\n\n").unwrap();
    let mut vec_start_locations = Vec::new();

    let location_traversal = location_data
        .lines()
        .map(|x| {
            let (key, paths) = x.split_once(" = ").unwrap();
            let binding = paths.replace(['(', ')'], "");
            let (left, rigth) = binding.trim().split_once(", ").unwrap();
            let location_routes = LocationRoutes {
                left: left.to_string(),
                right: rigth.to_string(),
            };
            if key.ends_with('A') {
                vec_start_locations.push(key.to_string());
            }
            (key.to_string(), location_routes)
        })
        .collect::<HashMap<String, LocationRoutes>>();

    let mut number_of_steps_for_each_start = HashMap::new();

    for start_location in vec_start_locations {
        let mut current_location = start_location.clone();
        for (idx, x) in instructions.chars().cycle().enumerate() {
            if current_location.ends_with('Z') {
                number_of_steps_for_each_start.insert(start_location.clone(), idx);
                break;
            }
            let current_available_routes = location_traversal.get(&current_location).unwrap();
            current_location = match x {
                'L' => current_available_routes.left.clone(),
                'R' => current_available_routes.right.clone(),
                _ => unreachable!(),
            };
        }
    }

    let steps_as_vec = Vec::from_iter(number_of_steps_for_each_start.values().copied());
    let result = lcm(steps_as_vec.as_slice());

    Solution::Usize(result)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const TEST_INPUT_TWO: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(2usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(6usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
