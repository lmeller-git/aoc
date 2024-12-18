use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;
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
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    ///data
    #[arg(long)]
    data: PathBuf,
    //out
    #[arg(long, default_value = "out/res.txt")]
    out: PathBuf,
    ///day
    #[arg(long)]
    day: u8,
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
    match args.day {
        1 => day1::_main(args.data, args.out),
        2 => day2::_main(args.data, args.out),
        3 => day3::_main(args.data, args.out),
        4 => day4::_main(args.data, args.out),
        5 => day5::_main(args.data, args.out),
        6 => day6::_main(args.data, args.out, args.verbosity),
        7 => day7::_main(args.data, args.out, args.verbosity),
        8 => day8::_main(args.data, args.out, args.verbosity),
        9 => day9::_main(args.data, args.out, args.verbosity),
        10 => day10::_main(args.data, args.out, args.verbosity),
        11 => day11::_main(args.data, args.out, args.verbosity),
        12 => day12::_main(args.data, args.out, args.verbosity),
        13 => day13::_main(args.data, args.out, args.verbosity),
        14 => day14::_main(args.data, args.verbosity),
        15 => day15::_main(args.data, args.verbosity),
        16 => day16::_main(args.data, args.verbosity),
        17 => day17::_main(args.data, args.verbosity),
        18 => day18::_main(args.data, args.verbosity),
        _ => Err(AOCError::GenError("Not implemented".into())),
    }?;
    Ok(())
}
