use std::collections::HashMap;

use crate::helpers::loader;
use crate::helpers::solution::Solution;
use crate::helpers::structs::Point2D;

const DAY: &str = "17";

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
    let grid_width = input.split_once('\n').unwrap().0.len();
    let grid_height = input.lines().count();
    let input = input.replace("\n", "");

    let grid = Grid {
        height: grid_height,
        width: grid_width,
        tiles: input.as_str(),
    };
    let goal = Point2D {
        x: grid_width - 1,
        y: grid_height - 1,
    };
    let mut cache: HashMap<(Point2D<usize>, (Direction, usize)), usize> = HashMap::new();

    let start = Point2D {
        x: 0_usize,
        y: 0_usize,
    };

    let next_directions = [
        (start.down().unwrap(), (Direction::Down, 1_usize)),
        (start.right().unwrap(), (Direction::Right, 1_usize)),
    ];

    let result = next_directions
        .iter()
        .map(|x| recursive_pathfinder(x.clone().0, &goal, 0, x.clone().1, &grid, &mut cache))
        .map(|x| x.2)
        .min()
        .unwrap();

    Solution::from(result)
}
#[derive(Debug)]
struct Grid<'a> {
    height: usize,
    width: usize,
    tiles: &'a str,
}

fn recursive_pathfinder(
    location: Point2D<usize>,
    final_goal: &Point2D<usize>,
    sum_so_far: usize,
    direction: (Direction, usize),
    input: &Grid,
    cache: &mut HashMap<(Point2D<usize>, (Direction, usize)), usize>,
) -> (Point2D<usize>, (Direction, usize), usize) {
    if let Some(seen) = cache.get(&(location.clone(), direction.clone())) {
        return (location, direction, *seen);
    }

    let location_value = input
        .tiles
        .chars()
        .nth(location.to_grid_index(input.width, input.height).unwrap())
        .unwrap()
        .to_digit(10);
    let location_value = location_value.unwrap() as usize;

    let next_location_options = next_locations(&location, &direction, input);
    let valid_nexts: Vec<(Point2D<usize>, (Direction, usize))> = next_location_options
        .iter()
        .filter(|x| x.0.is_some())
        .map(|x| (x.0.clone().unwrap(), x.1.clone()))
        .collect();

    if let Some(goal) = valid_nexts.iter().find(|x| x.0 == *final_goal) {
        let goal_value = input
            .tiles
            .chars()
            .nth(final_goal.to_grid_index(input.width, input.height).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let result = sum_so_far + location_value + goal_value;
        cache.insert((location.clone(), direction.clone()), result);
        return (location, direction, result);
    }

    let mut result_of_exits: Vec<usize> = Vec::new();
    let sum_to_send = sum_so_far + location_value;
    dbg!(&valid_nexts);
    for exit in valid_nexts {
        if let Some(seen) = cache.get(&(exit.clone().0, exit.clone().1)) {
            result_of_exits.push(*seen);
        } else {
            let exit_result =
                recursive_pathfinder(exit.0, final_goal, sum_to_send, exit.1, input, cache);
            // cache.insert(
            //     (exit_result.clone().0, exit_result.clone().1),
            //     exit_result.2,
            // );

            result_of_exits.push(exit_result.2);
        }
    }
    let shortest_from_location = *result_of_exits.iter().min().unwrap_or(&10000000000_usize);
    cache.insert(
        (location.clone(), direction.clone()),
        shortest_from_location,
    );
    (location, direction, shortest_from_location)
}

fn next_locations(
    location: &Point2D<usize>,
    direction: &(Direction, usize),
    input: &Grid,
) -> Vec<(Option<Point2D<usize>>, (Direction, usize))> {
    match direction.0 {
        Direction::Up => {
            if direction.1 == 3 {
                vec![
                    (location.left(), (Direction::Left, 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            } else {
                vec![
                    (location.up(), (direction.0.clone(), direction.1 + 1)),
                    (location.left(), (Direction::Left, 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            }
        }
        Direction::Down => {
            if direction.1 == 3 {
                vec![
                    (location.left(), (Direction::Left, 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            } else {
                vec![
                    (
                        location.down_bounded(input.height),
                        (direction.0.clone(), direction.1 + 1),
                    ),
                    (location.left(), (Direction::Left, 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            }
        }
        Direction::Left => {
            if direction.1 == 3 {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.up(), (Direction::Up, 1)),
                ]
            } else {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.left(), (direction.0.clone(), direction.1 + 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            }
        }
        Direction::Right => {
            if direction.1 == 3 {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.up(), (Direction::Up, 1)),
                ]
            } else {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.right(), (direction.0.clone(), direction.1 + 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part2(input: &str) -> Solution {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"2413432311323
3215453535623";
    // 3255245654254
    // 3446585845452
    // 4546657867536
    // 1438598798454
    // 4457876987766
    // 3637877979653
    // 4654967986887
    // 4564679986453
    // 1224686865563
    // 2546548887735
    // 4322674655533";
    const TEST_INPUT_TWO: &str = r"";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(102);
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
