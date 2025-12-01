use super::{AOCError, Result};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let stones2 = parse_hash(&data)?;
    let res2 = solve_efficient(stones2, 75);
    let stones = parse_hash(&data)?;
    let res = solve_efficient(stones, 25);
    println!("res: {}, res2: {}", res, res2);
    Ok(())
}

type HashStones = HashMap<u64, usize>;

fn parse_hash(data: &PathBuf) -> Result<HashStones> {
    let f = std::fs::read_to_string(data)?;
    let mut stones = HashStones::new();
    for stone in f.split_whitespace().map(|num| {
        num.parse::<u64>()
            .map_err(|_e| AOCError::ParseError("could not parse num".into()))
    }) {
        stones.entry(stone?).and_modify(|e| *e += 1).or_insert(1);
    }
    Ok(stones)
}

fn solve_efficient(mut stones: HashStones, blinks: usize) -> usize {
    for _ in 0..blinks {
        tick_efficient(&mut stones);
    }
    stones.values().sum()
}

fn tick_efficient(stones: &mut HashStones) {
    let mut new_entries = HashStones::with_capacity(stones.len());
    for (value, num) in stones.iter() {
        let mut current_value = *value;
        if let Some(new_value) = update_stone(&mut current_value) {
            new_entries
                .entry(new_value)
                .and_modify(|e| *e += *num)
                .or_insert(*num);
        }
        new_entries
            .entry(current_value)
            .and_modify(|e| *e += *num)
            .or_insert(*num);
    }
    *stones = new_entries;
}

fn update_stone(stone: &mut u64) -> Option<u64> {
    if *stone == 0 {
        *stone = 1;
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let div = 10_u64.pow((stone.ilog10() + 1) / 2);
        let first_half = *stone / div;
        let second_half = *stone % div;
        *stone = first_half;
        return Some(second_half);
    } else {
        *stone *= 2024;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::update_stone;

    #[test]
    fn split_stone() {
        let mut stone = 2010;
        let stone2 = update_stone(&mut stone);
        assert_eq!(stone2, Some(10));
        assert_eq!(stone, 20);
    }
}
