pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod helpers;

pub fn run(day: u8, part: &str) {
    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        4 => day4::run(part),
        5 => day5::run(part),
        6 => day6::run(part),
        7 => day7::run(part),
        8 => day8::run(part),
        9 => day9::run(part),
        10 => day10::run(part),
        11 => day11::run(part),
        12 => day12::run(part),
        13 => day13::run(part),
        14 => day14::run(part),
        15 => day15::run(part),
        16 => day16::run(part),
        17 => day17::run(part),
        18 => day18::run(part),
        19 => day19::run(part),
        20 => day20::run(part),
        21 => day21::run(part),
        22 => day22::run(part),
        23 => day23::run(part),
        24 => day24::run(part),
        25 => day25::run(part),
        _ => unimplemented!("Only days between 1-25"),
    }
}
