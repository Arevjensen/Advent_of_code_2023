use nom::InputTake;

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
        .iter()
        .map(|(data, vec)| recursive_configurations(data, vec))
        .sum::<usize>();

    Solution::from(100)
}

fn recursive_configurations(condition_record: &&str, number_groups: &[usize]) -> usize {
    // match (condition_record.len(), number_groups.len()) {
    //     (_, 0) => return 1,
    //     (0, _) => return 0,
    //     (_, _) => (),
    // }

    //Planen
    //skip til forste # eller ? i incoming str
    //Sjekk sa at det er # eller ? pa lengde med neste number_group
    // og at neste etter det er ?/./slutt slik at det blir skille mellom denne og neste
    // send saa resten av str og resten av nr videre i rekursjonen
    // hvis alle tallgrupper er brukt og det ikke er flere # igjen i str saa +1 alternativer

    let mut number_of_configurations = 0;
    //trim until ? or #
    let usable_string = condition_record.trim_start_matches('.');

    //Sjekk at vi ikke er pa slutten eller har kun ok igjen
    if !usable_string.is_empty() || !remaining_are_all_ok(usable_string) {
        //This string can be worked on
        if let Some(next_needed_sequence) = usable_string.get(..number_groups[0]) {
            // Det er x mengde symboler fremover

            //hvis det er slutten eller det er ?/. etter sequence
            if check_is_potential_ok_point(usable_string, number_groups[0] + 1) {
            } else {
                //There was more broken after so we cant use current number sequence here and have to move on
            }
            //     let etter_sequence = usable_string.chars().nth(number_groups[0] + 1);
            //     if let Some(etter_sequence_checked) = etter_sequence {
            //         if etter_sequence_checked == '.' || etter_sequence_checked == '?' {
            //             //Ok to continiue
            //         }
            //         return 1;
            //     } else {
            //         if number_groups.get(1..).is_none() && false {
            //             //Something
            //         }
            //     }
            // } else {
            //     //Vi klarte ikke aa hente ut nok til a fylle tallgruppen
            //     return number_of_configurations;
            // }
        } else {
            //String er tom eller vi kan ikke gjore mer paa den
            return number_of_configurations;
        }
    }

    fn remaining_are_all_ok(s: &str) -> bool {
        s.chars().all(|x| x == '.')
    }

    fn check_is_potential_ok_point(s: &str, idx: usize) -> bool {
        if let Some(string) = s.get(idx..idx + 1) {
            match string {
                "?" | "." => true,
                _ => false,
            }
        } else {
            true
        }
    }

    // if let Some(check_str) = condition_record.get(..number_groups[0]) {
    //last to check if sequence of broken ones are seperated at the end
    // let last_in_check_str = check_str.chars().last().unwrap();
    // if last_in_check_str == '?' || last_in_check_str == '.' {
    //     if check_str[..number_groups[0]]
    //         .chars()
    //         .all(|x| x == '#' || x == '?')
    //     {
    //         number_of_configurations +=
    //             recursive_configurations(condition_record, &number_groups[1..]);
    //     }
    // }
    // }
    // if next_possible_location_and_space
    //     .take(3)
    //     .chars()
    //     .all(|x| x == '?' || x == '#')
    //     && (next_possible_location_and_space.chars().last().unwrap() == '?'
    //         || next_possible_location_and_space.chars().last().unwrap() == '.')
    // {
    //     let remaining_str = &next_possible_location_and_space[4..];
    //     number_of_configurations += recursive_configurations(&remaining_str, &number_groups[1..]);
    // } else {
    //     let remaining_str = &next_possible_location_and_space[1..];
    //     number_of_configurations += recursive_configurations(&remaining_str, number_groups);
    // }

    number_of_configurations
}

pub fn part2(input: &str) -> Solution {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1 ";
    const TEST_INPUT_TWO: &str = r"";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(21_usize);
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
