use clap::Parser;
mod day1;
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
mod helpers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long, default_value_t = 1)]
    day: u8,
    /// Day to run
    #[arg(short, long, default_value_t = String::from("1"))]
    part: String,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::run(args.part.as_str()),
        2 => day2::run(args.part.as_str()),
        3 => day3::run(args.part.as_str()),
        4 => day4::run(args.part.as_str()),
        5 => day5::run(args.part.as_str()),
        6 => day6::run(args.part.as_str()),
        7 => day7::run(args.part.as_str()),
        8 => day8::run(args.part.as_str()),
        9 => day9::run(args.part.as_str()),
        10 => day10::run(args.part.as_str()),
        11 => day11::run(args.part.as_str()),
        12 => day12::run(args.part.as_str()),
        13 => day13::run(args.part.as_str()),
        14 => day14::run(args.part.as_str()),
        15 => day15::run(args.part.as_str()),
        16 => day16::run(args.part.as_str()),
        17 => day17::run(args.part.as_str()),
        18 => day18::run(args.part.as_str()),
        19 => day19::run(args.part.as_str()),
        20 => day20::run(args.part.as_str()),
        21 => day21::run(args.part.as_str()),
        22 => day22::run(args.part.as_str()),
        23 => day23::run(args.part.as_str()),
        24 => day24::run(args.part.as_str()),
        25 => day25::run(args.part.as_str()),
        _ => unimplemented!("Only days between 1-25"),
    }
}
