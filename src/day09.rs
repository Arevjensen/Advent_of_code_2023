use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "9";

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
    let readings_split = input.trim().split('\n');

    let mut sub_data_sets = Vec::new();
    let mut current_diff = Vec::new();

    let mut result = 0;

    for line in readings_split {
        sub_data_sets.clear();
        let mut working_data_set = line
            .split_whitespace()
            .filter_map(|x| x.parse::<isize>().ok())
            .collect::<Vec<_>>();

        let last = *working_data_set.last().unwrap();

        loop {
            current_diff.clear();
            for window in working_data_set.windows(2) {
                current_diff.push(window[1] - window[0]);
            }

            if current_diff.iter().all(|&num| num == 0) {
                break;
            }
            sub_data_sets.push(current_diff.clone());
            working_data_set.clone_from(&current_diff);
        }

        let line_result = sub_data_sets
            .iter()
            .map(|x| x.last().unwrap())
            // .inspect(|x|println!("{}, last {}", x, last))
            .sum::<isize>()
            + last;
        result += line_result;
    }
    Solution::from(result)
}

pub fn part2(input: &str) -> Solution {
    let readings_split = input.trim().split('\n');

    let mut sub_data_sets = Vec::new();
    let mut current_diff = Vec::new();

    let mut result = 0;

    for line in readings_split {
        sub_data_sets.clear();
        let mut working_data_set = line
            .split_whitespace()
            .filter_map(|x| x.parse::<isize>().ok())
            .rev()
            .collect::<Vec<_>>();

        let last = *working_data_set.last().unwrap();

        loop {
            current_diff.clear();
            for window in working_data_set.windows(2) {
                current_diff.push(window[1] - window[0]);
            }

            if current_diff.iter().all(|&num| num == 0) {
                break;
            }
            sub_data_sets.push(current_diff.clone());
            working_data_set.clone_from(&current_diff);
        }

        let line_result = sub_data_sets
            .iter()
            .map(|x| x.last().unwrap())
            .sum::<isize>()
            + last;
        result += line_result;
    }
    Solution::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(114isize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(2isize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
