use std::cmp;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "2";

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
struct Game {
    index: usize,
    red: u32,
    blue: u32,
    green: u32,
}

pub fn part1(input: &str) -> Solution {
    let games = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (_, moves) = line.split_once(": ").unwrap();
            let draws = moves.split(';');
            let (mut blue, mut red, mut green) = (0, 0, 0);
            for balls in draws {
                balls.split(',').for_each(|colour| {
                    if let Some((amount, colour)) = colour.trim().split_once(' ') {
                        match colour {
                            "red" => red = cmp::max(red, amount.parse().unwrap()),
                            "green" => green = cmp::max(green, amount.parse().unwrap()),
                            "blue" => blue = cmp::max(blue, amount.parse().unwrap()),
                            _ => unreachable!("{}", colour),
                        };
                    }
                });
            }
            let index = idx + 1;
            Game {
                index,
                red,
                blue,
                green,
            }
        })
        .collect::<Vec<Game>>();

    let valid_games = games
        .iter()
        .filter(|x| x.red <= 12 && x.green <= 13 && x.blue <= 14);

    let answer = valid_games.fold(0, |acc, val| acc + val.index);

    Solution::from(answer)
}

pub fn part2(input: &str) -> Solution {
    let games = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (_, moves) = line.split_once(": ").unwrap();
            let draws = moves.split(';');
            let (mut blue, mut red, mut green) = (0, 0, 0);
            for balls in draws {
                balls.split(',').for_each(|colour| {
                    let (amount, colour) = colour.trim().split_once(' ').unwrap();
                    match colour {
                        "red" => red = cmp::max(red, amount.parse().unwrap()),
                        "green" => green = cmp::max(green, amount.parse().unwrap()),
                        "blue" => blue = cmp::max(blue, amount.parse().unwrap()),
                        _ => unreachable!("{}", colour),
                    };
                });
            }
            let index = idx + 1;
            Game {
                index,
                red,
                blue,
                green,
            }
        })
        .collect::<Vec<Game>>();

    let answer = games
        .iter()
        .fold(0, |acc, x| acc + (x.red * x.green * x.blue));

    Solution::from(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const TEST_INPUT_TWO: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(8usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(2286_u32);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
