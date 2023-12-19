use std::collections::HashMap;



use crate::helpers::solution::Solution;
use crate::helpers::{loader};

const DAY: &str = "15";

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
    let comma_split = input.split(',');

    let result = comma_split
        .map(|x| {
            x.chars().fold(0, |mut acc, c| {
                acc += c as u32;
                acc *= 17;
                acc %= 256;

                acc
            })
        })
        .sum::<u32>();
    Solution::from(result)
}

pub fn part2(input: &str) -> Solution {
    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for single_command in input.split(',') {
        let lense_name = single_command
            .chars()
            .take_while(|x| x.is_alphabetic())
            .collect::<String>();

        let hash = single_command
            .chars()
            .take_while(|x| x.is_alphabetic())
            .fold(0, |mut acc, c| {
                acc += c as u32;
                acc *= 17;
                acc %= 256;

                acc
            });

        let lense = single_command
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .ok();

        let action = single_command
            .chars()
            .filter(|x| *x == '-' || *x == '=')
            .last()
            .unwrap();

        match action {
            '=' => {
                let lense = lense.unwrap();
                if let Some(hash_value) = boxes.get_mut(&hash) {
                    if let Some(exists) = hash_value.iter_mut().find(|x| x.0 == lense_name) {
                        exists.1 = lense;
                    } else {
                        hash_value.push((lense_name, lense))
                    }
                } else {
                    boxes.insert(hash, vec![(lense_name, lense)]);
                }
            }
            '-' => {
                if let Some(hash_value) = boxes.get_mut(&hash) {
                    if let Some(exists) = hash_value.iter_mut().position(|x| x.0 == lense_name) {
                        hash_value.remove(exists);
                    }
                }
            }
            _ => panic!("Managed to get unknown char {}", action),
        }
    }

    let mut result = 0;

    for (box_number, lense) in boxes {
        let adjusted_box_number = box_number + 1;

        let box_value = lense
            .iter()
            .enumerate()
            .map(|(idx, (_, lens_focus))| {
                (adjusted_box_number as usize) * (idx + 1) * *lens_focus as usize
            })
            .sum::<usize>();

        result += box_value;
    }

    Solution::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(1320_u32);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(145);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
