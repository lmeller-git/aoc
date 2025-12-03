use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::Result;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let banks = parse(data);
    let res1 = part1(&banks);
    let res2 = part2(&banks);
    println!("part1: {res1}, part2: {res2}");
    Ok(())
}

fn part1(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let (first_idx, first_jolt) = find_max(&bank[..bank.len() - 1]);
            let (_, scnd_jolt) = find_max(&bank[first_idx + 1..]);
            (first_jolt * 10 + scnd_jolt) as u64
        })
        .sum()
}

fn part2(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let mut total = 0;
            let mut last_idx = 0;
            for i in (0..12).rev() {
                let (idx, best) = find_max(&bank[last_idx..bank.len() - i]);
                last_idx += idx + 1;
                total += best as u64 * 10_u64.pow(i as u32)
            }
            total
        })
        .sum()
}

fn find_max(bank: &[u8]) -> (usize, u8) {
    bank.iter()
        .cloned()
        .enumerate()
        .max_by(|item, item2| item.1.cmp(&item2.1).then(item2.0.cmp(&item.0)))
        .unwrap()
}

fn parse(data: PathBuf) -> Vec<Vec<u8>> {
    let f = File::open(data).unwrap();
    let reader = BufReader::new(f);
    reader
        .lines()
        .filter_map(|l| {
            let l = l.unwrap();
            if l.is_empty() {
                None
            } else {
                Some(
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect::<Vec<u8>>(),
                )
            }
        })
        .collect()
}
