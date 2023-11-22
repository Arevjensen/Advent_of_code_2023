use clap::Parser;
mod day1;
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
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        6 => todo!(),
        7 => todo!(),
        8 => todo!(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        13 => todo!(),
        14 => todo!(),
        15 => todo!(),
        16 => todo!(),
        17 => todo!(),
        18 => todo!(),
        19 => todo!(),
        20 => todo!(),
        21 => todo!(),
        22 => todo!(),
        23 => todo!(),
        24 => todo!(),
        _ => unimplemented!("Only days between 1-24"),
    }
}
