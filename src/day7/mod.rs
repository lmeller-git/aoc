use super::{AOCError, Result};
use std::{io::Read, path::PathBuf};

pub fn _main(data: PathBuf, _out: PathBuf, _verbosity: u8) -> Result<()> {
    let eqs = parse(data)?;
    let res = solve(&eqs)?;
    println!("part1 : {}", res);
    Ok(())
}

#[derive(Debug, Default)]
struct Equation {
    res: u64,
    data: Vec<u64>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn is_solvable(eq: &Equation, current_operand: usize, current_res: u64) -> bool {
    if current_operand == eq.data.len() {
        return current_res == eq.res;
    }
    is_solvable(
        eq,
        current_operand + 1,
        current_res + eq.data[current_operand],
    ) || is_solvable(
        eq,
        current_operand + 1,
        current_res * eq.data[current_operand],
    )
}

fn solve(eqs: &[Equation]) -> Result<u64> {
    Ok(eqs
        .iter()
        .map(|eq| if is_solvable(eq, 0, 0) { eq.res } else { 0 })
        .sum())
}

fn parse(data: PathBuf) -> Result<Vec<Equation>> {
    let mut f = std::fs::File::open(data)?;
    let mut buf = String::new();
    let _b = f.read_to_string(&mut buf)?;
    let eqs = buf
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut l = line.split(':');
            if let (Some(res), Some(vals)) = (l.next(), l.next()) {
                let res = res
                    .parse::<u64>()
                    .map_err(|_e| AOCError::ParseError("could not parse result".into()))?;
                let vals = vals
                    .split_whitespace()
                    .map(|val| {
                        val.parse::<u64>()
                            .map_err(|_e| AOCError::ParseError("could not parse values".into()))
                    })
                    .collect::<Result<Vec<u64>>>()?;
                return Ok(Equation { res, data: vals });
            }
            Err(AOCError::ParseError("missing values and/or result".into()))
        })
        .collect::<Result<Vec<Equation>>>()?;

    Ok(eqs)
}
