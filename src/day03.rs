use std::collections::HashMap;
use std::fmt::DebugTuple;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "3";

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
    let schematic_grid = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut number_string = String::new();
    let mut result = 0;
    let mut touching_symbol = false;

    for (line_idx, line) in schematic_grid.iter().enumerate() {
        for (char_idx, character) in line.iter().enumerate() {
            if char_idx == 0 && number_string.len() != 0 && touching_symbol {
                dbg!(&number_string);
                let number = number_string.parse::<u32>().unwrap();
                result += number;
                touching_symbol = false;
                number_string = "".to_string();
            }
            if character.is_digit(10) {
                number_string += &character.to_string();
                if !touching_symbol {
                    touching_symbol = check_neigtbours_part_1(line_idx, char_idx, &schematic_grid);
                }
            } else {
                if touching_symbol {
                    let number = number_string.parse::<u32>().unwrap();
                    result += number;
                    touching_symbol = false;
                    number_string = "".to_string();
                } else {
                    number_string = "".to_string();
                    touching_symbol = false;
                }
            }
        }
    }
    Solution::from(result)
}

fn check_neigtbours_part_1(line: usize, x: usize, check_vec: &Vec<Vec<char>>) -> bool {
    if line != 0 {
        if let Some(line_above) = check_vec.get(line - 1) {
            if x != 0 {
                if let Some(up_l) = line_above.get(x - 1) {
                    match *up_l != '.' && !up_l.is_digit(10) {
                        true => return true,
                        false => (),
                    }
                }
            }
            if let Some(up) = line_above.get(x) {
                match *up != '.' && !up.is_digit(10) {
                    true => return true,
                    false => (),
                }
            }
            if let Some(up_r) = line_above.get(x + 1) {
                match *up_r != '.' && !up_r.is_digit(10) {
                    true => return true,
                    false => (),
                }
            }
        }
    }

    if let Some(line_below) = check_vec.get(line + 1) {
        if x != 0 {
            if let Some(down_l) = line_below.get(x - 1) {
                match *down_l != '.' && !down_l.is_digit(10) {
                    true => return true,
                    false => (),
                }
            }
        }
        if let Some(down) = line_below.get(x) {
            match *down != '.' && !down.is_digit(10) {
                true => return true,
                false => (),
            }
        }
        if let Some(down_r) = line_below.get(x + 1) {
            match *down_r != '.' && !down_r.is_digit(10) {
                true => return true,
                false => (),
            }
        }
    }
    if let Some(line_same) = check_vec.get(line) {
        if x != 0 {
            if let Some(same_l) = line_same.get(x - 1) {
                match *same_l != '.' && !same_l.is_digit(10) {
                    true => return true,
                    false => (),
                }
            }
        }
        if let Some(same_r) = line_same.get(x + 1) {
            match *same_r != '.' && !same_r.is_digit(10) {
                true => return true,
                false => (),
            }
        }
    }
    false
}

fn check_next_to_star_part_2(
    line: usize,
    x: usize,
    check_vec: &Vec<Vec<char>>,
) -> (bool, HashMap<(u32, u32), u32>) {
    let mut next_to_symbol = false;
    let mut star_coordinates: HashMap<(u32, u32), u32> = HashMap::new();
    if line != 0 {
        if let Some(line_above) = check_vec.get(line - 1) {
            if x != 0 {
                if let Some(up_l) = line_above.get(x - 1) {
                    match *up_l == '*' {
                        true => {
                            next_to_symbol = true;
                            match star_coordinates.get_mut(&(line as u32, x as u32 - 1)) {
                                Some(update) => *update += 1,
                                None => (),
                            }
                        }
                        false => (),
                    }
                }
            }
            if let Some(up) = line_above.get(x) {
                match *up != '.' && !up.is_digit(10) {
                    true => next_to_symbol = true,
                    false => (),
                }
            }
            if let Some(up_r) = line_above.get(x + 1) {
                match *up_r != '.' && !up_r.is_digit(10) {
                    true => next_to_symbol = true,
                    false => (),
                }
            }
        }
    }

    if let Some(line_below) = check_vec.get(line + 1) {
        if x != 0 {
            if let Some(down_l) = line_below.get(x - 1) {
                match *down_l != '.' && !down_l.is_digit(10) {
                    true => next_to_symbol = true,
                    false => (),
                }
            }
        }
        if let Some(down) = line_below.get(x) {
            match *down != '.' && !down.is_digit(10) {
                true => next_to_symbol = true,
                false => (),
            }
        }
        if let Some(down_r) = line_below.get(x + 1) {
            match *down_r != '.' && !down_r.is_digit(10) {
                true => next_to_symbol = true,
                false => (),
            }
        }
    }
    if let Some(line_same) = check_vec.get(line) {
        if x != 0 {
            if let Some(same_l) = line_same.get(x - 1) {
                match *same_l != '.' && !same_l.is_digit(10) {
                    true => next_to_symbol = true,
                    false => (),
                }
            }
        }
        if let Some(same_r) = line_same.get(x + 1) {
            match *same_r != '.' && !same_r.is_digit(10) {
                true => next_to_symbol = true,
                false => (),
            }
        }
    }
    (next_to_symbol, star_coordinates)
}
pub fn part2(input: &str) -> Solution {
    let schematic_grid = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // let possible_gears = schematic_grid
    //     .iter()
    //     .enumerate()
    //     .map(|(line_nmbr, x)| {
    //         (
    //             line_nmbr,
    //             x.iter()
    //                 .enumerate()
    //                 .filter(|(_, symbol)| **symbol == '*')
    //                 .collect::<Vec<(usize, &char)>>(),
    //         )
    //     })
    //     .collect::<Vec<(usize, Vec<(usize, &char)>)>>();

    // dbg!(possible_gears);

    todo!()
}

fn part_2_gear_exactly_two_adjacent(
    line_idx: usize,
    char_idx: usize,
    schematic: Vec<Vec<char>>,
) -> usize {
    let mut number_of_unique_adjacent = 0;
    if line_idx != 0 {
        if let Some(line_above) = schematic.get(line_idx - 1) {}
    }
    if let Some(line_below) = schematic.get(line_idx + 1) {}

    let rigth_side = match schematic.get(line_idx).unwrap().get(char_idx - 1) {
        Some(char_value) => {
            if char_value.is_digit(10) {
                number_of_unique_adjacent += 1
            }
        }
        None => (),
    };
    let left_side = 0;

    number_of_unique_adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(4361u32);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(467835u32);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
