use super::{AOCError, Result};
use std::fs;
use std::path::PathBuf;

pub fn _main(data: PathBuf, out: PathBuf) -> Result<()> {
    let res = parse_input(&data, false)?;
    let res2 = parse_input(&data, true)?;
    write_data((res, res2), out)?;
    Ok(())
}

fn write_data(data: (u64, u64), out: PathBuf) -> Result<()> {
    std::fs::write(out, format!("{}, {}", data.0, data.1))?;
    Ok(())
}

fn parse_input(data: &PathBuf, do_enabled: bool) -> Result<u64> {
    let f = fs::read(data)?;
    let pattern = b"mul(";
    let do_pattern = b"do";
    let dont_pattern = b"don't";
    let mut tot = 0;
    let mut last_do = true;
    for (i, w) in f.windows(5).enumerate() {
        if &w[..2] == do_pattern {
            last_do = w != dont_pattern;
        }
        if !last_do && do_enabled {
            continue;
        }
        if &w[..4] == pattern {
            let remainder = &f[i + 4..i + 12];
            let mut iter = remainder.split(|item| *item == b',');
            if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
                let (v1, v2);
                match parse_to_num(first) {
                    Ok(val) => v1 = val,
                    Err(AOCError::ParseError(_e)) => continue,
                    Err(e) => return Err(e),
                }
                match parse_to_num(second.split(|num| *num == b')').next().expect("cant err")) {
                    Ok(val) => v2 = val,
                    Err(AOCError::ParseError(_e)) => continue,
                    Err(e) => return Err(e),
                }
                tot += mul(v1, v2);
            }
        }
    }

    Ok(tot)
}

fn mul(n1: u64, n2: u64) -> u64 {
    n1 * n2
}

fn parse_to_num(bytes: &[u8]) -> Result<u64> {
    let mut res = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            return Err(AOCError::ParseError("NaN".into()));
        }
        res = 10 * res + *b as u64 - 48;
    }
    Ok(res)
}
