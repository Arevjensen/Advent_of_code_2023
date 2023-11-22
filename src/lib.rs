pub mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
