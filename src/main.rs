use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;

mod y2024;
mod y2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    ///data
    #[arg(long)]
    data: PathBuf,
    ///day
    #[arg(long)]
    day: u8,
    /// year
    #[arg(long, default_value = "2025")]
    year: u16,
    ///verbosity
    #[arg(long, short, default_value = "1")]
    verbosity: u8,
}

pub type Result<T> = std::result::Result<T, AOCError>;
#[derive(Error, Debug)]
pub enum AOCError {
    #[error("could not parse {0}")]
    ParseError(String),
    #[error("file handling failed {0}")]
    IOError(#[from] std::io::Error),
    #[error("something unexpected happened {0}")]
    GenError(String),
    #[error("solver failed with {0}")]
    SolverError(String),
}

fn main() -> Result<()> {
    let args = Args::parse();
    match (args.year, args.day) {
        (2024, 1) => y2024::day1::_main(args.data, args.verbosity),
        (2024, 2) => y2024::day2::_main(args.data, args.verbosity),
        (2024, 3) => y2024::day3::_main(args.data, args.verbosity),
        (2024, 4) => y2024::day4::_main(args.data, args.verbosity),
        (2024, 5) => y2024::day5::_main(args.data, args.verbosity),
        (2024, 6) => y2024::day6::_main(args.data, args.verbosity),
        (2024, 7) => y2024::day7::_main(args.data, args.verbosity),
        (2024, 8) => y2024::day8::_main(args.data, args.verbosity),
        (2024, 9) => y2024::day9::_main(args.data, args.verbosity),
        (2024, 10) => y2024::day10::_main(args.data, args.verbosity),
        (2024, 11) => y2024::day11::_main(args.data, args.verbosity),
        (2024, 12) => y2024::day12::_main(args.data, args.verbosity),
        (2024, 13) => y2024::day13::_main(args.data, args.verbosity),
        (2024, 14) => y2024::day14::_main(args.data, args.verbosity),
        (2024, 15) => y2024::day15::_main(args.data, args.verbosity),
        (2024, 16) => y2024::day16::_main(args.data, args.verbosity),
        (2024, 17) => y2024::day17::_main(args.data, args.verbosity),
        (2024, 18) => y2024::day18::_main(args.data, args.verbosity),
        (2024, 19) => y2024::day19::_main(args.data, args.verbosity),
        (2024, 20) => y2024::day20::_main(args.data, args.verbosity),
        (2024, 21) => y2024::day21::_main(args.data, args.verbosity),
        (2024, 22) => y2024::day22::_main(args.data, args.verbosity),
        (2024, 23) => y2024::day23::_main(args.data, args.verbosity),
        (2024, 24) => y2024::day24::_main(args.data, args.verbosity),
        (2024, 25) => y2024::day25::_main(args.data, args.verbosity),
        (2025, 1) => y2025::day1::_main(args.data, args.verbosity),
        (2025, 2) => y2025::day2::_main(args.data, args.verbosity),
        (2025, 3) => y2025::day3::_main(args.data, args.verbosity),
        (2025, 4) => y2025::day4::_main(args.data, args.verbosity),
        _ => Err(AOCError::GenError("Not implemented".into())),
    }?;
    Ok(())
}
