use rayon::result;

use crate::helpers::loader;
use crate::helpers::solution::Solution;

const DAY: &str = "11";

pub fn run(part: &str) {
    let result = match part {
        "1" => part1(loader::read_text_from_file(part, DAY).as_str()),
        "2" => part2(loader::read_text_from_file(part, DAY).as_str(), 1000000),
        _ => unimplemented!(),
    };
    println!("Solution for part {} on day {} is:", part, DAY);
    println!("{:?}", result)
}

pub fn part1(input: &str) -> Solution {
    let mut grid_width = input.lines().last().unwrap().len();
    let mut grid_height = input.lines().count();

    let indexes = (1..100000).into_iter();
    let galaxies_iter = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|x| x.1 == '#')
            .map(move |(x, symbol)| Coords { x: x, y: y })
    });

    let galaxies = galaxies_iter
        .zip(indexes)
        .map(|(galaxy, idx)| Galaxy {
            id: idx,
            location: galaxy,
        })
        .collect::<Vec<Galaxy>>();

    let mut galaxies_expanded = galaxies.clone();

    for x in 0..grid_width.clone() {
        if galaxies
            .iter()
            .filter(|galaxy| galaxy.location.x == x)
            .count()
            == 0
        {
            let galaxies_above = galaxies
                .iter()
                .filter(|galaxy| galaxy.location.x > x)
                .map(|x| x.id)
                .collect::<Vec<usize>>();

            galaxies_expanded
                .iter_mut()
                .filter(|galaxy| galaxies_above.contains(&galaxy.id))
                .for_each(|x| x.location.x += 1);
            grid_width += 1;
        }
    }
    for y in 0..grid_height.clone() as usize {
        if galaxies
            .iter()
            .filter(|galaxy| galaxy.location.y == y)
            .count()
            == 0
        {
            let galaxies_above = galaxies
                .iter()
                .filter(|galaxy| galaxy.location.y > y)
                .map(|x| x.id)
                .collect::<Vec<usize>>();
            galaxies_expanded
                .iter_mut()
                .filter(|galaxy| galaxies_above.contains(&galaxy.id))
                .for_each(|x| x.location.y += 1);
            grid_height += 1;
        }
    }
    drop(galaxies);

    let mut vec_of_shortest_paths = Vec::new();

    for galaxy in galaxies_expanded.iter() {
        for i in galaxy.id + 1..=galaxies_expanded.len() {
            let my_coords = galaxy.location;
            let other_galax = galaxies_expanded
                .iter()
                .filter(|x| x.id == i)
                .last()
                .unwrap();

            let x_diff = match my_coords.x >= other_galax.location.x {
                true => my_coords.x - other_galax.location.x,
                false => other_galax.location.x - my_coords.x,
            };

            let y_diff = match my_coords.y >= other_galax.location.y {
                true => my_coords.y - other_galax.location.y,
                false => other_galax.location.y - my_coords.y,
            };

            let tmp = x_diff + y_diff;
            if galaxy.id == 1 {
                dbg!("new outline");
                dbg!(&galaxy, &other_galax, &tmp, x_diff, y_diff);
            }
            vec_of_shortest_paths.push(tmp)
        }
    }
    let result: usize = vec_of_shortest_paths.iter().sum();

    //Print the final grid
    for y in 0..grid_height {
        for x in 0..grid_width {
            let maybe_galaxy = galaxies_expanded
                .iter()
                .filter(|gal| gal.location.x == x && gal.location.y == y)
                .last();
            if let Some(galaxy) = maybe_galaxy {
                print!("{}", galaxy.id);
            } else {
                print!(".")
            }
        }
        println!("");
    }

    Solution::from(result)
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    id: usize,
    location: Coords,
}
#[derive(Debug, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

pub fn part2(input: &str, duplicator: usize) -> Solution {
    let grid_width = input.lines().last().unwrap().len();
    let grid_height = input.lines().count();

    let indexes = (1..10000000).into_iter();
    let galaxies_iter = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|x| x.1 == '#')
            .map(move |(x, symbol)| Coords { x: x, y: y })
    });

    let galaxies = galaxies_iter
        .zip(indexes)
        .map(|(galaxy, idx)| Galaxy {
            id: idx,
            location: galaxy,
        })
        .collect::<Vec<Galaxy>>();

    let mut galaxies_expanded = galaxies.clone();

    for x in 0..grid_width {
        if galaxies
            .iter()
            .filter(|galaxy| galaxy.location.x == x)
            .count()
            == 0
        {
            let galaxies_above = galaxies
                .iter()
                .filter(|galaxy| galaxy.location.x > x)
                .map(|x| x.id)
                .collect::<Vec<usize>>();

            galaxies_expanded
                .iter_mut()
                .filter(|galaxy| galaxies_above.contains(&galaxy.id))
                .for_each(|x| x.location.x += duplicator);
        }
    }
    for y in 0..grid_height {
        if galaxies
            .iter()
            .filter(|galaxy| galaxy.location.y == y)
            .count()
            == 0
        {
            let galaxies_above = galaxies
                .iter()
                .filter(|galaxy| galaxy.location.y > y)
                .map(|x| x.id)
                .collect::<Vec<usize>>();
            galaxies_expanded
                .iter_mut()
                .filter(|galaxy| galaxies_above.contains(&galaxy.id))
                .for_each(|x| x.location.y += duplicator);
        }
    }
    drop(galaxies);

    let mut vec_of_shortest_paths = Vec::new();

    for galaxy in galaxies_expanded.iter() {
        for i in galaxy.id + 1..=galaxies_expanded.len() {
            let my_coords = galaxy.location;
            let other_galax = galaxies_expanded
                .iter()
                .filter(|x| x.id == i)
                .last()
                .unwrap();

            let x_diff = match my_coords.x >= other_galax.location.x {
                true => my_coords.x - other_galax.location.x,
                false => other_galax.location.x - my_coords.x,
            };

            let y_diff = match my_coords.y >= other_galax.location.y {
                true => my_coords.y - other_galax.location.y,
                false => other_galax.location.y - my_coords.y,
            };

            let tmp = x_diff + y_diff;
            // if galaxy.id == 1 {
            //     dbg!("new outline");
            //     dbg!(&galaxy, &other_galax, &tmp, x_diff.abs(), y_diff.abs());
            // }
            vec_of_shortest_paths.push(tmp)
        }
    }
    dbg!(&vec_of_shortest_paths);
    let result: usize = vec_of_shortest_paths.iter().sum::<usize>();

    Solution::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    const TEST_INPUT_TWO: &str = TEST_INPUT_ONE;
    const TEST_INPUT_THREE: &str = TEST_INPUT_ONE;

    #[test]
    fn test_part_1() {
        let fasit = Solution::from(374usize);
        let part_solution = part1(TEST_INPUT_ONE);
        assert_eq!(fasit, part_solution);
    }
    #[test]
    fn test_part_2_one() {
        let fasit = Solution::from(375usize);
        let part_solution = part2(TEST_INPUT_ONE, 1);
        assert_eq!(fasit, part_solution);
    }
    #[test]
    fn test_part_hundred() {
        let fasit = Solution::from(8410usize);
        let my_soultion = part2(TEST_INPUT_TWO, 100);
        assert_eq!(fasit, my_soultion);
    }

    #[test]
    fn test_part_2_ten() {
        let fasit = Solution::from(1030usize);
        let my_soultion = part2(TEST_INPUT_TWO, 10);
        assert_eq!(fasit, my_soultion);
    }
}
