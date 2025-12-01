use super::Result;
use core::f64;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::path::PathBuf;

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let mut grid = Grid::parse(data)?;
    let res = grid.get_antinodes(false);
    if verbosity > 2 {
        println!("part1:");
        println!("{}\n", grid);
    }
    grid.antinodes.clear();
    let res2 = grid.get_antinodes(true);
    if verbosity > 2 {
        println!("part2:");
        println!("{}\n", grid);
    }
    println!("part1: {}, part2: {}", res, res2);
    Ok(())
}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn is_on_line(&self, line: &Line) -> bool {
        let d = (line.pos.x - self.x) * line.delta.y - (line.pos.y - self.y) * line.delta.x;
        (-f64::EPSILON..f64::EPSILON).contains(&d)
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn is_equal(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= f64::EPSILON && (self.y - other.y).abs() <= f64::EPSILON
    }
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
struct Line {
    delta: Point,
    pos: Point,
}

impl Line {
    fn from_points(p1: &Point, p2: &Point) -> Self {
        Self {
            delta: Point {
                x: p1.x - p2.x,
                y: p1.y - p2.y,
            },
            pos: p1.clone(),
        }
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
struct Antenna {
    pos: Point,
}

#[derive(Default, Debug)]
struct Grid {
    antennas: HashMap<char, Vec<Antenna>>,
    cols: usize,
    rows: usize,
    antinodes: Vec<Point>,
}

impl Grid {
    fn parse(data: PathBuf) -> Result<Self> {
        let mut f = std::fs::File::open(data)?;
        let mut buf = String::new();
        let _n = f.read_to_string(&mut buf)?;
        let rows = buf.lines().count();
        let cols = buf.lines().next().unwrap().len();
        let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
        for (i, line) in buf.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .and_modify(|entry| {
                            entry.push(Antenna {
                                pos: Point {
                                    x: j as f64,
                                    y: i as f64,
                                },
                            })
                        })
                        .or_insert(vec![Antenna {
                            pos: Point {
                                x: j as f64,
                                y: i as f64,
                            },
                        }]);
                }
            }
        }
        Ok(Self {
            antennas,
            cols,
            rows,
            antinodes: Vec::new(),
        })
    }

    fn get_antinodes(&mut self, resonant_harmonics: bool) -> u32 {
        let mut tot = 0;
        for x in 0..self.rows {
            for y in 0..self.cols {
                let mut found = false;
                let p = Point {
                    x: x as f64,
                    y: y as f64,
                };
                for antenna_group in &self.antennas {
                    if found {
                        break;
                    }
                    for (i, antenna) in antenna_group.1.iter().enumerate() {
                        if found {
                            break;
                        }
                        if resonant_harmonics
                            && antenna_group.1.len() > 1
                            && p.is_equal(&antenna.pos)
                        {
                            tot += 1;
                            found = true;
                            self.antinodes.push(p.clone());
                            break;
                        }
                        if !resonant_harmonics && p.is_equal(&antenna.pos) {
                            continue;
                        }
                        let line = Line::from_points(&p, &antenna.pos);
                        let d = p.distance(&antenna.pos);
                        for other_antenna in antenna_group.1.iter().skip(i + 1) {
                            if !resonant_harmonics && p.is_equal(&other_antenna.pos) {
                                continue;
                            }
                            if other_antenna.pos.is_on_line(&line) {
                                let d2 = p.distance(&other_antenna.pos);
                                if resonant_harmonics
                                    || (d2.max(d) - 2. * d2.min(d)).abs() <= f64::EPSILON
                                {
                                    found = true;
                                    tot += 1;
                                    self.antinodes.push(p.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        tot
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![vec!['.'; self.cols]; self.rows];
        for p in &self.antinodes {
            v[p.y as usize][p.x as usize] = '#';
        }
        for a in &self.antennas {
            for a_ in a.1 {
                v[a_.pos.y as usize][a_.pos.x as usize] = *a.0;
            }
        }
        for r in &v {
            writeln!(f)?;
            for c in r {
                write!(f, "{}", c)?;
            }
        }

        Ok(())
    }
}
