use crate::helpers::loader;
use crate::helpers::solution::Solution;
use crate::helpers::structs::Point2D;

const DAY: &str = "18";

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
    let mut lagoon_grid: Vec<Cube> = Vec::new();

    let mut location = Point2D { x: 0_i32, y: 0_i32 };

    for instruction in input.lines() {
        let (direction, remaining) = instruction.split_once(' ').unwrap();
        let (length, remaining) = remaining
            .split_once(' ')
            .map(|x| (x.0.parse::<i32>().unwrap(), x.1))
            .unwrap();
        let hex = remaining
            .trim_matches(|x| x == '(' || x == ')' || x == '#')
            .to_string();

        for i in 1..=length {
            let point = match direction {
                "D" => location.down(),
                "L" => location.left(),
                "U" => location.up(),
                "R" => location.rigth(),
                _ => panic!("Could not find direction from {}", direction),
            };
            let new_cube = Cube {
                location: point.clone(),
                hex: hex.clone(),
            };
            lagoon_grid.push(new_cube);
            location = point;
        }
    }

    let grid_width_zero_indexed = lagoon_grid.iter().map(|x| x.location.x).max().unwrap();
    let grid_x_start = lagoon_grid.iter().map(|x| x.location.x).min().unwrap();
    let grid_height_zero_indexed = lagoon_grid.iter().map(|x| x.location.y).max().unwrap();
    let grid_y_start = lagoon_grid.iter().map(|x| x.location.y).min().unwrap();

    for y_iter in grid_y_start - 1..=grid_height_zero_indexed + 1 {
        for x_iter in grid_x_start - 1..=grid_width_zero_indexed + 1 {
            if let Some(digged) = lagoon_grid
                .iter()
                .find(|x| x.location.x == x_iter && x.location.y == y_iter)
            {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }

    Solution::from(1)
}

#[derive(Debug)]
struct Cube {
    location: Point2D<i32>,
    hex: String,
}

pub fn part2(input: &str) -> Solution {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    const TEST_INPUT_TWO: &str = r"";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(100);
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
