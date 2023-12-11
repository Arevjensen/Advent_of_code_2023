use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "1";

pub fn run(part: &str) {
    let result = match part {
        "1" => part1(loader::read_text_from_file(part, DAY).as_str()),
        "2" => part2(loader::read_text_from_file(part, DAY).as_str()),
        _ => unimplemented!(),
    };
    println!("Solution for part {} on day {} is:", part, DAY);
    println!("{}", result)
}

pub fn part1(input: &str) -> Solution {
    let result = input
        .split('\n')
        .map(|x| {
            x.chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let mut sum_vec = Vec::new();
    for number_string in result.iter() {
        let first = number_string.get(0..1);
        let last = number_string.get(number_string.len() - 1..number_string.len());
        let new_number_string = first.unwrap().to_string() + last.unwrap();
        let summable_value = new_number_string.parse::<u32>().unwrap();
        sum_vec.push(summable_value);
    }

    Solution::from(sum_vec.iter().sum::<u32>())
}

pub fn part2(input: &str) -> Solution {
    let valid_values_vec = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "eigh", "nine",
    ];

    let mut vec_strings = Vec::new();

    for lines in input.lines() {
        let mut build_string = String::new();
        let mut pattern_builer = String::new();
        for char in lines.chars() {
            if char.is_numeric() {
                build_string += char.to_string().as_str();
                pattern_builer = "".to_string();
            } else {
                pattern_builer += char.to_string().as_str();
                if let Some((number_char, value)) =
                    valid_checker(&pattern_builer, &valid_values_vec)
                {
                    build_string += number_char;
                    while pattern_builer.contains(&value) {
                        pattern_builer = pattern_builer[1..].to_string();
                    }
                }
            }
        }
        vec_strings.push(build_string);
    }
    // dbg!(&vec_strings);

    let mut sum_vec = Vec::new();
    for number_string in vec_strings.iter() {
        let first = number_string.get(0..1);
        let last = number_string.get(number_string.len() - 1..number_string.len());
        let new_number_string = first.unwrap().to_string() + last.unwrap();

        let summable_value = new_number_string.parse::<u32>().unwrap();

        sum_vec.push(summable_value);
    }

    Solution::from(sum_vec.iter().sum::<u32>())
}

fn valid_checker(check: &str, values: &Vec<&str>) -> Option<(&'static str, String)> {
    for valid_value in values {
        if check.contains(valid_value) {
            return Some((
                number_string_matcher(valid_value).unwrap(),
                valid_value.to_string(),
            ));
        }
    }
    None
}

fn number_string_matcher(check: &str) -> Option<&'static str> {
    match check {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "eigh" => Some("8"),
        "nine" => Some("9"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const TEST_INPUT_TWO: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(142u32);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(281u32);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
