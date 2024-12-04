use super::{AOCError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Array = (Vec<u64>, Vec<u64>);

pub fn _main(data: PathBuf, out: PathBuf) -> Result<()> {
    let data = load_data(data)?;
    let res2 = calc_similarity(&data);
    let res = get_diff(data);
    write_data((res, res2), out)?;
    Ok(())
}

fn get_diff(data: Array) -> u64 {
    let mut v1 = data.0;
    v1.sort();
    let mut v2 = data.1;
    v2.sort();
    v1.iter()
        .zip(v2.iter())
        .map(|(val1, val2)| val1.abs_diff(*val2))
        .sum()
}

fn calc_similarity(data: &Array) -> u64 {
    let mut map: HashMap<u64, u64> = HashMap::default();
    data.0
        .iter()
        .map(|value| {
            let key = map
                .entry(*value)
                .or_insert(data.1.iter().filter(|&v| v == value).count() as u64);
            *value * *key
        })
        .sum()
}

fn load_data(data: PathBuf) -> Result<Array> {
    let f = fs::read_to_string(data)?;
    let mut lines = Array::default();
    for line in f.lines() {
        let l = line
            .split_whitespace()
            .map(|item| match item.parse::<u64>() {
                Ok(v) => Ok(v),
                Err(_e) => Err(AOCError::ParseError(format!("could not parse {}", item))),
            })
            .collect::<Result<Vec<u64>>>()?;
        if let [v1, v2, ..] = l[..] {
            lines.0.push(v1);
            lines.1.push(v2);
        } else {
            return Err(AOCError::ParseError(
                "a line had less than 2 entries".into(),
            ));
        }
    }
    Ok(lines)
}

fn write_data(data: (u64, u64), out: PathBuf) -> Result<()> {
    std::fs::write(out, format!("{}, {}", data.0, data.1))?;
    Ok(())
}
