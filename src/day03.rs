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

pub fn part2(input: &str) -> Solution {
    let schematic_grid = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let gear_coordinates = schematic_grid
        .iter()
        .enumerate()
        .map(|(y, x)| {
            x.iter()
                .enumerate()
                .filter(|(x, c)| **c == '*')
                .map(move |(x, c)| (y as isize, x as isize))
        })
        .flatten()
        .collect::<Vec<(isize, isize)>>();

    let mut number_string = String::new();
    let mut number_coords: Vec<(isize, isize)> = Vec::new();
    let mut number_and_coords_vec = Vec::new();

    let number_coordinates: Vec<(Vec<(isize, isize)>, u32)> = schematic_grid
        .iter()
        .enumerate()
        .map(|(y, c)| {
            number_and_coords_vec.clear();
            c.iter().enumerate().for_each(|(x, c)| {
                if x == 0 && number_string.len() != 0 {
                    number_and_coords_vec
                        .push((number_coords.clone(), number_string.parse::<u32>().unwrap()));
                    number_string = "".to_string();
                    number_coords.clear();
                }
                if c.is_digit(10) {
                    number_string.push(*c);
                    number_coords.push((y as isize, x as isize));
                } else {
                    if number_string.len() != 0 {
                        number_and_coords_vec
                            .push((number_coords.clone(), number_string.parse::<u32>().unwrap()));
                    }
                    number_string = "".to_string();
                    number_coords.clear();
                }
            });
            number_and_coords_vec.clone()
        })
        .flatten()
        .collect();
    let mut result = 0;
    for (y_gear, x_gear) in gear_coordinates {
        let near_coords = vec![
            (y_gear - 1, x_gear - 1),
            (y_gear - 1, x_gear),
            (y_gear - 1, x_gear + 1),
            (y_gear, x_gear - 1),
            (y_gear, x_gear + 1),
            (y_gear + 1, x_gear - 1),
            (y_gear + 1, x_gear),
            (y_gear + 1, x_gear + 1),
        ];
        let valid_nearby_numbers = number_coordinates
            .iter()
            .filter(|(number_coords, _)| {
                let mut some = false;
                for number in number_coords {
                    if near_coords.contains(number) {
                        some = true;
                    }
                }
                some
            })
            .map(|x| x.1)
            .collect::<Vec<u32>>();

        if valid_nearby_numbers.len() == 2 {
            let first = valid_nearby_numbers.first().unwrap();
            let last = valid_nearby_numbers.last().unwrap();
            result += first * last;
        }
    }

    Solution::from(result)
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
