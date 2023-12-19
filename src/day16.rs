use std::collections::HashSet;

use crate::helpers::loader;
use crate::helpers::solution::Solution;
use crate::helpers::structs::Point2D;

const DAY: &str = "16";

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
struct Grid<'a> {
    height: usize,
    width: usize,
    tiles: &'a str,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn find_next_directions(location: &Position, symbol: char) -> Vec<Position> {
    match symbol {
        '-' => match location.direction {
            Direction::North => vec![
                Position {
                    coords: Point2D {
                        x: location.coords.x - 1,
                        y: location.coords.y,
                    },
                    direction: Direction::West,
                },
                Position {
                    coords: Point2D {
                        x: location.coords.x + 1,
                        y: location.coords.y,
                    },
                    direction: Direction::East,
                },
            ],
            Direction::South => vec![
                Position {
                    coords: Point2D {
                        x: location.coords.x - 1,
                        y: location.coords.y,
                    },
                    direction: Direction::West,
                },
                Position {
                    coords: Point2D {
                        x: location.coords.x + 1,
                        y: location.coords.y,
                    },
                    direction: Direction::East,
                },
            ],
            Direction::East => vec![Position {
                coords: Point2D {
                    x: location.coords.x + 1,
                    y: location.coords.y,
                },
                direction: Direction::East,
            }],
            Direction::West => vec![Position {
                coords: Point2D {
                    x: location.coords.x - 1,
                    y: location.coords.y,
                },
                direction: Direction::West,
            }],
        },
        '|' => match location.direction {
            Direction::North => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y - 1,
                },
                direction: Direction::North,
            }],
            Direction::South => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y + 1,
                },
                direction: Direction::South,
            }],
            Direction::East => vec![
                Position {
                    coords: Point2D {
                        x: location.coords.x,
                        y: location.coords.y + 1,
                    },
                    direction: Direction::South,
                },
                Position {
                    coords: Point2D {
                        x: location.coords.x,
                        y: location.coords.y - 1,
                    },
                    direction: Direction::North,
                },
            ],
            Direction::West => vec![
                Position {
                    coords: Point2D {
                        x: location.coords.x,
                        y: location.coords.y + 1,
                    },
                    direction: Direction::South,
                },
                Position {
                    coords: Point2D {
                        x: location.coords.x,
                        y: location.coords.y - 1,
                    },
                    direction: Direction::North,
                },
            ],
        },
        '/' => match location.direction {
            Direction::North => vec![Position {
                coords: Point2D {
                    x: location.coords.x + 1,
                    y: location.coords.y,
                },
                direction: Direction::East,
            }],
            Direction::South => vec![Position {
                coords: Point2D {
                    x: location.coords.x - 1,
                    y: location.coords.y,
                },
                direction: Direction::West,
            }],
            Direction::East => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y - 1,
                },
                direction: Direction::North,
            }],
            Direction::West => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y + 1,
                },
                direction: Direction::South,
            }],
        },
        '\\' => match location.direction {
            Direction::North => vec![Position {
                coords: Point2D {
                    x: location.coords.x - 1,
                    y: location.coords.y,
                },
                direction: Direction::West,
            }],
            Direction::South => vec![Position {
                coords: Point2D {
                    x: location.coords.x + 1,
                    y: location.coords.y,
                },
                direction: Direction::East,
            }],
            Direction::East => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y + 1,
                },
                direction: Direction::South,
            }],
            Direction::West => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y - 1,
                },
                direction: Direction::North,
            }],
        },
        '.' => match location.direction {
            Direction::North => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y - 1,
                },
                direction: Direction::North,
            }],
            Direction::South => vec![Position {
                coords: Point2D {
                    x: location.coords.x,
                    y: location.coords.y + 1,
                },
                direction: Direction::South,
            }],
            Direction::East => vec![Position {
                coords: Point2D {
                    x: location.coords.x + 1,
                    y: location.coords.y,
                },
                direction: Direction::East,
            }],
            Direction::West => vec![Position {
                coords: Point2D {
                    x: location.coords.x - 1,
                    y: location.coords.y,
                },
                direction: Direction::West,
            }],
        },
        _ => panic!("Unknown symbol: {}", symbol),
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    coords: Point2D<i32>,
    direction: Direction,
}

pub fn part1(input: &str) -> Solution {
    let grid_heigth = input.lines().count();
    let grid_width = input.chars().take_while(|x| *x != '\n').count();
    let input_stripped = input.replace('\n', "");

    let griddy = Grid {
        height: grid_heigth,
        width: grid_width,
        tiles: &input_stripped,
    };

    let mut visited: HashSet<Position> = HashSet::new();

    let mut modified_grid = vec!['.'; grid_heigth * grid_width];

    let start = Position {
        coords: Point2D { x: 0, y: 0 },
        direction: Direction::East,
    };

    let mut upcoming_positions = vec![start.clone()];

    while !upcoming_positions.is_empty() {
        let current_positions = upcoming_positions.clone();
        upcoming_positions.clear();
        for position in current_positions {
            if visited.contains(&position) {
                continue;
            }

            let coords_as_index = position.coords.x + (position.coords.y * grid_width as i32);
            if let Some(grid_location) = input_stripped.chars().nth(coords_as_index as usize) {
                let next_positions = find_next_directions(&position, grid_location);
                // dbg!(&position, &grid_location, &next_positions);
                for valid_next in next_positions.iter() {
                    if valid_next.coords.x < 0
                        || valid_next.coords.y < 0
                        || valid_next.coords.x >= grid_width as i32
                        || valid_next.coords.y >= grid_heigth as i32
                    {
                    } else {
                        upcoming_positions.push(valid_next.clone());
                    }
                }
                visited.insert(position);
            }
        }
    }

    for position in visited {
        let coords = position.coords.x + (position.coords.y * grid_width as i32);

        modified_grid[coords as usize] = '#';
    }

    for y in 0..grid_heigth {
        for x in 0..grid_width {
            print!("{}", modified_grid[x + (y * grid_width)]);
        }
        println!();
    }

    let result = modified_grid.iter().filter(|x| **x == '#').count();
    Solution::from(result)
}

pub fn part2(input: &str) -> Solution {
    let grid_heigth = input.lines().count();
    let grid_width = input.chars().take_while(|x| *x != '\n').count();
    let input_stripped = input.replace('\n', "");

    let griddy = Grid {
        height: grid_heigth,
        width: grid_width,
        tiles: &input_stripped,
    };
    let mut start_positions: Vec<Position> = Vec::new();

    for x in 0..grid_width {
        let down_from_top = Position {
            coords: Point2D { x: x as i32, y: 0 },
            direction: Direction::South,
        };
        start_positions.push(down_from_top);
    }
    for x in 0..grid_width {
        let up_from_bottom = Position {
            coords: Point2D {
                x: x as i32,
                y: (grid_heigth - 1) as i32,
            },
            direction: Direction::North,
        };

        start_positions.push(up_from_bottom);
    }

    for y in 0..grid_heigth {
        let from_left = Position {
            coords: Point2D { x: 0, y: y as i32 },
            direction: Direction::East,
        };
        start_positions.push(from_left);
    }

    for y in 0..grid_heigth {
        let from_right = Position {
            coords: Point2D {
                x: (grid_width - 1) as i32,
                y: y as i32,
            },
            direction: Direction::West,
        };
        start_positions.push(from_right);
    }
    let mut variation_values = Vec::new();

    for start_position in start_positions {
        let mut visited: HashSet<Position> = HashSet::new();

        let mut modified_grid = vec!['.'; grid_heigth * grid_width];

        let mut upcoming_positions = vec![start_position.clone()];

        while !upcoming_positions.is_empty() {
            let current_positions = upcoming_positions.clone();
            upcoming_positions.clear();
            for position in current_positions {
                if visited.contains(&position) {
                    continue;
                }

                let coords_as_index = position.coords.x + (position.coords.y * grid_width as i32);
                if let Some(grid_location) = input_stripped.chars().nth(coords_as_index as usize) {
                    let next_positions = find_next_directions(&position, grid_location);
                    for valid_next in next_positions.iter() {
                        if valid_next.coords.x < 0
                            || valid_next.coords.y < 0
                            || valid_next.coords.x >= grid_width as i32
                            || valid_next.coords.y >= grid_heigth as i32
                        {
                        } else {
                            upcoming_positions.push(valid_next.clone());
                        }
                    }
                    visited.insert(position);
                }
            }
        }

        for position in visited {
            let coords = position.coords.x + (position.coords.y * grid_width as i32);

            modified_grid[coords as usize] = '#';
        }

        let result = modified_grid.iter().filter(|x| **x == '#').count();
        variation_values.push(result);
    }

    Solution::from(*variation_values.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(46_usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(51_usize);
        let my_soultion = part2(TEST_INPUT_TWO);
        assert_eq!(fasit, my_soultion);
    }
}
