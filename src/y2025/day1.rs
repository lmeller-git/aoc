use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let ops = parse(data)?;
    let res1 = part1(&ops);
    let res2 = part2(&ops);
    println!("res1: {res1}");
    println!("res2: {res2}");
    Ok(())
}

fn part2(ops: &[i16]) -> u32 {
    let mut pos = 50;
    let mut zeros = 0;
    for op in ops {
        let (pos_, zeroes_) = rotate(pos, *op);
        pos = pos_;
        zeros += zeroes_ as u32;
    }
    zeros
}

fn part1(ops: &[i16]) -> u32 {
    let mut pos = 50;
    let mut zeros = 0;
    for op in ops {
        (pos, _) = rotate(pos, *op);
        if pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn _rotate_brute_force(mut pos: u16, mut by: i16) -> (u16, u16) {
    let mut zeroes = 0;
    while by != 0 {
        pos = ((pos as i16 + by.signum()).rem_euclid(100)) as u16;
        if pos == 0 {
            zeroes += 1;
        }
        by -= by.signum();
    }
    (pos, zeroes)
}

fn rotate(pos: u16, by: i16) -> (u16, i32) {
    let from = pos as i32;
    let to = from + by as i32;

    let laps = if by < 0 {
        (to - 1).div_euclid(100) - (from - 1).div_euclid(100) // if pos was 0, we need to subtract 1
    } else if by > 0 {
        to / 100
    } else {
        0
    }
    .abs();
    (to.rem_euclid(100) as u16, laps)
}

fn parse(data: PathBuf) -> Result<Vec<i16>> {
    let file = std::fs::File::open(data).map_err(|_| AOCError::ParseError("".into()))?;
    let reader = BufReader::new(file);
    let mut ops = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|_| AOCError::ParseError("".into()))?;
        let (sign, num) = line
            .split_at_checked(1)
            .ok_or(AOCError::ParseError("".into()))?;
        let num = num
            .parse::<i16>()
            .map_err(|_| AOCError::ParseError("".into()))?;
        ops.push(match sign {
            "L" => -num,
            "R" => num,
            _ => return Err(AOCError::ParseError("".into())),
        });
    }
    Ok(ops)
}
