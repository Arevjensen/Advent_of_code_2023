use pathfinding::directed::dijkstra::dijkstra;

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
        .map(|x| path_test(x.clone(), goal.clone(), &grid))
        .inspect(|x| {
            dbg!(&x);
        })
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

fn path_test(
    start: (Point2D<usize>, (Direction, usize)),
    goal: Point2D<usize>,
    grid: &Grid,
) -> usize {
    let x = dijkstra(
        &start,
        |x| successors(&x.0, x.clone().1, grid),
        |x| x.0 == goal,
    );
    if let Some(map) = x {
        // dbg!(&map);
        let mut sum = 0;
        for y_loop in 0..grid.height {
            for x_loop in 0..grid.width {
                let coords = map.0.iter().find(|x| x.0.x == x_loop && x.0.y == y_loop);
                if let Some(value) = coords {
                    let idx = Point2D {
                        x: x_loop,
                        y: y_loop,
                    };
                    let location_value = grid
                        .tiles
                        .chars()
                        .nth(idx.to_grid_index(grid.width, grid.height).unwrap())
                        .unwrap()
                        .to_digit(10)
                        .unwrap() as usize;
                    sum += location_value;

                    print!(
                        "{}",
                        match value.1 .0 {
                            Direction::Up => "^",
                            Direction::Down => "v",
                            Direction::Left => "<",
                            Direction::Right => ">",
                        }
                    );
                } else {
                    let idx = Point2D {
                        x: x_loop,
                        y: y_loop,
                    };
                    print!(
                        "{}",
                        grid.tiles
                            .chars()
                            .nth(idx.to_grid_index(grid.width, grid.height).unwrap())
                            .unwrap()
                    );
                }
            }
            println!()
        }
        println!("Splitter");
        return sum;
    }
    10000000
}

fn successors(
    point: &Point2D<usize>,
    diretion: (Direction, usize),
    grid: &Grid,
) -> Vec<((Point2D<usize>, (Direction, usize)), usize)> {
    let next_locations = next_locations(&point, &diretion, grid)
        .iter()
        .filter(|x| x.0.is_some())
        .map(|x| (x.clone().0.unwrap(), x.clone().1))
        .map(|x| {
            let location_value = grid
                .tiles
                .chars()
                .nth(x.0.to_grid_index(grid.width, grid.height).unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;

            (x, location_value)
        })
        .collect();

    next_locations
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
                    (location.up(), (Direction::Up, 1)),
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
                    (
                        location.right_bounded(input.width),
                        (direction.0.clone(), direction.1 + 1),
                    ),
                    (location.up(), (Direction::Up, 1)),
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

    let start = Point2D {
        x: 0_usize,
        y: 0_usize,
    };

    let next_directions = [
        (start.down().unwrap(), (Direction::Down, 2_usize)),
        (start.right().unwrap(), (Direction::Right, 2_usize)),
    ];

    let result = next_directions
        .iter()
        .map(|x| path_test_2(x.clone(), goal.clone(), &grid))
        .min()
        .unwrap();

    Solution::from(result)
}
fn path_test_2(
    start: (Point2D<usize>, (Direction, usize)),
    goal: Point2D<usize>,
    grid: &Grid,
) -> usize {
    let x = dijkstra(
        &start,
        |x| successors_2(&x.0, x.clone().1, grid),
        |x| x.0 == goal && x.1 .1 >= 4,
    );
    if let Some(map) = x {
        // dbg!(&map);
        let mut sum = 0;
        for y_loop in 0..grid.height {
            for x_loop in 0..grid.width {
                let coords = map.0.iter().find(|x| x.0.x == x_loop && x.0.y == y_loop);
                if let Some(value) = coords {
                    let idx = Point2D {
                        x: x_loop,
                        y: y_loop,
                    };
                    let location_value = grid
                        .tiles
                        .chars()
                        .nth(idx.to_grid_index(grid.width, grid.height).unwrap())
                        .unwrap()
                        .to_digit(10)
                        .unwrap() as usize;
                    sum += location_value;

                    print!(
                        "#",
                        // match value.1 .0 {
                        //     Direction::Up => "^",
                        //     Direction::Down => "v",
                        //     Direction::Left => "<",
                        //     Direction::Right => ">",
                        // }
                    );
                } else {
                    let idx = Point2D {
                        x: x_loop,
                        y: y_loop,
                    };
                    print!(
                        ".",
                        // grid.tiles
                        //     .chars()
                        //     .nth(idx.to_grid_index(grid.width, grid.height).unwrap())
                        //     .unwrap()
                    );
                }
            }
            println!()
        }
        println!("Splitter");
        println!("{}", sum);
        return sum;
    }
    10000000
}
fn successors_2(
    point: &Point2D<usize>,
    diretion: (Direction, usize),
    grid: &Grid,
) -> Vec<((Point2D<usize>, (Direction, usize)), usize)> {
    let next_locations = next_locations_2(&point, &diretion, grid)
        .iter()
        .filter(|x| x.0.is_some())
        .map(|x| (x.clone().0.unwrap(), x.clone().1))
        .map(|x| {
            let location_value = grid
                .tiles
                .chars()
                .nth(x.0.to_grid_index(grid.width, grid.height).unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;

            (x, location_value)
        })
        .collect();

    next_locations
}

fn next_locations_2(
    location: &Point2D<usize>,
    direction: &(Direction, usize),
    input: &Grid,
) -> Vec<(Option<Point2D<usize>>, (Direction, usize))> {
    match direction.0 {
        Direction::Up => {
            if direction.1 < 4 {
                vec![(location.up(), (direction.0.clone(), direction.1 + 1))]
            } else if direction.1 == 10 {
                vec![
                    (location.left(), (Direction::Left, 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            } else {
                vec![
                    (location.left(), (Direction::Left, 1)),
                    (location.up(), (direction.0.clone(), direction.1 + 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            }
        }
        Direction::Down => {
            if direction.1 == 10 {
                vec![
                    (location.left(), (Direction::Left, 1)),
                    (location.right_bounded(input.width), (Direction::Right, 1)),
                ]
            } else if direction.1 < 4 {
                vec![(
                    location.down_bounded(input.height),
                    (direction.0.clone(), direction.1 + 1),
                )]
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
            if direction.1 == 10 {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.up(), (Direction::Up, 1)),
                ]
            } else if direction.1 < 4 {
                vec![(location.left(), (direction.0.clone(), direction.1 + 1))]
            } else {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.left(), (direction.0.clone(), direction.1 + 1)),
                    (location.up(), (Direction::Up, 1)),
                ]
            }
        }
        Direction::Right => {
            if direction.1 == 10 {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (location.up(), (Direction::Up, 1)),
                ]
            } else if direction.1 < 4 {
                vec![(
                    location.right_bounded(input.width),
                    (direction.0.clone(), direction.1 + 1),
                )]
            } else {
                vec![
                    (location.down_bounded(input.height), (Direction::Down, 1)),
                    (
                        location.right_bounded(input.width),
                        (direction.0.clone(), direction.1 + 1),
                    ),
                    (location.up(), (Direction::Up, 1)),
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT_ONE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    // const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;
    const TEST_INPUT_TWO: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(102_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[rstest]
    #[case(TEST_INPUT_ONE, 94_usize)]
    #[case(TEST_INPUT_TWO, 71_usize)]
    fn test_part_2(#[case] input: &str, #[case] solution: usize) {
        let fasit = Solution::from(solution);
        let my_soultion = part2(input);
        assert_eq!(fasit, my_soultion);
    }
}
