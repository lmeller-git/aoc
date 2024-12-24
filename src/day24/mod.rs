#![allow(clippy::upper_case_acronyms)]
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use super::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let (_deps, ops, solved) = parse(data)?;
    let res1 = solve(ops, solved);
    println!("res1: {}", res1);
    Ok(())
}

fn solve(mut ops: Vec<Operation>, mut solved: HashSet<Wire>) -> u64 {
    //println!("{:#?}", ops);
    let mut i = ops.len() - 1;
    while let Some(current_op) = ops.get(i) {
        if let (Some(lhs), Some(rhs)) = (solved.get(current_op.lhs()), solved.get(current_op.rhs()))
        {
            let res = current_op.execute_with_vals(lhs.value, rhs.value);
            solved.insert(res);
            ops.remove(i);
        } else if i == 0 {
            i = ops.len();
        }
        i = i.saturating_sub(1);
        //println!("i: {}, length: {}", i, ops.len());
    }
    println!("{:#?}, i: {}", ops, i);
    let mut vals = solved.drain().collect::<Vec<Wire>>();
    vals.sort_by(|a, b| b.id.cmp(&a.id));
    let mut res = 0;
    for val in vals.iter() {
        if val.id.starts_with("z") {
            println!("id: {}, val: {}", val.id, val.value);
            res = (res << 1) ^ val.value as u64;
        }
    }
    res
}

#[derive(Default, Debug, Clone)]
struct Wire {
    id: String,
    value: bool,
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Wire {}

impl Hash for Wire {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    XOR(Wire, Wire, Wire),
    AND(Wire, Wire, Wire),
    OR(Wire, Wire, Wire),
}

impl Operation {
    fn _execute(&self) -> Wire {
        let val = match self {
            Self::XOR(lhs, rhs, _) => lhs.value ^ rhs.value,
            Self::OR(lhs, rhs, _) => lhs.value || rhs.value,
            Self::AND(lhs, rhs, _) => lhs.value && rhs.value,
        };
        let mut w = self.res();
        w.value = val;
        w
    }

    fn execute_with_vals(&self, lhs: bool, rhs: bool) -> Wire {
        let val = match self {
            Self::XOR(_, _, _) => lhs ^ rhs,
            Self::AND(_, _, _) => lhs && rhs,
            Self::OR(_, _, _) => lhs || rhs,
        };
        let mut w = self.res();
        w.value = val;
        w
    }

    fn lhs(&self) -> &Wire {
        match self {
            Self::XOR(lhs, _rhs, _) | Self::AND(lhs, _rhs, _) | Self::OR(lhs, _rhs, _) => lhs,
        }
    }
    fn rhs(&self) -> &Wire {
        match self {
            Self::XOR(_lhs, rhs, _) | Self::AND(_lhs, rhs, _) | Self::OR(_lhs, rhs, _) => rhs,
        }
    }
    fn res(&self) -> Wire {
        match self {
            Self::XOR(_lhs, _rhs, res) | Self::AND(_lhs, _rhs, res) | Self::OR(_lhs, _rhs, res) => {
                res.clone()
            }
        }
    }
}

type Dependencies = HashMap<Wire, Vec<Wire>>;

fn parse(data: PathBuf) -> Result<(Dependencies, Vec<Operation>, HashSet<Wire>)> {
    let mut deps: HashMap<Wire, Vec<Wire>> = Dependencies::new();
    let mut solved: HashSet<Wire> = HashSet::new();
    let mut operations = Vec::new();
    let f = fs::read_to_string(data)?;
    let f = f.lines().collect::<Vec<&str>>();
    let mut f = f.split(|line| line.is_empty());
    if let (Some(init), Some(ops)) = (f.next(), f.next()) {
        for line in init.iter() {
            let mut l = line.split_whitespace();
            if let (Some(wire), Some(val)) = (l.next(), l.next()) {
                let v = wire.strip_suffix(":");
                if let Some(v) = v {
                    deps.insert(
                        Wire {
                            id: v.into(),
                            value: val.parse::<u32>().map_err(|_e| {
                                AOCError::ParseError(format!("could not parse init val, {val}"))
                            })? != 0,
                        },
                        Vec::new(),
                    );
                    solved.insert(Wire {
                        id: v.into(),
                        value: val.parse::<u32>().map_err(|_e| {
                            AOCError::ParseError(format!("could not parse init val, {val}"))
                        })? != 0,
                    });
                }
            }
        }
        for line in ops.iter() {
            let mut l = line.split_whitespace();
            if let (Some(lhs), Some(op), Some(rhs), Some(_), Some(res)) =
                (l.next(), l.next(), l.next(), l.next(), l.next())
            {
                let lhs = Wire {
                    id: lhs.into(),
                    value: false,
                };
                let rhs = Wire {
                    id: rhs.into(),
                    value: false,
                };
                let res = Wire {
                    id: res.into(),
                    value: false,
                };
                match op {
                    "XOR" => operations.push(Operation::XOR(lhs.clone(), rhs.clone(), res.clone())),
                    "OR" => operations.push(Operation::OR(lhs.clone(), rhs.clone(), res.clone())),
                    "AND" => operations.push(Operation::AND(lhs.clone(), rhs.clone(), res.clone())),
                    _ => return Err(AOCError::ParseError("wrong operation".into())),
                }
                deps.entry(res)
                    .and_modify(|e| e.extend([lhs.clone(), rhs.clone()]))
                    .or_insert(vec![lhs, rhs]);
            }
        }
    } else {
        return Err(AOCError::ParseError("not enough values to unpack".into()));
    }
    Ok((deps, operations, solved))
}
