use std::collections::HashMap;

use itertools::cloned;
use nom::FindSubstring;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "12";

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
    let data_lines = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (data, numbers) = x.split_once(' ').unwrap();
            (
                data,
                numbers
                    .split(',')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let x = data_lines
        .par_iter()
        .map(|(data, vec)| recursive_configurations(data, vec))
        .sum::<usize>();

    Solution::from(x)
}

fn recursive_configurations(condition_record: &&str, number_groups: &[usize]) -> usize {
    //remove all leading dots as we cant use them
    let usable_string = condition_record.trim_start_matches('.');
    let approx_min_chars_needed_to_end = number_groups.iter().sum::<usize>();

    //The remaning str is less than what is needed for amount of broken sequences
    if usable_string.len() < approx_min_chars_needed_to_end {
        return 0;
    }

    //end of str and number groups meaning we solved it
    if usable_string.is_empty() && number_groups.is_empty() {
        return 1;
    }

    //If no more numbers and no more broken left
    if number_groups.is_empty() && !usable_string.contains('#') {
        return 1;
    }

    if number_groups.is_empty() && !usable_string.is_empty() {
        return 0;
    }

    let mut number_of_configurations = 0;
    //This string can be worked on for this group

    if let Some(chars_needed_for_sequence) = usable_string.get(..number_groups[0]) {
        if !chars_needed_for_sequence.contains(".") {
            //hvis det er slutten eller det er ?/. etter sequence
            if check_is_potential_ok_point(usable_string, number_groups[0]) {
                //The sequence is all
                let continue_from = number_groups[0] + 1;
                let str_to_keep_going_from = &usable_string.get(continue_from..).unwrap_or("");
                let remaining_numbers = &number_groups[1..];

                number_of_configurations +=
                    recursive_configurations(str_to_keep_going_from, remaining_numbers)
            }
        }
    }
    //Needs to count # as one block, cant do multiple with the same one as they are contigues
    if usable_string.starts_with("#") {
        // let skip_current_until_non_square = usable_string[..].trim_start_matches(|x| x == '#');

        // number_of_configurations +=
        //     recursive_configurations(&skip_current_until_non_square, number_groups);
    } else {
        number_of_configurations += recursive_configurations(&&usable_string[1..], number_groups);
    }

    return number_of_configurations;
}

fn remaining_are_all_ok(s: &str) -> bool {
    s.chars().all(|x| x == '.')
}

fn check_is_potential_ok_point(s: &str, idx: usize) -> bool {
    if let Some(string) = s.chars().nth(idx) {
        match string {
            '?' | '.' => true,
            _ => false,
        }
    } else {
        true
    }
}

pub fn part2(input: &str) -> Solution {
    let data_lines = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (data, numbers) = x.split_once(' ').unwrap();
            let mut duplicated_string = String::new();
            for _ in 0..5 {
                duplicated_string += data;
                duplicated_string += "?";
            }
            duplicated_string.pop();

            let numbers = numbers
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let mut duplicated_numbrs = Vec::new();
            for _ in 0..5 {
                for number in numbers.iter() {
                    duplicated_numbrs.push(number.clone());
                }
            }

            (duplicated_string, duplicated_numbrs)
        })
        .collect::<Vec<_>>();

    //

    let x = data_lines
        .par_iter()
        .enumerate()
        // .inspect(|x| println!("on line {}", x.0))
        .map(|(_, (data, vec))| {
            let my_hash = HashMap::new();

            let (patterns, _) = recursive_with_cache(data, vec, &my_hash);

            // println!("map {:?}", return_map);
            patterns
        })
        .sum::<usize>();

    Solution::from(x)
}

fn recursive_with_cache(
    condition_record: &String,
    number_groups: &Vec<usize>,
    visited: &HashMap<(String, Vec<usize>), usize>,
) -> (usize, HashMap<(String, Vec<usize>), usize>) {
    let usable_string = condition_record.trim_start_matches('.');
    let approx_min_chars_needed_to_end = number_groups.iter().sum::<usize>();
    let mut my_map = visited.clone();
    if usable_string.len() < approx_min_chars_needed_to_end {
        my_map.insert((condition_record.clone(), number_groups.clone()), 0);
        return (0, my_map.clone());
    }
    if usable_string.is_empty() && number_groups.is_empty() {
        my_map.insert((condition_record.clone(), number_groups.clone()), 1);
        return (1, my_map.clone());
    }

    //If no more numbers and no more broken left
    if number_groups.is_empty() && !usable_string.contains('#') {
        my_map.insert((condition_record.clone(), number_groups.clone()), 1);
        return (1, my_map.clone());
    }
    if number_groups.is_empty() && !usable_string.is_empty() {
        my_map.insert((condition_record.clone(), number_groups.clone()), 0);
        return (0, my_map.clone());
    }

    let mut number_of_configurations = 0;
    //This string can be worked on for this group
    if let Some(chars_needed_for_sequence) = usable_string.get(..number_groups[0]) {
        if !chars_needed_for_sequence.contains(".") {
            //hvis det er slutten eller det er ?/. etter sequence
            if check_is_potential_ok_point(usable_string, number_groups[0]) {
                //The sequence is all
                let continue_from = number_groups[0] + 1;
                let str_to_keep_going_from =
                    usable_string.get(continue_from..).unwrap_or("").to_string();
                let remaining_numbers = &number_groups[1..].to_vec();
                if let Some(checked_this_before) =
                    my_map.get(&(str_to_keep_going_from.clone(), remaining_numbers.clone()))
                {
                    number_of_configurations += checked_this_before;
                } else {
                    let result_from_sub_str = recursive_with_cache(
                        &str_to_keep_going_from,
                        remaining_numbers,
                        &my_map.clone(),
                    );
                    for entry in result_from_sub_str.1.iter() {
                        my_map.insert(entry.0.clone(), *entry.1);
                    }

                    number_of_configurations += result_from_sub_str.0
                }
            }
        }
    }
    //Needs to count # as one block, cant do multiple with the same one as they are contigues
    if usable_string.starts_with("#") {
        // let skip_current_until_non_square = usable_string[..].trim_start_matches(|x| x == '#');

        // number_of_configurations +=
        //     recursive_configurations(&skip_current_until_non_square, number_groups);
    } else {
        let str_to_keep_going_from = usable_string.get(1..).unwrap_or("").to_string();
        let remaining_numbers = number_groups.clone();
        if let Some(seen_before) =
            my_map.get(&(str_to_keep_going_from.clone(), remaining_numbers.clone()))
        {
            number_of_configurations += seen_before;
        } else {
            let result_from_sub_str = recursive_with_cache(
                &str_to_keep_going_from.clone(),
                &remaining_numbers,
                &my_map.clone(),
            );
            for entry in result_from_sub_str.1.iter() {
                my_map.insert(entry.0.clone(), *entry.1);
            }

            number_of_configurations += result_from_sub_str.0
        }
    }

    my_map.insert(
        (condition_record.clone(), number_groups.clone()),
        number_of_configurations,
    );

    return (number_of_configurations, my_map.clone());
}

fn recursive_configurations_2(condition_record: &&str, number_groups: &[usize]) -> usize {
    //remove all leading dots as we cant use them
    let usable_string = condition_record.trim_start_matches('.');
    let approx_min_chars_needed_to_end = number_groups.iter().sum::<usize>();

    //The remaning str is less than what is needed for amount of broken sequences
    if usable_string.len() < approx_min_chars_needed_to_end {
        return 0;
    }

    //end of str and number groups meaning we solved it
    if usable_string.is_empty() && number_groups.is_empty() {
        return 1;
    }

    //If no more numbers and no more broken left
    if number_groups.is_empty() && !usable_string.contains('#') {
        return 1;
    }

    if number_groups.is_empty() && !usable_string.is_empty() {
        return 0;
    }

    let mut number_of_configurations = 0;
    //This string can be worked on for this group

    if let Some(chars_needed_for_sequence) = usable_string.get(..number_groups[0]) {
        if !chars_needed_for_sequence.contains(".") {
            //hvis det er slutten eller det er ?/. etter sequence
            if check_is_potential_ok_point(usable_string, number_groups[0]) {
                //The sequence is all
                let continue_from = number_groups[0] + 1;
                let str_to_keep_going_from = &usable_string.get(continue_from..).unwrap_or("");
                let remaining_numbers = &number_groups[1..];
                let result_from_sub_str =
                    recursive_configurations_2(str_to_keep_going_from, remaining_numbers);

                number_of_configurations += result_from_sub_str
            }
        }
    }
    //Needs to count # as one block, cant do multiple with the same one as they are contigues
    if usable_string.starts_with("#") {
        // let skip_current_until_non_square = usable_string[..].trim_start_matches(|x| x == '#');

        // number_of_configurations +=
        //     recursive_configurations(&skip_current_until_non_square, number_groups);
    } else {
        number_of_configurations += recursive_configurations_2(&&usable_string[1..], number_groups);
    }

    return number_of_configurations;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(21_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(525152_usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
