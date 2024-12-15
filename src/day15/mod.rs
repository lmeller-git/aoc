use super::{AOCError, Result};
use std::{
    collections::HashSet,
    fmt::Display,
    fs::read_to_string,
    ops::{Add, AddAssign},
    path::PathBuf,
};

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let mut warehouse = WareHouse::parse(data)?;
    if verbosity > 2 {
        println!("{}", warehouse);
        println!("solving");
    }
    warehouse.solve(verbosity);
    let res1 = warehouse.coords();
    println!("res1 {}", res1);
    Ok(())
}

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug)]
struct Robot {
    moves: Vec<Point>,
    position: Point,
}

#[derive(Debug)]
struct WareHouse {
    robot: Robot,
    boxes: HashSet<Point>,
    walls: HashSet<Point>,
    x_bounds: i32,
    y_bounds: i32,
}

impl WareHouse {
    fn parse(data: PathBuf) -> Result<Self> {
        let f = read_to_string(data)?;
        let l = f.lines().collect::<Vec<&str>>();
        let mut l = l.split(|line| line.is_empty());
        if let (Some(warehousemap), Some(moves)) = (l.next(), l.next()) {
            let mut walls: HashSet<Point> = HashSet::new();
            let mut boxes: HashSet<Point> = HashSet::new();
            let (mut x, mut y) = (0, 0);
            let mut robot_pos = Point::default();
            for row in warehousemap.iter() {
                x = 0;
                for tile in row.chars() {
                    match tile {
                        '#' => {
                            walls.insert(Point { x, y });
                        }
                        '@' => {
                            robot_pos.x = x;
                            robot_pos.y = y;
                        }
                        'O' => {
                            boxes.insert(Point { x, y });
                        }
                        _ => {}
                    }
                    x += 1;
                }
                y += 1;
            }

            let robot_moves = moves
                .concat()
                .chars()
                .filter_map(|m| match m {
                    '>' => Some(Point { x: 1, y: 0 }),
                    '<' => Some(Point { x: -1, y: 0 }),
                    '^' => Some(Point { x: 0, y: -1 }),
                    'v' => Some(Point { x: 0, y: 1 }),
                    _ => None,
                })
                .collect::<Vec<Point>>();

            return Ok(WareHouse {
                robot: Robot {
                    moves: robot_moves,
                    position: robot_pos,
                },
                boxes,
                walls,
                x_bounds: x,
                y_bounds: y,
            });
        }
        Err(AOCError::ParseError("could not parse input".into()))
    }

    fn solve(&mut self, verbosity: u8) {
        for next_move in self.robot.moves.clone().iter() {
            if verbosity > 3 {
                println!("{}", self);
            }
            self.tick(next_move);
        }
    }

    fn tick(&mut self, current_move: &Point) {
        let mut next_point = self.robot.position;
        loop {
            next_point += *current_move;
            if self.walls.contains(&next_point) {
                break;
            }
            if !self.boxes.contains(&next_point) {
                self.boxes.insert(next_point);
                self.boxes.remove(&(self.robot.position + *current_move));
                self.robot.position += *current_move;
                break;
            }
        }
    }

    fn coords(&self) -> i32 {
        let mut tot = 0;
        for p in self.boxes.iter() {
            tot += p.x + p.y * 100;
        }
        tot
    }
}

impl Display for WareHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.x_bounds {
            writeln!(f)?;
            for y in 0..self.y_bounds {
                let p = Point { x, y };
                if self.walls.contains(&p) {
                    write!(f, "#")?;
                } else if self.boxes.contains(&p) {
                    write!(f, "O")?;
                } else if p == self.robot.position {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}
