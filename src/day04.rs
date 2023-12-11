use crate::helpers::loader;
use crate::helpers::solution::Solution;
use std::collections::HashMap;

const DAY: &str = "4";

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
    let games = input
        .lines()
        .map(|line| {
            let (_, winners_and_scratched) = line.split_once(": ").unwrap();
            let winners_scarched_split = winners_and_scratched.split_once('|').unwrap();
            let (winners_str, scratched_str) = (winners_scarched_split.0, winners_scarched_split.1);
            let winners_vec = winners_str
                .trim()
                .split(' ')
                .filter_map(|x| x.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();
            let sratched_vec = scratched_str
                .trim()
                .split(' ')
                .filter_map(|x| x.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();
            (winners_vec, sratched_vec)
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>();

    let mut total_score = 0;
    for game in games {
        let mut game_score = 0;
        for winning_number in game.0 {
            if game.1.contains(&winning_number) {
                if game_score == 0 {
                    game_score = 1;
                } else {
                    game_score *= 2;
                }
            }
        }
        total_score += game_score;
    }
    Solution::from(total_score)
}

pub fn part2(input: &str) -> Solution {
    let games = input
        .lines()
        .map(|line| {
            let (_, winners_and_scratched) = line.split_once(": ").unwrap();
            let winners_scarched_split = winners_and_scratched.split_once('|').unwrap();
            let (winners_str, scratched_str) = (winners_scarched_split.0, winners_scarched_split.1);
            let winners_vec = winners_str
                .trim()
                .split(' ')
                .filter_map(|x| x.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();
            let sratched_vec = scratched_str
                .trim()
                .split(' ')
                .filter_map(|x| x.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();
            (winners_vec, sratched_vec)
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>();

    let mut number_of_same_cards: HashMap<u32, u32> = HashMap::new();

    for default_game in games.iter().enumerate() {
        number_of_same_cards.insert(default_game.0 as u32 + 1, 1);
    }
    for (zero_idx, game) in games.iter().enumerate() {
        let mut game_score = 0;
        let hashmap_key_for_game = zero_idx as u32 + 1;
        for winning_number in game.0.iter() {
            if game.1.contains(winning_number) {
                if game_score == 0 {
                    game_score = 1;
                } else {
                    game_score += 1;
                }
            }
        }
        if game_score > 0 {
            let number_of_cards_to_add =
                *number_of_same_cards.get(&(hashmap_key_for_game)).unwrap();
            for hashmap_idx in 1..=game_score {
                if let Some(game_to_add_to) =
                    number_of_same_cards.get_mut(&(hashmap_key_for_game + hashmap_idx))
                {
                    *game_to_add_to += number_of_cards_to_add;
                }
            }
        }
    }

    let mut result_games = 0;

    for value in number_of_same_cards.iter() {
        result_games += value.1;
    }

    Solution::from(result_games)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    const TEST_INPUT_TWO: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(13);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(30_u32);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
