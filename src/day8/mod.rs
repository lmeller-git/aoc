use super::Result;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

pub fn _main(data: PathBuf, _out: PathBuf, _verbosity: u8) -> Result<()> {
    let grid = Grid::parse(data)?;
    let res = grid.get_antinodes();
    println!("{}", res);
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
        (0. - f64::EPSILON..0. + f64::EPSILON).contains(&d)
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
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
        })
    }

    fn get_antinodes(&self) -> u32 {
        let mut tot = 0;
        let mut found = false;
        for x in 0..self.rows {
            for y in 0..self.cols {
                let p = Point {
                    x: x as f64,
                    y: y as f64,
                };
                for antenna_group in &self.antennas {
                    if found {
                        found = false;
                        break;
                    }
                    for antenna in antenna_group.1 {
                        if found {
                            break;
                        }
                        if p == antenna.pos {
                            continue;
                        }
                        let line = Line::from_points(&p, &antenna.pos);
                        let d = p.distance(&antenna.pos);
                        for other_antenna in antenna_group.1 {
                            if antenna == other_antenna || p == other_antenna.pos {
                                continue;
                            }
                            if other_antenna.pos.is_on_line(&line) {
                                let d2 = p.distance(&other_antenna.pos);
                                if (0. - f64::EPSILON..0. + f64::EPSILON)
                                    .contains(&(d2.max(d) - 2. * d2.min(d)))
                                {
                                    found = true;
                                    tot += 1;
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
