use crate::helpers::loader;
use crate::helpers::solution::Solution;
use itertools::{self, Itertools};

const DAY: &str = "13";

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
    let x = input.split("\n\n");
    let mut result_horizontal = 0;
    let mut result_vertical = 0;

    for map in x {
        let mirror = find_mirror_part_1(map);
        match mirror.0 {
            MirrorDirection::Vertical => result_vertical += mirror.1,
            MirrorDirection::Horizontal => result_horizontal += mirror.1,
        }
    }

    let result = result_vertical + (result_horizontal * 100);
    Solution::from(result)
}

#[derive(Debug)]
enum MirrorDirection {
    Vertical,
    Horizontal,
}

fn find_mirror_part_1(input: &str) -> (MirrorDirection, usize) {
    let (vert, horizontal) = (find_vertical_part_1(input), find_horizontal_part_1(input));

    match (vert, horizontal) {
        (None, None) => panic!("Found not mirroring at all"),
        (None, Some(value)) => (MirrorDirection::Horizontal, value),
        (Some(value), None) => (MirrorDirection::Vertical, value),
        (Some(_), Some(_)) => {
            panic!("Found valid mirroring in both horizontal and vertical direction")
        }
    }
}

fn find_vertical_part_1(input: &str) -> Option<usize> {
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

    columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|(la, lb)| la.1 == lb.1)
        .find_map(|((idx_a, la), (idx_b, lb))| {
            let lines_above_a = columns[..=idx_a].iter().rev();
            let lines_below_b = columns[idx_b..].iter();

            lines_above_a
                .clone()
                .zip(lines_below_b)
                .all(|(line_above, line_below)| line_above == line_below)
                .then_some(idx_a + 1)
        })
}

fn find_horizontal_part_1(input: &str) -> Option<usize> {
    let map = input.lines().collect::<Vec<_>>();
    input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|(la, lb)| la.1 == lb.1)
        .find_map(|((idx_a, la), (idx_b, lb))| {
            let lines_above_a = map[..=idx_a].iter().rev();
            let lines_below_b = map[idx_b..].iter();

            lines_above_a
                .clone()
                .zip(lines_below_b)
                .all(|(line_above, line_below)| line_above == line_below)
                .then_some(idx_a + 1)
        })
}

pub fn part2(input: &str) -> Solution {
    let x = input.split("\n\n");
    let mut result_horizontal = 0;
    let mut result_vertical = 0;

    for map in x {
        let mirror = find_mirror_part_2(map);
        match mirror.0 {
            MirrorDirection::Vertical => result_vertical += mirror.1,
            MirrorDirection::Horizontal => result_horizontal += mirror.1,
        }
    }

    let result = result_vertical + (result_horizontal * 100);
    Solution::from(result)
}

fn find_mirror_part_2(input: &str) -> (MirrorDirection, usize) {
    let (vert, horizontal) = (find_vertical_part_2(input), find_horizontal_part_2(input));

    match (vert, horizontal) {
        (None, None) => panic!("Found not mirroring at all"),
        (None, Some(value)) => (MirrorDirection::Horizontal, value),
        (Some(value), None) => (MirrorDirection::Vertical, value),
        (Some(_), Some(_)) => {
            panic!("Found valid mirroring in both horizontal and vertical direction")
        }
    }
}

fn find_vertical_part_2(input: &str) -> Option<usize> {
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

    columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|(la, lb)| {
            let diff = la.1.iter().zip(lb.1.iter()).filter(|(a, b)| a != b).count();

            diff <= 1
        })
        .find_map(|((idx_a, la), (idx_b, lb))| {
            let lines_above_a = columns[..=idx_a].iter().rev();
            let lines_below_b = columns[idx_b..].iter();

            (lines_above_a
                .flatten()
                .zip(lines_below_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(idx_a + 1)
        })
}

fn find_horizontal_part_2(input: &str) -> Option<usize> {
    let map = input.lines().collect::<Vec<_>>();
    input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|(la, lb)| {
            let diff =
                la.1.chars()
                    .zip(lb.1.chars())
                    .filter(|(a, b)| a != b)
                    .count();

            diff <= 1
        })
        .find_map(|((idx_a, la), (idx_b, lb))| {
            let lines_above_a = map[..=idx_a].iter().map(|x| x.chars()).rev();
            let lines_below_b = map[idx_b..].iter().map(|x| x.chars());

            (lines_above_a
                .flatten()
                .zip(lines_below_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(idx_a + 1)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(405_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(400_usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
