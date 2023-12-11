use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "10";

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
    let width_and_legth_finder_helper = input
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .last()
        .unwrap();

    let grid_width = width_and_legth_finder_helper.1.len();
    let grid_heigth = width_and_legth_finder_helper.0 + 1;

    let mut start_coords = Coords { x: 0, y: 0 };

    let grid = input
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|line| {
            line.1
                .chars()
                .enumerate()
                .map(|line_chars| {
                    let tile_type = TileType::from_str(line_chars.1.to_string().as_str()).unwrap();
                    let coords = Coords {
                        x: line_chars.0 as isize,
                        y: line.0 as isize,
                    };
                    if tile_type == TileType::Start {
                        start_coords = coords.clone();
                    }
                    (coords, tile_type)
                })
                .collect::<HashMap<Coords, TileType>>()
        })
        .collect::<HashMap<Coords, TileType>>();

    let mut all_paths = Vec::new();
    for start_neighbour in TileType::Start.allowed_neighbours(&start_coords) {
        if let Some(coords_exist) = grid.get(&start_neighbour) {
            if check_allowed_entry(&start_coords, &start_neighbour, &grid) {
                let mut coords_in_direction = HashSet::new();
                coords_in_direction.insert(start_neighbour.clone());
                for next in coords_exist.allowed_neighbours(&start_neighbour) {
                    recursive_path_finder(
                        &start_neighbour,
                        &&next,
                        &mut coords_in_direction,
                        &grid,
                        &start_coords,
                    )
                }
                all_paths.push(coords_in_direction);
            }
        }
    }

    let paths_with_start = all_paths
        .iter()
        .filter(|x| x.contains(&start_coords))
        .collect::<Vec<&HashSet<Coords>>>();

    if paths_with_start.len() != 2 {
        dbg!(&paths_with_start);
    }

    let result = paths_with_start.first().unwrap().len() / 2;

    let printing = paths_with_start.first().unwrap();
    for y in 0..grid_heigth {
        for x in 0..grid_width {
            let coords = Coords {
                x: x as isize,
                y: y as isize,
            };
            if printing.contains(&coords) {
                print!("{}", grid.get(&coords).unwrap())
            } else {
                print!(".")
            }
        }
        println!()
    }

    Solution::from(result)
}

fn recursive_path_finder(
    current: &Coords,
    destination: &Coords,
    coords_so_far: &mut HashSet<Coords>,
    grid: &HashMap<Coords, TileType>,
    goal: &Coords,
) {
    if check_allowed_entry(&current, &destination, &grid) {
        coords_so_far.insert(destination.clone());
        for next_desination in grid
            .get(destination)
            .unwrap()
            .allowed_neighbours(destination)
        {
            if check_allowed_entry(&destination, &next_desination, &grid)
                && !coords_so_far.contains(&next_desination)
            {
                recursive_path_finder(destination, &next_desination, coords_so_far, grid, goal);
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coords {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl TileType {
    pub fn allowed_neighbours(&self, coords: &Coords) -> Vec<Coords> {
        match self {
            TileType::Vertical => [
                Coords {
                    x: coords.x,
                    y: coords.y - 1,
                },
                Coords {
                    x: coords.x,
                    y: coords.y + 1,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
            TileType::Horizontal => [
                Coords {
                    x: coords.x + 1,
                    y: coords.y,
                },
                Coords {
                    x: coords.x - 1,
                    y: coords.y,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
            TileType::NorthEast => [
                Coords {
                    x: coords.x,
                    y: coords.y - 1,
                },
                Coords {
                    x: coords.x + 1,
                    y: coords.y,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
            TileType::NorthWest => [
                Coords {
                    x: coords.x,
                    y: coords.y - 1,
                },
                Coords {
                    x: coords.x - 1,
                    y: coords.y,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
            TileType::SouthWest => [
                Coords {
                    x: coords.x,
                    y: coords.y + 1,
                },
                Coords {
                    x: coords.x - 1,
                    y: coords.y,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
            TileType::SouthEast => [
                Coords {
                    x: coords.x,
                    y: coords.y + 1,
                },
                Coords {
                    x: coords.x + 1,
                    y: coords.y,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
            TileType::Ground => vec![],
            TileType::Start => [
                Coords {
                    x: coords.x + 1,
                    y: coords.y,
                },
                Coords {
                    x: coords.x - 1,
                    y: coords.y,
                },
                Coords {
                    x: coords.x,
                    y: coords.y + 1,
                },
                Coords {
                    x: coords.x,
                    y: coords.y - 1,
                },
            ]
            .iter()
            .filter(|x| *x != coords)
            .cloned()
            .collect(),
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Vertical => write!(f, "|"),
            TileType::Horizontal => write!(f, "-"),
            TileType::NorthEast => write!(f, "L"),
            TileType::NorthWest => write!(f, "J"),
            TileType::SouthWest => write!(f, "7"),
            TileType::SouthEast => write!(f, "F"),
            TileType::Ground => write!(f, "."),
            TileType::Start => write!(f, "S"),
        }
    }
}

impl FromStr for TileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::Vertical),
            "-" => Ok(Self::Horizontal),
            "L" => Ok(Self::NorthEast),
            "J" => Ok(Self::NorthWest),
            "7" => Ok(Self::SouthWest),
            "F" => Ok(Self::SouthEast),
            "S" => Ok(Self::Start),
            "." => Ok(Self::Ground),
            x => Err(format!("Could not match char | {} |to tile type", x)),
        }
    }
}

fn check_allowed_entry(
    source: &Coords,
    destination: &Coords,
    grid: &HashMap<Coords, TileType>,
) -> bool {
    if let Some(destination_value) = grid.get(destination) {
        if destination_value
            .allowed_neighbours(destination)
            .contains(&&source)
        {
            return true;
        }
    }
    false
}

pub fn part2(input: &str) -> Solution {
    let width_and_legth_finder_helper = input
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .last()
        .unwrap();

    let grid_width = width_and_legth_finder_helper.1.len();
    let grid_heigth = width_and_legth_finder_helper.0 + 1;

    let mut start_coords = Coords { x: 0, y: 0 };

    let grid = input
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|line| {
            line.1
                .chars()
                .enumerate()
                .map(|line_chars| {
                    let tile_type = TileType::from_str(line_chars.1.to_string().as_str()).unwrap();
                    let coords = Coords {
                        x: line_chars.0 as isize,
                        y: line.0 as isize,
                    };
                    if tile_type == TileType::Start {
                        start_coords = coords.clone();
                    }
                    (coords, tile_type)
                })
                .collect::<HashMap<Coords, TileType>>()
        })
        .collect::<HashMap<Coords, TileType>>();

    let mut all_paths = Vec::new();
    for start_neighbour in TileType::Start.allowed_neighbours(&start_coords) {
        if let Some(coords_exist) = grid.get(&start_neighbour) {
            if check_allowed_entry(&start_coords, &start_neighbour, &grid) {
                let mut coords_in_direction = HashSet::new();
                coords_in_direction.insert(start_neighbour.clone());
                for next in coords_exist.allowed_neighbours(&start_neighbour) {
                    recursive_path_finder(
                        &start_neighbour,
                        &&next,
                        &mut coords_in_direction,
                        &grid,
                        &start_coords,
                    )
                }
                all_paths.push(coords_in_direction);
            }
        }
    }

    let paths_with_start = all_paths
        .iter()
        .filter(|x| x.contains(&start_coords))
        .collect::<Vec<&HashSet<Coords>>>();

    if paths_with_start.len() != 2 {
        dbg!(&paths_with_start);
    }

    for y in 0..grid_heigth {
        for x in 0..grid_width {}
    }

    let mut result = 0;
    let mut test_grid = HashSet::new();
    let maze = paths_with_start.first().unwrap();

    for tile in grid.keys() {
        if !maze.contains(tile) {
            let mut is_inside = false;
            for x in -1..=tile.x as isize {
                let check_coords = Coords { x: x, y: tile.y };
                if let Some(tube_piece) = maze.get(&check_coords) {
                    let tile_type = grid.get(tube_piece).unwrap();
                    match tile_type {
                        TileType::Horizontal => {}
                        TileType::NorthEast => {}
                        TileType::NorthWest => {}
                        TileType::Start => {}
                        _ => is_inside = !is_inside,
                    }
                }
            }
            if is_inside {
                test_grid.insert(tile);
                result += 1;
            }
        }
    }

    let printing = paths_with_start.first().unwrap();
    for y in 0..grid_heigth {
        for x in 0..grid_width {
            let coords = Coords {
                x: x as isize,
                y: y as isize,
            };
            if printing.contains(&coords) {
                print!("{}", grid.get(&coords).unwrap())
            } else if test_grid.contains(&coords) {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!()
    }

    Solution::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;
    const TEST_INPUT_TWO: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    const TEST_INPUT_TREE: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    #[test]
    fn test_part_1() {
        let fasit = Solution::from(4usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }

    #[test]
    fn test_part_2() {
        let fasit = Solution::from(10);
        let my_soultion = part2(TEST_INPUT_TREE);
        assert_eq!(fasit, my_soultion);
    }
}
