use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use super::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let computers = parse(data)?;
    let res1 = get_clusters(&computers);
    println!("res1: {}", res1);
    Ok(())
}

fn get_clusters(computers: &HashMap<String, HashSet<String>>) -> usize {
    let relevant_cps = computers
        .iter()
        .filter(|(_k, v)| {
            v.iter().all(|id| {
                if let Some(ids) = computers.get(id) {
                    if ids.len() >= 2 {
                        return true;
                    }
                }
                false
            })
        })
        .collect::<HashMap<&String, &HashSet<String>>>();
    let mut cycles: Vec<HashSet<String>> = Vec::new();
    for (k, _v) in relevant_cps.iter() {
        for cycle in is_cyclic(&relevant_cps, k) {
            if !cycles.contains(&cycle) {
                cycles.push(cycle);
            }
        }
    }
    cycles.retain(|cycle| cycle.iter().any(|id| id.starts_with('t')));
    cycles.len()
}

fn is_cyclic(
    computers: &HashMap<&String, &HashSet<String>>,
    current: &String,
) -> Vec<HashSet<String>> {
    let mut cycles = Vec::new();
    for cp2 in computers[current] {
        if let Some(other) = computers.get(cp2) {
            let inter = computers[current].intersection(other);
            for cp in inter {
                let next = HashSet::from([current.into(), cp.into(), cp2.into()]);
                if !cycles.contains(&next) {
                    cycles.push(next);
                }
            }
        }
    }
    cycles
}

fn parse(data: PathBuf) -> Result<HashMap<String, HashSet<String>>> {
    let f = fs::read_to_string(data)?;
    let mut computers = HashMap::new();
    for line in f.lines() {
        if line.is_empty() {
            continue;
        }
        let line = line.trim();
        let mut l = line.split('-');
        if let (Some(first), Some(second)) = (l.next(), l.next()) {
            let first = first.to_string();
            let second = second.to_string();
            computers
                .entry(first.clone())
                .and_modify(|e: &mut HashSet<String>| {
                    e.insert(second.clone());
                })
                .or_insert(HashSet::from([second.clone()]));
            computers
                .entry(second)
                .and_modify(|e: &mut HashSet<String>| {
                    e.insert(first.clone());
                })
                .or_insert(HashSet::from([first]));
        } else {
            return Err(AOCError::ParseError("line too short".into()));
        }
    }
    Ok(computers)
}
