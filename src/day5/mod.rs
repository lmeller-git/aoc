use super::{AOCError, Result};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

type Data = Vec<Vec<u64>>;

pub fn _main(data: PathBuf, out: PathBuf) -> Result<()> {
    let (map, records) = parse(data)?;
    let res = get_sorted_sum(&map, &records);
    println!("{}", res);
    Ok(())
}

#[derive(Debug, Default)]
struct OrderMap {
    nodes: HashMap<u64, Vec<u64>>,
    in_degree: HashMap<u64, usize>,
}

fn get_sorted_sum(map: &OrderMap, recs: &Data) -> u64 {
    let mut tot = 0;
    for rec in recs {
        if is_sorted(map, rec) {
            tot += rec[rec.len() / 2];
        }
    }
    tot
}

fn is_sorted(map: &OrderMap, rec: &[u64]) -> bool {
    let mut in_degree: HashMap<u64, usize> = HashMap::new();
    for (k, v) in map.nodes.iter() {
        if !rec.contains(k) {
            continue;
        }
        for n in v {
            in_degree
                .entry(*n)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
        }
    }
    for p in rec {
        if let Some(d) = in_degree.get(p) {
            if *d > 0 {
                return false;
            }
        }
        if let Some(node) = map.nodes.get(p) {
            for child in node {
                in_degree
                    .entry(*child)
                    .and_modify(|entry| *entry -= 1)
                    .or_insert(0);
            }
        }
    }
    true
}

enum ParseState {
    Order,
    Records,
}

fn parse(data: PathBuf) -> Result<(OrderMap, Data)> {
    let mut map = OrderMap::default();
    let mut recs = Data::new();
    let f = fs::File::open(data)?;
    let mut reader = io::BufReader::new(f);
    let mut buf = String::new();
    let mut state = ParseState::Order;
    while reader.read_line(&mut buf)? > 0 {
        buf.retain(|c| c != '\n');
        match state {
            ParseState::Order => {
                if !buf.contains('|') {
                    state = ParseState::Records;
                } else {
                    parse_order(&buf, &mut map)?;
                }
            }
            ParseState::Records => recs.push(parse_record(&buf)?),
        }
        buf.clear();
    }

    Ok((map, recs))
}

fn parse_order(line: &str, map: &mut OrderMap) -> Result<()> {
    let nums = line
        .split("|")
        .map(|item| {
            item.parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse order".into()))
        })
        .collect::<Result<Vec<u64>>>()?;
    if let [first, second, ..] = &nums[..] {
        map.nodes
            .entry(*first)
            .and_modify(|entry| entry.push(*second))
            .or_insert(vec![*second]);
        map.in_degree.entry(*first).or_insert(0);
        map.in_degree
            .entry(*second)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
        Ok(())
    } else {
        Err(AOCError::ParseError(
            "less than two elements in order detected".into(),
        ))
    }
}

fn parse_record(record: &str) -> Result<Vec<u64>> {
    let rec = record
        .split(',')
        .map(|v| {
            v.parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse record".into()))
        })
        .collect::<Result<Vec<u64>>>()?;
    Ok(rec)
}
