use std::iter::zip;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "6";

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
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let times = time_str
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.trim().parse::<usize>().ok());

    let distances = distance_str
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.trim().parse::<usize>().ok());

    let races = zip(times, distances)
        .map(|(time, distance)| (time, distance))
        .collect::<Vec<(usize, usize)>>();

    let mut sum_result_vec: Vec<usize> = Vec::new();

    for (race_time, race_record) in races {
        let mut speeds_faster_than_record = 0;
        for speed_held in 0..race_time {
            let distance_traveled = speed_held * (race_time - speed_held);
            if distance_traveled > race_record {
                speeds_faster_than_record += 1;
            }
        }
        sum_result_vec.push(speeds_faster_than_record);
    }

    let result = sum_result_vec
        .into_iter()
        .reduce(|acc, x| (acc * x))
        .unwrap();

    Solution::from(result)
}

pub fn part2(input: &str) -> Solution {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let time = time_str
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    let distance = distance_str
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    let mut speeds_faster_than_record = 0;
    for speed_held in 0..time {
        let distance_traveled = speed_held * (time - speed_held);
        if distance_traveled > distance {
            speeds_faster_than_record += 1;
        }
    }

    Solution::from(speeds_faster_than_record)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"Time:      7  15   30
Distance:  9  40  200";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(288_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(71503);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
