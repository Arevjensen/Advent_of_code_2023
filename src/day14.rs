use std::collections::{HashMap, HashSet};

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "14";

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
    let result = move_columns_north_part_1(input);
    Solution::from(result)
}

fn move_columns_north_part_1(input: &str) -> usize {
    let mut my_memory_leave_me_alone = input.lines().map(|x| x.chars()).collect::<Vec<_>>();

    let columns: Vec<Vec<char>> = std::iter::from_fn(move || {
        let mut col_items = vec![];
        for symbols in &mut my_memory_leave_me_alone {
            match symbols.next() {
                Some(ash_rock) => col_items.push(ash_rock),
                None => return None,
            }
        }
        Some(col_items)
    })
    .collect();

    let mut new_columns: Vec<String> = Vec::new();

    for column in columns {
        let mut moved_column = String::new();
        for (idx, symbol) in column.iter().enumerate() {
            match symbol {
                '#' => {
                    while moved_column.len() < idx {
                        moved_column.push('.');
                    }
                    moved_column.push('#')
                }
                'O' => moved_column.push('O'),
                '.' => {}
                _ => unreachable!(),
            }
        }
        while moved_column.len() < column.len() {
            moved_column.push('.');
        }
        new_columns.push(moved_column)
    }

    let mut total = 0;
    for column in new_columns {
        let column_reversed = column.chars().rev().enumerate();

        let mut number_of_round_rocks = 0;
        for (idx, place) in column_reversed {
            match place {
                'O' => number_of_round_rocks += idx + 1,
                _ => {}
            }
        }
        total += number_of_round_rocks;
    }

    total
}
pub fn part2(input: &str) -> Solution {
    let grid = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut rounds_remaining = 0_usize;

    let mut last_position: Vec<String> = grid;
    let mut hash_earlier_positions: HashMap<Vec<String>, usize> = HashMap::new();

    while rounds_remaining < 1000000000 {
        if rounds_remaining < 5000 {
            let something = hash_earlier_positions.insert(last_position.clone(), rounds_remaining);
            if let Some(previous_time_we_saw_this_position) = something {
                let diff = rounds_remaining - previous_time_we_saw_this_position;

                while rounds_remaining + diff < 1000000000 {
                    rounds_remaining += diff;
                }
            }
        }

        last_position = do_loop(&last_position);
        rounds_remaining += 1;
    }

    let mut total = 0;
    for (idx, row) in last_position.iter().enumerate().rev() {
        let mut rock_weigth_line = 0;
        for place in row.chars() {
            match place {
                'O' => rock_weigth_line += last_position.len() - idx,
                _ => {}
            }
        }
        total += rock_weigth_line;
    }
    Solution::Usize(total)
}

fn do_loop(input: &Vec<String>) -> Vec<String> {
    let one_north = move_grid_north(&input);
    let one_west = move_grid_west(&one_north);
    let one_south = move_grid_south(&one_west);
    move_grid_east(&one_south)
}

fn move_grid_west(input: &Vec<String>) -> Vec<String> {
    let mut new_rows: Vec<String> = Vec::new();

    for column in input.iter() {
        let mut moved_column = String::new();
        for (idx, symbol) in column.chars().enumerate() {
            match symbol {
                '#' => {
                    while moved_column.len() < idx {
                        moved_column.push('.');
                    }
                    moved_column.push('#')
                }
                'O' => moved_column.push('O'),
                '.' => {}
                _ => unreachable!(),
            }
        }
        while moved_column.len() < column.len() {
            moved_column.push('.');
        }
        new_rows.push(moved_column)
    }

    new_rows
}
fn move_grid_east(input: &Vec<String>) -> Vec<String> {
    let mut new_rows: Vec<String> = Vec::new();

    for column in input.iter() {
        let mut moved_column = String::new();
        for (idx, symbol) in column.chars().rev().enumerate() {
            match symbol {
                '#' => {
                    while moved_column.len() < idx {
                        moved_column.push('.');
                    }
                    moved_column.push('#')
                }
                'O' => moved_column.push('O'),
                '.' => {}
                _ => unreachable!(),
            }
        }
        while moved_column.len() < column.len() {
            moved_column.push('.');
        }
        new_rows.push(moved_column)
    }
    let new_rows = new_rows
        .iter_mut()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    new_rows
}

fn move_grid_north(input: &Vec<String>) -> Vec<String> {
    let mut my_memory_leave_me_alone = input.iter().map(|x| x.chars()).collect::<Vec<_>>();

    let columns: Vec<Vec<char>> = std::iter::from_fn(move || {
        let mut col_items = vec![];
        for symbols in &mut my_memory_leave_me_alone {
            match symbols.next() {
                Some(ash_rock) => col_items.push(ash_rock),
                None => return None,
            }
        }
        Some(col_items)
    })
    .collect();

    let mut new_columns: Vec<String> = Vec::new();

    for column in columns {
        let mut moved_column = String::new();
        for (idx, symbol) in column.iter().enumerate() {
            match symbol {
                '#' => {
                    while moved_column.len() < idx {
                        moved_column.push('.');
                    }
                    moved_column.push('#')
                }
                'O' => moved_column.push('O'),
                '.' => {}
                _ => unreachable!(),
            }
        }
        while moved_column.len() < column.len() {
            moved_column.push('.');
        }
        new_columns.push(moved_column)
    }

    let mut more_wasted_memory = new_columns.iter().map(|x| x.chars()).collect::<Vec<_>>();
    let rows: Vec<Vec<char>> = std::iter::from_fn(move || {
        let mut col_items = vec![];
        for symbols in &mut more_wasted_memory {
            match symbols.next() {
                Some(ash_rock) => col_items.push(ash_rock),
                None => return None,
            }
        }
        Some(col_items)
    })
    .collect();

    rows.iter().map(|x| x.iter().collect::<String>()).collect()
}

fn move_grid_south(input: &Vec<String>) -> Vec<String> {
    let mut my_memory_leave_me_alone = input.iter().map(|x| x.chars()).collect::<Vec<_>>();

    let columns: Vec<Vec<char>> = std::iter::from_fn(move || {
        let mut col_items = vec![];
        for symbols in &mut my_memory_leave_me_alone {
            match symbols.next() {
                Some(ash_rock) => col_items.push(ash_rock),
                None => return None,
            }
        }
        Some(col_items)
    })
    .collect();

    let mut new_columns: Vec<String> = Vec::new();

    for column in columns {
        let mut moved_column = String::new();
        for (idx, symbol) in column.iter().rev().enumerate() {
            match symbol {
                '#' => {
                    while moved_column.len() < idx {
                        moved_column.push('.');
                    }
                    moved_column.push('#')
                }
                'O' => moved_column.push('O'),
                '.' => {}
                _ => unreachable!(),
            }
        }
        while moved_column.len() < column.len() {
            moved_column.push('.');
        }
        new_columns.push(moved_column)
    }
    let new_columns = new_columns
        .iter_mut()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let mut more_wasted_memory = new_columns.iter().map(|x| x.chars()).collect::<Vec<_>>();
    let rows: Vec<Vec<char>> = std::iter::from_fn(move || {
        let mut col_items = vec![];
        for symbols in &mut more_wasted_memory {
            match symbols.next() {
                Some(ash_rock) => col_items.push(ash_rock),
                None => return None,
            }
        }
        Some(col_items)
    })
    .collect();

    rows.iter().map(|x| x.iter().collect::<String>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(136_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(64_usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
