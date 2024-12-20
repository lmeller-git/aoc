use std::{fs, path::PathBuf};

// build tree for each pattern. if a tree does not terminate, it is impossible
use super::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let (avail, designs) = parse(data)?;
    let res1 = n_valid(&avail, &designs);
    println!("res1: {}", res1);
    Ok(())
}

fn n_valid(avail: &TowelStack, patterns: &TowelStack) -> usize {
    patterns
        .iter()
        .map(|p| is_valid_pattern(avail, p, 0) as usize)
        .sum()
}

// DFS for valid pattern
fn is_valid_pattern(avail: &TowelStack, pattern: &Towel, current_stripe: usize) -> bool {
    if current_stripe == pattern.len() {
        return true;
    }
    for a in avail {
        if a.len() + current_stripe > pattern.len() {
            continue;
        }
        if pattern[current_stripe..current_stripe + a.len()] == a[..]
            && is_valid_pattern(avail, pattern, current_stripe + a.len())
        {
            return true;
        }
    }
    false
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Towel = Vec<Stripe>;
type TowelStack = Vec<Towel>;

fn parse(data: PathBuf) -> Result<(TowelStack, TowelStack)> {
    let f = fs::read_to_string(data)?;
    let mut f = f.lines();
    let avail = if let (Some(avail), Some(_)) = (f.next(), f.next()) {
        Ok(avail
            .split(',')
            .map(|pattern| {
                pattern
                    .chars()
                    .filter_map(|c| match c {
                        'r' => Some(Stripe::Red),
                        'w' => Some(Stripe::White),
                        'u' => Some(Stripe::Blue),
                        'b' => Some(Stripe::Black),
                        'g' => Some(Stripe::Green),
                        _ => None,
                    })
                    .collect::<Towel>()
            })
            .collect::<TowelStack>())
    } else {
        Err(AOCError::ParseError("too few lines".into()))
    }?;
    let designs = f
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    'r' => Some(Stripe::Red),
                    'w' => Some(Stripe::White),
                    'u' => Some(Stripe::Blue),
                    'b' => Some(Stripe::Black),
                    'g' => Some(Stripe::Green),
                    _ => None,
                })
                .collect::<Towel>()
        })
        .collect::<TowelStack>();
    Ok((avail, designs))
}
