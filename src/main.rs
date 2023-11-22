use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long, default_value_t = 1)]
    day: u8,
    /// Part of day to run
    #[arg(short, long, default_value_t = String::from("1"))]
    part: String,
}

fn main() {
    let args = Args::parse();
    aoc_2023::run(args.day, &args.part)
}
