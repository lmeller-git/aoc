use std::{collections::HashMap, fs, path::PathBuf};

use super::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let nums = parse(data)?;
    let res1 = solve(&nums);
    let res2 = best_sequence(&nums);
    println!("res1: {}, res2: {}", res1, res2);
    Ok(())
}

fn best_sequence(nums: &Vec<u64>) -> i64 {
    let mut all_changes: HashMap<[i8; 4], i64> = HashMap::new();
    let mut prices = HashMap::new();
    for num in nums {
        let mut last_changes = [0, 0, 0, 0];
        let mut number = first_four(num, &mut last_changes, &mut prices);
        for _ in 4..2000 {
            number = next_change(&number, &mut last_changes, &mut prices);
        }
        for (k, v) in prices.drain() {
            all_changes
                .entry(k)
                .and_modify(|e| *e += v as i64)
                .or_insert(v as i64);
        }
    }
    let mut max = i64::MIN;
    let mut best_changes = [0, 0, 0, 0];
    for (k, v) in all_changes {
        if v > max {
            max = v;
            best_changes = k;
        }
    }
    max
}

fn first_four(num: &u64, last_changes: &mut [i8; 4], prices: &mut HashMap<[i8; 4], i8>) -> u64 {
    let mut next_num = next_secret_num(num);
    let last_digit = (next_num % 10) as i8;
    let prev_last_digit = (*num % 10) as i8;
    *last_changes.first_mut().unwrap() = last_digit - prev_last_digit;
    let mut prev_num = last_digit;
    for i in 1..4 {
        next_num = next_secret_num(&next_num);
        let last_digit = (next_num % 10) as i8;
        let prev_last_digit = prev_num;
        prev_num = last_digit;
        *last_changes.get_mut(i).unwrap() = last_digit - prev_last_digit;
    }
    prices.insert(*last_changes, last_digit);
    next_num
}

fn next_change(num: &u64, last_changes: &mut [i8; 4], prices: &mut HashMap<[i8; 4], i8>) -> u64 {
    let next_num = next_secret_num(num);
    last_changes.rotate_left(1);
    let last_digit = (next_num % 10) as i8;
    let prev_last_digit = (*num % 10) as i8;
    *last_changes.last_mut().unwrap() = last_digit - prev_last_digit;
    if !prices.contains_key(last_changes) {
        prices.insert(*last_changes, last_digit);
    }
    next_num
}

fn solve(nums: &Vec<u64>) -> u64 {
    let mut tot = 0;
    for n in nums {
        tot += nth_secret_num(n, 2000);
    }
    tot
}

fn nth_secret_num(num: &u64, n: usize) -> u64 {
    let mut number = next_secret_num(num);
    for _ in 1..n {
        number = next_secret_num(&number);
    }
    number
}

fn next_secret_num(num: &u64) -> u64 {
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
