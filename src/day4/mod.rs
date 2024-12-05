use super::{AOCError, Result};
use std::fs;
use std::path::PathBuf;

type Array = Vec<Vec<u8>>;

pub fn _main(data: PathBuf, out: PathBuf) -> Result<()> {
    let arr = parse_data(data)?;
    let res = search(&arr);
    let res2 = search_x(&arr);
    println!("part1: {}, part2: {}", res, res2);
    write_data((res, res2), out)?;
    Ok(())
}

fn parse_data(data: PathBuf) -> Result<Array> {
    let f = fs::read(data)?;
    let d = f
        .split(|item| *item == b'\n')
        .map(|line| line.to_vec())
        .filter(|l| !l.is_empty());
    Ok(d.collect::<Array>())
}

fn write_data(data: (u64, u64), out: PathBuf) -> Result<()> {
    std::fs::write(out, format!("{}, {}", data.0, data.1))?;
    Ok(())
}
fn get_diags(arr: &Array) -> Array {
    let mut res = Array::new();
    for k in 0..arr[0].len() + arr.len() - 1 {
        res.push(Vec::new());
        for r in 0..arr.len() {
            for c in 0..arr[0].len() {
                if r as i64 == c as i64 + k as i64 - arr[0].len() as i64 + 1 {
                    res.last_mut().unwrap().push(arr[r][c]);
                }
            }
        }
    }

    res
}

fn transpose(arr: &Array) -> Array {
    let mut t = Array::new();
    for _ in 0..arr[0].len() {
        t.push(Vec::new());
    }
    for r in arr.iter() {
        for (j, c) in r.iter().enumerate() {
            t[j].push(*c);
        }
    }
    t
}

fn reverse_rows(arr: &Array) -> Array {
    arr.iter().rev().cloned().collect()
}

fn search_x(arr: &Array) -> u64 {
    let pattern1 = b"MAS";
    let pattern2 = b"SAM";
    let mut tot = 0;
    for (k, r) in get_diags(arr).iter().enumerate() {
        for (o, w) in r.windows(3).enumerate() {
            if (w == pattern1 || w == pattern2) && analyze_x(k, o, arr) {
                tot += 1;
            }
        }
    }
    tot
}

fn analyze_x(diag: usize, offset: usize, arr: &Array) -> bool {
    let cols = arr[0].len();
    let actual_diag = diag as i64 - cols as i64 + 1;
    let x = (offset as i64 - actual_diag.min(0)) as usize;
    let y = (offset as i64 + actual_diag.max(0)) as usize;
    let word = [arr[y + 2][x], arr[y + 1][x + 1], arr[y][x + 2]];
    word == *b"MAS" || word == *b"SAM"
}

fn search(arr: &Array) -> u64 {
    let pattern1 = b"XMAS";
    let pattern2 = b"SAMX";
    let mut tot = 0;
    for r in arr {
        for w in r.windows(4) {
            if w == pattern1 || w == pattern2 {
                tot += 1;
            }
        }
    }
    for r in &transpose(arr) {
        for w in r.windows(4) {
            if w == pattern1 || w == pattern2 {
                tot += 1;
            }
        }
    }
    for r in &get_diags(arr) {
        for w in r.windows(4) {
            if w == pattern1 || w == pattern2 {
                tot += 1;
            }
        }
    }
    for r in &get_diags(&reverse_rows(arr)) {
        for w in r.windows(4) {
            if w == pattern1 || w == pattern2 {
                tot += 1;
            }
        }
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t() {
        let test = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            transpose(&test),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
        let test2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(&test2), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_diag() {
        let test = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            get_diags(&test),
            vec![vec![3], vec![2, 6], vec![1, 5, 9], vec![4, 8], vec![7]]
        );
        let test2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            get_diags(&test2),
            vec![vec![3], vec![2, 6], vec![1, 5], vec![4]]
        );
    }
}
