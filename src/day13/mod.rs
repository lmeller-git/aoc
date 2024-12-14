use super::{AOCError, Result};
use std::path::PathBuf;

pub fn _main(data: PathBuf, _out: PathBuf, _verbosity: u8) -> Result<()> {
    let mut claw_machines = parse(data)?;
    let res = solve(&mut claw_machines);
    println!("res1: {}", res);
    Ok(())
}

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
enum SolveCost {
    Unsolvable,
    Solved(usize),
    #[default]
    Unspecified,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
struct ClawMachine {
    prize: Point,
    da: Point,
    db: Point,
    solve_cost: SolveCost,
}

fn solve(machines: &mut [ClawMachine]) -> usize {
    for machine in machines.iter_mut() {
        if let Some(cost) = get_min(machine.da, machine.db, machine.prize) {
            machine.solve_cost = SolveCost::Solved(cost);
        } else {
            machine.solve_cost = SolveCost::Unsolvable;
        }
    }
    machines
        .iter()
        .filter_map(|machine| match machine.solve_cost {
            SolveCost::Unsolvable | SolveCost::Unspecified => None,
            SolveCost::Solved(cost) => Some(cost),
        })
        .sum()
}

fn get_min(b1: Point, b2: Point, target: Point) -> Option<usize> {
    let det = b1.x * b2.y - b1.y * b2.x;
    if det == 0 {
        return None;
    }
    let xb = (target.y as f64 - b1.y as f64 / b1.x as f64 * target.x as f64)
        / (b2.y as f64 - b1.y as f64 / b1.x as f64 * b2.x as f64);
    let xa = (target.x as f64 - xb * b2.x as f64) / b1.x as f64;
    let xa = xa.round() as i32;
    let xb = xb.round() as i32;
    if xa < 0 || xb < 0 || (xa * b1.x + xb * b2.x, xa * b1.y + xb * b2.y) != (target.x, target.y) {
        return None;
    }
    Some(3 * xa as usize + xb as usize)
}

fn parse(data: PathBuf) -> Result<Vec<ClawMachine>> {
    let f = std::fs::read_to_string(data)?;
    let f = f.lines().collect::<Vec<&str>>();
    f.split(|line| line.is_empty())
        .map(|lines| {
            if let (Some(a), Some(b), Some(s)) = (lines.first(), lines.get(1), lines.get(2)) {
                if let (Some(a), Some(b), Some(s)) = (
                    a.strip_prefix("Button A: X"),
                    b.strip_prefix("Button B: X"),
                    s.strip_prefix("Prize: X="),
                ) {
                    let (a, b, s) = (
                        a.split(',').collect::<Vec<&str>>(),
                        b.split(',').collect::<Vec<&str>>(),
                        s.split(',').collect::<Vec<&str>>(),
                    );
                    let dxa = a[0]
                        .parse::<i32>()
                        .map_err(|_e| AOCError::ParseError("could not parse dxa".into()))?;
                    let dxb = b[0]
                        .parse::<i32>()
                        .map_err(|_e| AOCError::ParseError("could not parse dxb".into()))?;
                    let xs = s[0]
                        .parse::<i32>()
                        .map_err(|_e| AOCError::ParseError("could not parse xs".into()))?;
                    if let (Some(a), Some(b), Some(s)) = (a.get(1), b.get(1), s.get(1)) {
                        if let (Some(a), Some(b), Some(s)) = (
                            a.strip_prefix(" Y"),
                            b.strip_prefix(" Y"),
                            s.strip_prefix(" Y="),
                        ) {
                            let dya = a
                                .parse::<i32>()
                                .map_err(|_e| AOCError::ParseError("could not parse dya".into()))?;
                            let dyb = b
                                .parse::<i32>()
                                .map_err(|_e| AOCError::ParseError("could not parse dyb".into()))?;
                            let ys = s
                                .parse::<i32>()
                                .map_err(|_e| AOCError::ParseError("could not part ys".into()))?;
                            return Ok(ClawMachine {
                                prize: Point { x: xs, y: ys },
                                da: Point { x: dxa, y: dya },
                                db: Point { x: dxb, y: dyb },
                                solve_cost: SolveCost::Unspecified,
                            });
                        }
                    }
                }
            }
            Err(AOCError::ParseError("could not parse input".into()))
        })
        .collect::<Result<Vec<ClawMachine>>>()
}
