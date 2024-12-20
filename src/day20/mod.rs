use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    ops::{Add, Sub},
    path::PathBuf,
};

use super::Result;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let grid = Grid::parse(data)?;
    //let res1 = solve_iter(&grid);
    let res1 = solve(&grid);
    println!("res1: {}", res1);
    Ok(())
}

fn solve_iter(grid: &Grid) -> usize {
    let worst_cheat = 20;
    let mut cheats: HashSet<Point> = HashSet::new();
    let baseline = solve_nocheat(grid);
    for wall in grid.walls.iter() {
        if let Some(new_grid) = cheated_grid(grid, wall) {
            if solve_nocheat(&new_grid) + worst_cheat <= baseline {
                cheats.insert(*wall);
            }
        }
    }
    cheats.len()
}

fn cheated_grid(grid: &Grid, wall: &Point) -> Option<Grid> {
    let mut is_viable = false;
    for delta in [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ] {
        let next_p = *wall + delta;
        if next_p.x < 0
            || next_p.x > grid.x_bounds
            || next_p.y < 0
            || next_p.y > grid.y_bounds
            || grid.walls.contains(&next_p)
        {
            continue;
        }
        if is_viable {
            let mut new_walls = grid.walls.clone();
            new_walls.remove(wall);
            return Some(Grid {
                walls: new_walls,
                start: grid.start,
                end: grid.end,
                x_bounds: grid.x_bounds,
                y_bounds: grid.y_bounds,
            });
        } else {
            is_viable = true;
        }
    }
    None
}

fn solve(grid: &Grid) -> usize {
    let no_cheat_length = solve_nocheat(grid);
    println!("n: {}", no_cheat_length);
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        g_cost: 0,
        cheat: Cheat::default(),
        current: grid.start,
        h_cost: grid.get_cost(&grid.start),
        cheat_time: 0,
    });
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut cheats: HashSet<Cheat> = HashSet::new();
    let mut cheated_states: HashSet<State> = HashSet::new();
    let best_cheat_times = 64;
    while let Some(next) = queue.pop() {
        if next.current == grid.end {
            println!("{}", cheats.len());
            if next.g_cost + best_cheat_times > no_cheat_length {
                break;
            }
            cheats.insert(next.cheat.clone());
            cheated_states.insert(next);
            //println!("{:#?}", cheats);

            continue;
        }
        if let Some(old) = visited.get(&next.current) {
            if *old + best_cheat_times < next.g_cost {
                continue;
            }
        }
        if cheats.contains(&next.cheat) {
            continue;
        }
        visited.insert(next.current, next.g_cost);
        for s in next_p(&next, grid, true) {
            queue.push(s);
        }
    }
    //println!("{:#?}", cheats);
    //println!("{:#?}", cheated_states);
    cheats.len()
}

fn solve_nocheat(grid: &Grid) -> i32 {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        g_cost: 0,
        cheat: Cheat::default(),
        current: grid.start,
        h_cost: grid.get_cost(&grid.start),
        cheat_time: 0,
    });
    let mut visited: HashMap<Point, i32> = HashMap::new();
    while let Some(next) = queue.pop() {
        if next.current == grid.end {
            return next.g_cost;
        }
        if let Some(old) = visited.get(&next.current) {
            if *old <= next.g_cost {
                continue;
            }
        }
        visited.insert(next.current, next.g_cost);
        for s in next_p(&next, grid, false) {
            queue.push(s);
        }
    }
    0
}

fn next_p(current: &State, grid: &Grid, cheat: bool) -> Vec<State> {
    let mut next_states = Vec::new();
    for delta in [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ] {
        let next_point = current.current + delta;
        let is_wall = grid.walls.contains(&next_point);
        if is_wall && cheat {
            if current.cheat_time >= 19 {
                continue;
            }
            if current.cheat_time == 0 {
                let cheat = Cheat {
                    start: Some(next_point),
                    end: None,
                };
                next_states.push(State {
                    h_cost: grid.get_cost(&next_point),
                    g_cost: current.g_cost + 1,
                    cheat,
                    current: next_point,
                    cheat_time: 1,
                });
            } else {
                let cheat = if current.cheat.end.is_none() {
                    current.cheat.clone()
                } else {
                    Cheat {
                        start: current.cheat.start,
                        end: None,
                    }
                };
                next_states.push(State {
                    h_cost: grid.get_cost(&next_point),
                    g_cost: current.g_cost + 1,
                    cheat,
                    current: next_point,
                    cheat_time: current.cheat_time + 1,
                });
            }
        } else if !is_wall {
            let next_cheat = if current.cheat.start.is_some() && current.cheat.end.is_none() {
                Cheat {
                    start: current.cheat.start,
                    end: Some(next_point),
                }
            } else {
                current.cheat.clone()
            };
            next_states.push(State {
                cheat_time: if current.cheat_time == 20 || current.cheat_time == 0 {
                    current.cheat_time
                } else {
                    current.cheat_time + 1
                },
                g_cost: current.g_cost + 1,
                h_cost: grid.get_cost(&next_point),
                cheat: next_cheat,
                current: next_point,
            });
        }
    }
    next_states
}

#[derive(Hash, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Default, Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Cheat {
    start: Option<Point>,
    end: Option<Point>,
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
struct State {
    h_cost: i32,
    g_cost: i32,
    cheat: Cheat,
    current: Point,
    cheat_time: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g_cost + other.h_cost).cmp(&(self.g_cost + self.h_cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Debug)]
struct Grid {
    walls: HashSet<Point>,
    start: Point,
    end: Point,
    x_bounds: i32,
    y_bounds: i32,
}

impl Grid {
    fn parse(data: PathBuf) -> Result<Self> {
        let f = fs::read_to_string(data)?;
        let f = f.lines();
        let mut grid = Grid::default();
        for (y, line) in f.enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.x_bounds = (x as i32).max(grid.x_bounds);
                match c {
                    '#' => {
                        grid.walls.insert(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    'S' => {
                        grid.start = Point {
                            x: x as i32,
                            y: y as i32,
                        }
                    }
                    'E' => {
                        grid.end = Point {
                            x: x as i32,
                            y: y as i32,
                        }
                    }
                    _ => continue,
                }
            }
            grid.y_bounds = (y as i32).max(grid.y_bounds)
        }

        Ok(grid)
    }

    fn get_cost(&self, point: &Point) -> i32 {
        (self.end.x.abs_diff(point.x) + self.end.y.abs_diff(point.y)) as i32
    }
}
