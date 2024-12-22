use std::{fs, path::PathBuf};

use super::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let nums = parse(data)?;
    let res1 = solve(nums);
    println!("res1: {}", res1);
    Ok(())
}

fn solve(nums: Vec<u64>) -> u64 {
    let mut tot = 0;
    for n in nums {
        tot += nth_secret_num(n, 2000);
    }
    tot
}

fn nth_secret_num(num: u64, n: usize) -> u64 {
    let mut number = next_secret_num(num);
    for _ in 1..n {
        number = next_secret_num(number);
    }
    number
}

fn next_secret_num(num: u64) -> u64 {
    let num2 = ((num * 64) ^ num) % 16777216;
    let num3 = ((num2 / 32) ^ num2) % 16777216;
    ((num3 * 2048) ^ num3) % 16777216
}

fn parse(data: PathBuf) -> Result<Vec<u64>> {
    let f = fs::read_to_string(data)?;
    f.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse num".into()))
        })
        .collect::<Result<Vec<u64>>>()
}
