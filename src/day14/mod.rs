use super::{AOCError, Result};
use std::{collections::HashMap, fmt::Display, path::PathBuf};

const MAX_X: i32 = 101 - 1;
const MAX_Y: i32 = 103 - 1;
const MIN_X: i32 = 0;
const MIN_Y: i32 = 0;
const MID_X: i32 = 50;
const MID_Y: i32 = 51;

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let mut area = Area::parse(data)?;
    let res1 = solve(&mut area, verbosity);
    println!("res1: {}, res2: {}", res1.0, res1.1);
    Ok(())
}

fn solve(robots: &mut Area, verbosity: u8) -> (usize, usize) {
    if verbosity > 2 {
        println!("{robots}");
    }
    let mut tree_idx = 0;
    let mut safety = 0;
    for i in 0..100000 {
        if verbosity > 1 && robots.is_c_tree() {
            println!("{i}");
            println!("{robots}");
            if tree_idx == 0 {
                tree_idx = i;
            }
            if i > 99 {
                break;
            }
        }
        robots.step();
        if i == 99 {
            safety = robots.safety_factor();
        }
        if verbosity > 2 {
            println!("{i}");
            println!("{robots}");
        }
    }
    (safety, tree_idx)
}

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut next_x = self.x + rhs.x;
        let mut next_y = self.y + rhs.y;
        if next_x > MAX_X {
            next_x -= MAX_X + 1;
        } else if next_x < MIN_X {
            next_x += MAX_X + 1;
        }
        if next_y > MAX_Y {
            next_y -= MAX_Y + 1;
        } else if next_y < MIN_Y {
            next_y += MAX_Y + 1;
        }
        Point {
            x: next_x,
            y: next_y,
        }
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Robot {
    velocity: Point,
}

#[derive(Default, Debug)]
struct Area {
    robots: HashMap<Point, Vec<Robot>>,
}

impl Area {
    fn parse(data: PathBuf) -> Result<Self> {
        let f = std::fs::read_to_string(data)?;
        Ok(Self {
            robots: f.lines().try_fold(HashMap::new(), |mut acc, line| {
                let mut l = line.split_whitespace();
                if let (Some(p), Some(v)) = (l.next(), l.next()) {
                    let mut p = p.strip_prefix("p=").expect("parsing failed").split(',');
                    let mut v = v.strip_prefix("v=").expect("parsing failed").split(',');
                    if let (Some(px), Some(py), Some(vx), Some(vy)) =
                        (p.next(), p.next(), v.next(), v.next())
                    {
                        let point = Point {
                            x: px
                                .parse::<i32>()
                                .map_err(|_e| AOCError::ParseError("could not parse px".into()))?,
                            y: py
                                .parse::<i32>()
                                .map_err(|_e| AOCError::ParseError("could not parse py".into()))?,
                        };

                        let robot = Robot {
                            //current: point.clone(),
                            velocity: Point {
                                x: vx.parse::<i32>().map_err(|_e| {
                                    AOCError::ParseError("could not parse px".into())
                                })?,
                                y: vy.parse::<i32>().map_err(|_e| {
                                    AOCError::ParseError("could not parse py".into())
                                })?,
                            },
                        };

                        acc.entry(point)
                            .and_modify(|e: &mut Vec<Robot>| e.push(robot.clone()))
                            .or_insert(vec![robot]);
                    }
                    Ok(acc)
                } else {
                    Err(AOCError::ParseError("could not parse line".into()))
                }
            })?,
        })
    }

    fn step(&mut self) {
        let mut next_area = HashMap::new();
        for (point, robots) in self.robots.iter() {
            for robot in robots.iter() {
                next_area
                    .entry(*point + robot.velocity)
                    .and_modify(|e: &mut Vec<Robot>| e.push(robot.clone()))
                    .or_insert(vec![robot.clone()]);
            }
        }
        self.robots = next_area;
    }

    fn safety_factor(&self) -> usize {
        let midpoint_x = MID_X;
        let midpoint_y = MID_Y;
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        for (point, robots) in self.robots.iter() {
            if point.x == midpoint_x || point.y == midpoint_y {
                continue;
            }
            if point.x < midpoint_x && point.y < midpoint_y {
                q1 += robots.len();
            } else if point.x > midpoint_x && point.y < midpoint_y {
                q2 += robots.len();
            } else if point.x < midpoint_x && point.y > midpoint_y {
                q3 += robots.len();
            } else if point.x > midpoint_x && point.y > midpoint_y {
                q4 += robots.len();
            }
        }
        q1 * q2 * q3 * q4
    }

    fn is_c_tree(&self) -> bool {
        let w_size_y = (MAX_Y - MIN_Y) / 10;
        let w_size_x = (MAX_X - MIN_X) / 10;
        let t = ((w_size_x * w_size_y) as f64 * 0.8) as usize;
        for y in MIN_Y..=MAX_Y / 10 {
            for x in MIN_X..=MAX_X / 10 {
                //calculate density around the point in a nxm window. if it exceeds some threshold, a tree is likely found
                let p0 = Point {
                    x: x * 10,
                    y: y * 10,
                };
                let mut n_robots = 0;
                for dx in 0..w_size_x {
                    for dy in 0..w_size_y {
                        if let Some(robots) = self.robots.get(&Point {
                            x: p0.x + dx,
                            y: p0.y + dy,
                        }) {
                            n_robots += robots.len();
                        }
                    }
                }
                if n_robots >= t {
                    return true;
                }
            }
        }
        false
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "area:")?;
        for i in MIN_Y..=MAX_Y {
            writeln!(f)?;
            for j in MIN_X..=MAX_X {
                if let Some(robots) = self.robots.get(&Point { x: j, y: i }) {
                    write!(f, "{}", robots.len())?;
                } else if i == MID_Y || j == MID_X {
                    write!(f, " ")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}
