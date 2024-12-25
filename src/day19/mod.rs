use super::{AOCError, Result};
use std::{collections::HashMap, fs, path::PathBuf};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let (avail, designs) = parse(data)?;
    let res1 = n_valid(&avail, &designs);
    println!("r1: {}", res1);
    let res2 = num_patterns(&avail, &designs);
    println!("res1: {}, res2: {}", res1, res2);
    Ok(())
}

fn n_valid(avail: &TowelStack, patterns: &TowelStack) -> usize {
    patterns
        .iter()
        .map(|p| is_valid_pattern(avail, p, 0) as usize)
        .sum()
}

fn num_patterns(avail: &TowelStack, patterns: &TowelStack) -> usize {
    let mut stored: HashMap<(usize, Towel), usize> = HashMap::new();
    let mut tot = 0;
    for towel in patterns.iter() {
        tot += get_valid_patterns(avail, towel, 0, &mut stored);
    }

    tot
}

fn get_valid_patterns(
    avail: &TowelStack,
    pattern: &Towel,
    current_stripe: usize,
    stored_patterns: &mut HashMap<(usize, Towel), usize>,
) -> usize {
    if current_stripe == pattern.len() {
        return 1;
    }
    let key = (current_stripe, pattern[current_stripe..].to_vec());
    if let Some(&cached_result) = stored_patterns.get(&key) {
        return cached_result;
    }

    let mut tot = 0;
    for a in avail.iter() {
        if a.len() + current_stripe > pattern.len() {
            continue;
        }
        if pattern[current_stripe..current_stripe + a.len()] == a[..] {
            tot += get_valid_patterns(avail, pattern, current_stripe + a.len(), stored_patterns);
        }
    }

    stored_patterns.insert(key, tot);
    tot
}

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

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
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
