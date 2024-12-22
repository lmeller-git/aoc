use super::{AOCError, Result};
use std::{fs, path::PathBuf, sync::Arc, thread};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let (avail, designs) = parse(data)?;
    let res1 = n_valid(&avail, &designs);
    println!("r1: {}", res1);
    let res2 = n_patterns(avail, &designs);
    println!("res1: {}, res2: {}", res1, res2);
    Ok(())
}

fn n_valid(avail: &TowelStack, patterns: &TowelStack) -> usize {
    patterns
        .iter()
        .map(|p| is_valid_pattern(avail, p, 0) as usize)
        .sum()
}

fn n_patterns(avail: TowelStack, patterns: &TowelStack) -> usize {
    let n_threads = 13;
    let avail = Arc::new(avail);
    let chunks = patterns
        .chunks(patterns.len() / n_threads)
        .map(|chunk| Arc::new(chunk.to_vec()));
    let mut handles = Vec::new();
    for chunk in chunks {
        let chunk = Arc::new(chunk);
        let avail = Arc::clone(&avail);
        handles.push(thread::spawn(move || {
            chunk
                .iter()
                .map(|p| get_valid_patterns(avail.clone(), p, 0))
                .sum::<usize>()
        }))
    }
    let mut tot = 0;
    for h in handles {
        tot += h.join().unwrap();
    }
    tot
}

fn get_valid_patterns(avail: Arc<TowelStack>, pattern: &Towel, current_stripe: usize) -> usize {
    if current_stripe == pattern.len() {
        return 1;
    }
    let mut tot = 0;
    for a in avail.iter() {
        if a.len() + current_stripe > pattern.len() {
            continue;
        }
        if pattern[current_stripe..current_stripe + a.len()] == a[..] {
            tot += get_valid_patterns(avail.clone(), pattern, current_stripe + a.len());
        }
    }
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
