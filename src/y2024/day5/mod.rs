use super::{AOCError, Result};
use core::panic;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

type Data = Vec<Vec<u64>>;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let (map, records) = parse(data)?;
    let res = get_sorted_sum(&map, &records);
    println!("{}, {}", res.0, res.1);
    Ok(())
}

#[derive(Debug, Default)]
struct OrderMap {
    nodes: HashMap<u64, Vec<u64>>,
    in_degree: HashMap<u64, usize>,
}

fn get_sorted_sum(map: &OrderMap, recs: &Data) -> (u64, u64) {
    let mut tot = 0;
    let mut tot2 = 0;
    for rec in recs {
        if is_sorted(map, rec) {
            tot += rec[rec.len() / 2];
        } else {
            match sort(map, rec) {
                Ok(res) => tot2 += res[res.len() / 2],
                Err(AOCError::SolverError(_)) => continue,
                _ => panic!("wtf"),
            }
        }
    }
    (tot, tot2)
}

fn sort(map: &OrderMap, rec: &[u64]) -> Result<Vec<u64>> {
    let mut in_degree = build_in_degree(map, rec);
    let mut zero_stack = Vec::new();
    for (node, degree) in in_degree.iter() {
        if *degree == 0 {
            zero_stack.push(*node);
        }
    }
    let mut res = Vec::new();
    while let Some(next) = zero_stack.pop() {
        res.push(next);
        for succ in &map.nodes[&next] {
            in_degree.entry(*succ).and_modify(|succ| *succ -= 1);
            if let Some(s) = in_degree.get(succ) {
                if *s == 0 {
                    zero_stack.push(*succ);
                }
            }
        }
    }
    if res.len() != rec.len() {
        return Err(AOCError::SolverError("cyclic graph".into()));
    }

    Ok(res)
}

fn build_in_degree(map: &OrderMap, rec: &[u64]) -> HashMap<u64, usize> {
    let mut in_degree: HashMap<u64, usize> = HashMap::new();
    for (k, v) in map.nodes.iter() {
        if !rec.contains(k) {
            continue;
        }
        in_degree.entry(*k).or_insert(0);
        for n in v {
            if !rec.contains(n) {
                continue;
            }
            in_degree
                .entry(*n)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
        }
    }
    in_degree
}

fn is_sorted(map: &OrderMap, rec: &[u64]) -> bool {
    let mut in_degree: HashMap<u64, usize> = build_in_degree(map, rec);
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
