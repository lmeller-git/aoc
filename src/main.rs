use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;
mod day1;
mod day2;

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
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.day {
        1 => day1::_main(args.data, args.out),
        2 => day2::_main(args.data, args.out),
        _ => Err(AOCError::GenError("Not implemented".into())),
    }?;
    Ok(())
}
