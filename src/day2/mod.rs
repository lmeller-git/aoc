use super::{AOCError, Result};
use std::fs;
use std::path::PathBuf;

type Array = Vec<Vec<u32>>;

pub fn _main(data: PathBuf, out: PathBuf) -> Result<()> {
    let data = load_data(data)?;
    let res1 = safe_recs(&data);
    let res2 = safe_recs(&data);
    write_data((res1, res2), out)?;
    Ok(())
}

fn safe_recs(data: &Array) -> u64 {
    let mut tot = 0;
    for report in data {
        let mut is_valid = (1..=3).contains(
            &report
                .first()
                .expect("well shit")
                .abs_diff(*report.get(1).expect("crap")),
        );
        let is_decreasing = report.first() > report.get(1);
        for levels in report[1..].windows(2) {
            if let [p1, p2, ..] = levels {
                if !(1..=3).contains(&p1.abs_diff(*p2)) || ((p2 < p1) != is_decreasing) {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            tot += 1;
        }
    }
    tot
}

fn load_data(data: PathBuf) -> Result<Array> {
    let f = fs::read_to_string(data)?;
    let mut lines = Array::default();
    for line in f.lines() {
        let l = line
            .split_whitespace()
            .map(|item| match item.parse::<u32>() {
                Ok(v) => Ok(v),
                Err(_e) => Err(AOCError::ParseError(format!("could not parse {}", item))),
            })
            .collect::<Result<Vec<u32>>>()?;
        lines.push(l);
    }
    Ok(lines)
}

fn write_data(data: (u64, u64), out: PathBuf) -> Result<()> {
    std::fs::write(out, format!("{}, {}", data.0, data.1))?;
    Ok(())
}
