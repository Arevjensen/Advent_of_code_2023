use geo::{line_string, Area, Coord, LineString, Polygon};
use itertools::Itertools;
use pathfinding::directed::bfs::bfs_reach;
use pathfinding::grid;

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

    let start_point: Point2D<i32> = Point2D { x: 1, y: 1 };

    let inside = bfs_reach(start_point, |x| find_next_inside(x, &lagoon_grid));
    let test = inside.collect::<Vec<Point2D<i32>>>();
    dbg!(&test);

    for y_iter in grid_y_start - 1..=grid_height_zero_indexed + 1 {
        for x_iter in grid_x_start - 1..=grid_width_zero_indexed + 1 {
            if let Some(digged) = lagoon_grid
                .iter()
                .find(|x| x.location.x == x_iter && x.location.y == y_iter)
            {
                print!("#")
            } else {
                if let Some(inside_point) = test.iter().find(|x| x.x == x_iter && x.y == y_iter) {
                    print!("I")
                } else {
                    print!(".")
                }
            }
        }
        println!()
    }

    let result = test.len() + lagoon_grid.len();

    Solution::from(result)
}

fn find_next_inside(point: &Point2D<i32>, lagoon_grid: &Vec<Cube>) -> Vec<Point2D<i32>> {
    let grid_width_zero_indexed = lagoon_grid.iter().map(|x| x.location.x).max().unwrap();
    let grid_x_start = lagoon_grid.iter().map(|x| x.location.x).min().unwrap();
    let grid_height_zero_indexed = lagoon_grid.iter().map(|x| x.location.y).max().unwrap();
    let grid_y_start = lagoon_grid.iter().map(|x| x.location.y).min().unwrap();

    let my_next_points: Vec<Point2D<i32>> =
        vec![point.up(), point.down(), point.left(), point.rigth()];
    let mut testing = Vec::new();
    for possible_next in my_next_points {
        if let Some(exists) = lagoon_grid.iter().find(|x| x.location == possible_next) {
        } else {
            if possible_next.x >= grid_x_start
                && possible_next.x <= grid_width_zero_indexed
                && possible_next.y >= grid_y_start
                && possible_next.y <= grid_height_zero_indexed
            {
                testing.push(possible_next);
            }
        }
    }

    testing
}

#[derive(Debug)]
struct Cube {
    location: Point2D<i32>,
    hex: String,
}

pub fn part2(input: &str) -> Solution {
    let mut lagoon_grid: Vec<Point2D<i32>> = Vec::new();

    let mut location = Point2D { x: 0_i32, y: 0_i32 };

    for instruction in input.lines() {
        let (_, remaining) = instruction.split_once(' ').unwrap();
        let (length, remaining) = remaining
            .split_once(' ')
            .map(|x| (x.0.parse::<i32>().unwrap(), x.1))
            .unwrap();
        let hex = remaining
            .trim_matches(|x| x == '(' || x == ')' || x == '#')
            .to_string();
        let direction = hex.chars().last().unwrap();
        let new_number = u32::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();

        for i in 0..new_number {
            let point = match direction {
                '0' => location.rigth(),
                '1' => location.down(),
                '2' => location.left(),
                '3' => location.up(),
                _ => panic!("Could not find direction from {}", direction),
            };
            lagoon_grid.push(point.clone());
            location = point;
        }
    }

    // for instruction in input.lines() {
    //     let (direction, remaining) = instruction.split_once(' ').unwrap();
    //     let (length, remaining) = remaining
    //         .split_once(' ')
    //         .map(|x| (x.0.parse::<i32>().unwrap(), x.1))
    //         .unwrap();
    //     let hex = remaining
    //         .trim_matches(|x| x == '(' || x == ')' || x == '#')
    //         .to_string();

    //     for i in 1..=length {
    //         let point = match direction {
    //             "D" => location.down(),
    //             "L" => location.left(),
    //             "U" => location.up(),
    //             "R" => location.rigth(),
    //             _ => panic!("Could not find direction from {}", direction),
    //         };
    //         lagoon_grid.push(point.clone());
    //         location = point;
    //     }
    // }

    let grid_width_zero_indexed = lagoon_grid.iter().map(|x| x.x).max().unwrap();
    let grid_x_start = lagoon_grid.iter().map(|x| x.x).min().unwrap();
    let grid_height_zero_indexed = lagoon_grid.iter().map(|x| x.y).max().unwrap();
    let grid_y_start = lagoon_grid.iter().map(|x| x.y).min().unwrap();

    let start_point: Point2D<i32> = Point2D { x: 1, y: 1 };

    let mut test_vec: Vec<Coord> = Vec::new();
    for point in lagoon_grid.iter() {
        test_vec.push(Coord {
            x: point.x as f64,
            y: point.y as f64,
        });
    }
    let linestring = LineString::new(test_vec);

    let polygon = Polygon::new(linestring, vec![]);

    let area = polygon.unsigned_area();
    // dbg!(area, lagoon_grid.len());

    // let mut sums: Vec<u128> = Vec::new();
    // for y_outer in grid_y_start..=grid_height_zero_indexed {
    //     let mut current_line_points = lagoon_grid
    //         .iter()
    //         .filter(|x| x.location.y == y_outer)
    //         .map(|x| &x.location)
    //         .collect::<Vec<&Point2D<i32>>>();

    //     current_line_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));

    //     let mut is_inside = true;
    //     let mut sum = 0_u128;

    //     for (window_a, window_b) in current_line_points.iter().tuple_windows() {
    //         if window_a.x - window_b.x != 1 {
    //             if is_inside {
    //                 sum += ((window_b.x - window_a.x) - 1) as u128;
    //             }
    //             is_inside = !is_inside;
    //         }
    //     }
    //     sums.push(sum);
    // }

    let result = area as usize + ((lagoon_grid.len() + 2) / 2);
    Solution::from(result)
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
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(62_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(952408144115_usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
