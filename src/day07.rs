use std::collections::HashMap;
use std::str::FromStr;

use nom::InputIter;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "7";

pub fn run(part: &str) {
    let result = match part {
        "1" => part1(loader::read_text_from_file(part, DAY).as_str()),
        "2" => part2(loader::read_text_from_file(part, DAY).as_str()),
        _ => unimplemented!(),
    };
    println!("Solution for part {} on day {} is:", part, DAY);
    println!("{:?}", result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P" => Ok(Self::Joker),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err("Letter not found".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for HandType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut jokers = 0;
        let hash_map = s.chars().fold(HashMap::new(), |mut map, c| {
            if c == 'P' {
                jokers += 1;
            } else {
                map.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
            map
        });

        let mut card_counts_for_hand = Vec::from_iter(hash_map.values());
        card_counts_for_hand.sort_by(|a, b| b.cmp(&a));

        let highest = *card_counts_for_hand.get(0).unwrap_or(&&0);
        let highest = highest + jokers;
        let second = *card_counts_for_hand.get(1).unwrap_or(&&0);

        match (highest, second) {
            (1, _) => Ok(Self::HighCard),
            (2, 2) => Ok(Self::TwoPairs),
            (2, _) => Ok(Self::OnePair),
            (3, 2) => Ok(Self::FullHouse),
            (3, _) => Ok(Self::ThreeOfAKind),
            (4, _) => Ok(Self::FourOfAKind),
            (5, _) => Ok(Self::FiveOfAKind),
            _ => Err("Somehow manged to get six of a kind, or no cards".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                match self_card.cmp(other_card) {
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                }
            }
            return std::cmp::Ordering::Equal;
        }
        if self.hand_type > other.hand_type {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> Solution {
    let mut hands = input
        .lines()
        .map(|x| {
            let (cards, bid) = x.split_once(" ").unwrap();

            let cards_vec = cards
                .chars()
                .map(|x| x.to_string().parse::<Card>().unwrap())
                .collect::<Vec<Card>>();
            let bid_number = bid.parse::<usize>().unwrap();
            let hand_type = cards.parse::<HandType>().unwrap();

            Hand {
                cards: cards_vec,
                hand_type,
                bid: bid_number,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_unstable();

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, val| acc + (val.1.bid * (val.0 + 1)));

    Solution::from(result)
}

pub fn part2(input: &str) -> Solution {
    let mut hands = input
        .lines()
        .map(|x| {
            let (cards, bid) = x.split_once(" ").unwrap();

            let cards_vec = cards
                .replace("J", "P")
                .chars()
                .map(|x| x.to_string().parse::<Card>().unwrap())
                .collect::<Vec<Card>>();
            let bid_number = bid.parse::<usize>().unwrap();
            let hand_type = cards.replace("J", "P").parse::<HandType>().unwrap();

            Hand {
                cards: cards_vec,
                hand_type,
                bid: bid_number,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_unstable();

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, val| acc + (val.1.bid * (val.0 + 1)));

    Solution::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(6440usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(5905usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
