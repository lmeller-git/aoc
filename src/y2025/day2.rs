use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let ids = parse(data)?;
    let res1 = part1(&ids);
    let res2 = part2(&ids);
    println!("res1: {res1}, res2: {res2}");
    Ok(())
}

#[derive(Debug)]
struct IDRange {
    from: u64,
    to: u64,
}

impl IDRange {
    fn compute_invalids(&self) -> Vec<u64> {
        let mut invalids = Vec::new();
        let mut iter = self.from..=self.to;
        let iter_ref = &mut iter;
        loop {
            let Some(item) = iter_ref.next() else {
                break;
            };
            let shift = item.ilog10();
            if shift % 2 != 0 {
                let div = 10_u64.pow(shift.div_ceil(2));
                let upper = item / div;
                let lower = item % div;

                if upper == lower {
                    invalids.push(item);
                    _ = iter_ref.nth(div.saturating_sub(1) as usize);
                }
            }
        }

        invalids
    }

    fn compute_invalid_part2(&self) -> u64 {
        let mut iter = self.from..=self.to;
        let iter_ref = &mut iter;
        let mut sum = 0;
        while let Some(item) = iter_ref.next() {
            let max_shift = item.ilog10() + 1;
            'shifts: for shift in 1..=(max_shift / 2) {
                if max_shift % shift != 0 {
                    continue;
                }
                let div = 10_u64.pow(shift);
                let mut current = item;
                let mut last = current % div;
                for _ in 1..(max_shift / shift) {
                    current /= div;
                    let next = current % div;
                    if last != next {
                        continue 'shifts;
                    }
                    last = next;
                }
                sum += item;
                break;
            }
        }
        sum
    }
}

impl TryFrom<&[u8]> for IDRange {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut from_to = value.split(|item| *item == b'-');
        if let Some(from) = from_to.next()
            && let Some(to) = from_to.next()
        {
            let from = str::from_utf8(from)
                .map_err(|_| "could not parse from")?
                .trim()
                .parse()
                .map_err(|_| "could not parse from num")?;
            let to = str::from_utf8(to)
                .map_err(|_| "could not parse to")?
                .trim()
                .parse()
                .map_err(|_| "could not parse to num")?;
            Ok(Self { from, to })
        } else {
            Err("no - found")
        }
    }
}

fn part1(data: &[IDRange]) -> u64 {
    data.iter()
        .map(|range| range.compute_invalids().iter().sum::<u64>())
        .sum()
}

fn part2(data: &[IDRange]) -> u64 {
    data.iter().map(|id| id.compute_invalid_part2()).sum()
}

fn parse(data: PathBuf) -> Result<Vec<IDRange>> {
    let f = fs::File::open(data).map_err(|_| AOCError::ParseError("".into()))?;
    let reader = BufReader::new(f);
    let mut ids = Vec::new();
    let mut split = reader.split(b',');
    while let Some(Ok(pair)) = split.next() {
        if let Ok(range) = pair.as_slice().try_into() {
            ids.push(range);
        }
    }
    Ok(ids)
}
