use super::{AOCError, Result};
use std::fs;
use std::path::PathBuf;

type Array = Vec<Vec<u64>>;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let data = load_data(data)?;
    let res1 = safe_recs(&data);
    let res2 = safe_recs_with_damp(&data);
    println!("res1: {}, res2: {}", res1, res2);
    Ok(())
}

fn safe_recs_with_damp(data: &Array) -> u64 {
    let mut tot = 0;
    for report in data {
        if report.is_empty() {
            tot += 1;
            continue;
        }
        for rem in 0..report.len() {
            let mut is_valid = true;
            let mut rec = report.clone();
            rec.remove(rem);
            if rec.len() < 2 {
                tot += 1;
                break;
            }
            let last_dec = rec.first() > rec.get(1);
            for levels in rec.windows(2) {
                if let [p1, p2, ..] = levels {
                    if !(1..=3).contains(&p1.abs_diff(*p2)) || ((p1 > p2) != last_dec) {
                        is_valid = false;
                        break;
                    }
                }
            }
            if is_valid {
                tot += 1;
                break;
            }
        }
    }
    tot
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
            .map(|item| match item.parse::<u64>() {
                Ok(v) => Ok(v),
                Err(_e) => Err(AOCError::ParseError(format!("could not parse {}", item))),
            })
            .collect::<Result<Vec<u64>>>()?;
        lines.push(l);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let data = vec![vec![48, 46, 48, 51, 54, 56]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_2() {
        let data = vec![vec![1, 1, 2, 3, 4, 5]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_3() {
        let data = vec![vec![1, 2, 3, 4, 5, 5]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_4() {
        let data = vec![vec![5, 1, 2, 3, 4, 5]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_5() {
        let data = vec![vec![1, 4, 3, 2, 1]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_6() {
        let data = vec![vec![1, 6, 7, 8, 9]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_7() {
        let data = vec![vec![1, 2, 3, 4, 3]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_8() {
        let data = vec![vec![9, 8, 7, 6, 7]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_9() {
        let data = vec![vec![7, 10, 8, 10, 11]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_10() {
        let data = vec![vec![29, 28, 27, 25, 26, 22, 20]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_11() {
        let data = vec![vec![12, 10, 13, 16, 19, 21, 22]];
        assert_eq!(safe_recs_with_damp(&data), 1);
    }
    #[test]
    fn test_short() {
        let data = vec![
            vec![0, 1],
            vec![2, 5],
            vec![2, 4, 3],
            vec![8, 6, 1],
            vec![1, 5, 9],
        ];
        assert_eq!(safe_recs_with_damp(&data), 4);
    }
}
