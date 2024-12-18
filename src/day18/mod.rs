use super::{AOCError, Result};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    ops::Add,
    path::PathBuf,
};

const XBOUNDS: i32 = 70;
const YBOUNDS: i32 = 70;
const BYTES: u32 = 1024;

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let grid = parse(data)?;
    if verbosity > 1 {
        print_grid(&grid, &State::default());
        println!();
    }
    let res = astar(&grid, verbosity);
    println!();
    println!("res1: {}", res);
    Ok(())
}

#[derive(Hash, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

type Grid = HashMap<Point, u32>;

#[derive(Default, Debug, PartialEq, Eq)]
struct State {
    pos: Point,
    g_cost: u32,
    h_cost: u32,
    last_move: Point,
    visited: HashSet<Point>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g_cost + other.h_cost).cmp(&(self.h_cost + self.g_cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(grid: &Grid, verbosity: u8) -> u32 {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        pos: Point { x: 0, y: 0 },
        g_cost: 0,
        h_cost: get_cost(&Point { x: 0, y: 0 }),
        last_move: Point { x: 0, y: 0 },
        visited: HashSet::from([Point { x: 0, y: 0 }]),
    });
    let mut visited: HashMap<Point, u32> = HashMap::new();

    while let Some(next) = queue.pop() {
        if next.pos.x == XBOUNDS && next.pos.y == YBOUNDS {
            if verbosity > 2 {
                print_grid(grid, &next);
            }
            return next.g_cost;
        }
        if let Some(old) = visited.get(&next.pos) {
            if *old <= next.g_cost + next.h_cost {
                continue;
            }
        }
        visited
            .entry(next.pos)
            .and_modify(|e| *e = next.g_cost + next.h_cost)
            .or_insert(next.g_cost + next.h_cost);
        if let Some(next_s) = next_up(&next, grid) {
            queue.push(next_s);
        }
        if let Some(next_s) = next_down(&next, grid) {
            queue.push(next_s);
        }
        if let Some(next_s) = next_left(&next, grid) {
            queue.push(next_s);
        }
        if let Some(next_s) = next_right(&next, grid) {
            queue.push(next_s);
        }
    }

    0
}

fn next_up(state: &State, grid: &Grid) -> Option<State> {
    if state.pos.y == 0 {
        return None;
    }
    let down = Point { x: 0, y: 1 };
    let up = Point { x: 0, y: -1 };
    if state.last_move == down {
        return None;
    }
    if let Some(byte) = grid.get(&(state.pos + up)) {
        if *byte < BYTES {
            return None;
        }
    }
    let mut next = state.visited.clone();
    next.insert(state.pos + up);
    Some(State {
        pos: state.pos + up,
        g_cost: state.g_cost + 1,
        h_cost: get_cost(&(state.pos + up)),
        last_move: up,
        visited: next,
    })
}

fn next_down(state: &State, grid: &Grid) -> Option<State> {
    if state.pos.y == YBOUNDS {
        return None;
    }
    let down = Point { x: 0, y: 1 };
    let up = Point { x: 0, y: -1 };
    if state.last_move == up {
        return None;
    }
    if let Some(byte) = grid.get(&(state.pos + down)) {
        if *byte < BYTES {
            return None;
        }
    }
    let mut next = state.visited.clone();
    next.insert(state.pos + down);
    Some(State {
        pos: state.pos + down,
        g_cost: state.g_cost + 1,
        h_cost: get_cost(&(state.pos + down)),
        last_move: down,
        visited: next,
    })
}
fn next_right(state: &State, grid: &Grid) -> Option<State> {
    if state.pos.x == XBOUNDS {
        return None;
    }
    let left = Point { x: -1, y: 0 };
    let right = Point { x: 1, y: 0 };
    if state.last_move == left {
        return None;
    }
    if let Some(byte) = grid.get(&(state.pos + right)) {
        if *byte < BYTES {
            return None;
        }
    }
    let mut next = state.visited.clone();
    next.insert(state.pos + right);
    Some(State {
        pos: state.pos + right,
        g_cost: state.g_cost + 1,
        h_cost: get_cost(&(state.pos + right)),
        last_move: right,
        visited: next,
    })
}
fn next_left(state: &State, grid: &Grid) -> Option<State> {
    if state.pos.x == 0 {
        return None;
    }
    let left = Point { x: -1, y: 0 };
    let right = Point { x: 1, y: 0 };
    if state.last_move == right {
        return None;
    }
    if let Some(byte) = grid.get(&(state.pos + left)) {
        if *byte < BYTES {
            return None;
        }
    }
    let mut next = state.visited.clone();
    next.insert(state.pos + left);
    Some(State {
        pos: state.pos + left,
        g_cost: state.g_cost + 1,
        h_cost: get_cost(&(state.pos + left)),
        last_move: left,
        visited: next,
    })
}

fn get_cost(point: &Point) -> u32 {
    point.x.abs_diff(XBOUNDS) + point.y.abs_diff(YBOUNDS)
}

fn print_grid(grid: &Grid, state: &State) {
    for y in 0..=YBOUNDS {
        println!();
        for x in 0..=XBOUNDS {
            if state.visited.contains(&Point { x, y }) {
                print!("O");
            } else if let Some(byte) = grid.get(&Point { x, y }) {
                if *byte < BYTES {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
    }
}

fn parse(data: PathBuf) -> Result<Grid> {
    let f = fs::read_to_string(data)?;
    f.lines()
        .enumerate()
        .map(|(i, line)| {
            let mut l = line.split(',');
            if let (Some(x), Some(y)) = (l.next(), l.next()) {
                Ok((
                    Point {
                        x: x.parse::<i32>()
                            .map_err(|_e| AOCError::ParseError("could not parse x".into()))?,
                        y: y.parse::<i32>()
                            .map_err(|_e| AOCError::ParseError("could not parse y".into()))?,
                    },
                    i as u32,
                ))
            } else {
                Err(AOCError::ParseError("could not parse point".into()))
            }
        })
        .collect::<Result<Grid>>()
}
