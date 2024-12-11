use super::{AOCError, Result};
use std::path::PathBuf;

pub fn _main(data: PathBuf, _out: PathBuf, _verbosity: u8) -> Result<()> {
    let stones = parse(data)?;
    let res = solve(stones);
    println!("res: {}", res);
    Ok(())
}

type Stones = Vec<u64>;

fn parse(data: PathBuf) -> Result<Stones> {
    let f = std::fs::read_to_string(data)?;
    f.split_whitespace()
        .map(|num| {
            num.parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse num".into()))
        })
        .collect::<Result<Stones>>()
}

fn solve(mut stones: Stones) -> usize {
    for _ in 0..25 {
        tick(&mut stones);
    }
    stones.len()
}

fn tick(stones: &mut Stones) {
    let mut i = 0;
    while i < stones.len() {
        if let Some(new_stone) = update_stone(stones.get_mut(i)) {
            i += 1;
            stones.insert(i, new_stone);
        }
        i += 1;
    }
}

fn update_stone(stone: Option<&mut u64>) -> Option<u64> {
    if let Some(stone) = stone {
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
    }
    None
}

#[cfg(test)]
mod tests {
    use super::update_stone;

    #[test]
    fn split_stone() {
        let mut stone = 2010;
        let stone2 = update_stone(Some(&mut stone));
        assert_eq!(stone2, Some(10));
        assert_eq!(stone, 20);
    }
}
