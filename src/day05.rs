use std::u128;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "5";

pub fn run(part: &str) {
    let result = match part {
        "1" => part1(loader::read_text_from_file(part, DAY).as_str()),
        "2" => part2(loader::read_text_from_file(part, DAY).as_str()),
        _ => unimplemented!(),
    };
    println!("Solution for part {} on day {} is:", part, DAY);
    println!("{:?}", result)
}

#[derive(Debug, Clone, Copy)]
struct SourceToDestinationMap {
    pub destination_range_start: usize,
    pub source_range_start: usize,
    pub range_length: usize,
}

pub fn part1(input: &str) -> Solution {
    let split_into_sections = input.split("\n\n").collect::<Vec<&str>>();

    let start_seeds: Vec<usize> = split_into_sections[0]
        .split("seeds: ")
        .skip(1)
        .map(|x| {
            String::from(x)
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    let source_to_destination_maps = split_into_sections[1..]
        .iter()
        .map(|x| {
            x.split('\n')
                .skip(1)
                .map(|x| {
                    let split_white = x
                        .split_whitespace()
                        .filter_map(|x| x.parse().ok())
                        .collect::<Vec<usize>>();
                    SourceToDestinationMap {
                        destination_range_start: split_white[0],
                        source_range_start: split_white[1],
                        range_length: split_white[2],
                    }
                })
                .collect::<Vec<SourceToDestinationMap>>()
        })
        .collect::<Vec<Vec<SourceToDestinationMap>>>();

    let seed_final_mappings = start_seeds
        .iter()
        .map(|x| {
            let mut next_mapping = *x;
            for map in &source_to_destination_maps {
                next_mapping = find_new_number_from_mapping(next_mapping, &map);
            }
            next_mapping
        })
        .collect::<Vec<usize>>();

    Solution::from(*seed_final_mappings.iter().min().unwrap() as usize)
}

fn find_new_number_from_mapping(number: usize, mapping: &Vec<SourceToDestinationMap>) -> usize {
    let mut return_number = number;
    for map_variation in mapping {
        if number < map_variation.source_range_start
            || number > map_variation.source_range_start + map_variation.range_length
        {
            continue;
        }
        let check_number_in_range = (map_variation.source_range_start
            ..map_variation.source_range_start + map_variation.range_length)
            .collect::<Vec<usize>>()
            .iter()
            .position(|x| x == &number);
        if let Some(position) = check_number_in_range {
            return_number = *(map_variation.destination_range_start
                ..map_variation.destination_range_start + map_variation.range_length)
                .collect::<Vec<usize>>()
                .get(position)
                .unwrap()
        }
    }
    return_number
}

pub fn part2(input: &str) -> Solution {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    const TEST_INPUT_TWO: &str = r"";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(35 as usize);
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
