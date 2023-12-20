use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "20";

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
    //struct for each module
    //different send and recieve functions? make trait?
    //Keep vec of incoming signals? or outgoing?, read instructions properly
    //Make module 1 send, then 2, then 3,
    // then handle recived pulses
    Solution::from(100)
}

pub fn part2(input: &str) -> Solution {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT_ONE: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST_INPUT_TWO: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[rstest]
    #[case(TEST_INPUT_ONE, 32000000_usize)]
    #[case(TEST_INPUT_TWO, 11687500_usize)]
    fn test_part_1(#[case] input: &str, #[case] result: usize) {
        let fasit = Solution::from(result);
        let part_solution = part1(input);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(200);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
