use super::{AOCError, Result};
use std::{
    collections::HashSet,
    fmt::Display,
    fs::read_to_string,
    ops::{Add, AddAssign, Sub, SubAssign},
    path::PathBuf,
};

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let mut warehouse = WareHouse::parse(&data)?;
    if verbosity > 2 {
        println!("{}", warehouse);
        println!("solving");
    }
    warehouse.solve(verbosity);
    let mut warehouse2 = WareHouse::parse(&data)?.gen_part2();
    if verbosity > 2 {
        println!("part2:");
        println!("{}", warehouse2);
        println!("solving");
    }
    warehouse2.solve(verbosity);
    let res1 = warehouse.coords();
    let res2 = warehouse2.coords();
    println!("res1 {}, res2 {}", res1, res2);
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

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
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
    fat: bool,
}

impl WareHouse {
    fn parse(data: &PathBuf) -> Result<Self> {
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
                fat: false,
            });
        }
        Err(AOCError::ParseError("could not parse input".into()))
    }

    fn gen_part2(mut self) -> Self {
        if self.fat {
            return self;
        }
        self.fat = true;
        // fat boxes are anchored at a point x, y which corresponds to their left corner. only that point is stored
        let mut fat_boxes = HashSet::new();
        let mut fat_borders = HashSet::new();
        for x in 0..self.x_bounds {
            for y in 0..self.x_bounds {
                let p = Point { x, y };
                if self.boxes.contains(&p) {
                    fat_boxes.insert(Point { x: x * 2, y });
                } else if self.walls.contains(&p) {
                    fat_borders.insert(Point { x: x * 2, y });
                    fat_borders.insert(Point { x: x * 2 + 1, y });
                }
            }
        }
        self.boxes = fat_boxes;
        self.walls = fat_borders;
        self.x_bounds *= 2;
        self.robot.position.x *= 2;
        self
    }

    fn solve(&mut self, verbosity: u8) {
        for next_move in self.robot.moves.clone().iter() {
            if verbosity > 3 {
                println!("move: dx {}, dy {}", next_move.x, next_move.y);
            }
            if self.fat && next_move.x == 0 {
                if self.can_mv_box(next_move, self.robot.position + *next_move) {
                    let mut new_boxes = HashSet::new();
                    self.recursive_mv_box(
                        next_move,
                        self.robot.position + *next_move,
                        &mut new_boxes,
                    );
                    self.robot.position += *next_move;
                    self.boxes.extend(new_boxes);
                }
            } else {
                self.tick(next_move);
            }
            if verbosity > 3 {
                println!("{}", self);
            }
        }
    }

    fn tick(&mut self, current_move: &Point) {
        let mut next_point = self.robot.position;
        loop {
            next_point += *current_move;
            if self.walls.contains(&next_point) {
                break;
            }
            if !(self.boxes.contains(&next_point)
                || (self.fat
                    && self.boxes.contains(&Point {
                        x: next_point.x - 1,
                        y: next_point.y,
                    })))
            {
                if self.fat {
                    loop {
                        if next_point - *current_move == self.robot.position {
                            break;
                        }
                        if current_move.x == 1 {
                            next_point -= *current_move;
                        }
                        self.boxes.insert(next_point);
                        next_point -= *current_move;
                        self.boxes.remove(&next_point);
                        if current_move.x == -1 {
                            next_point -= *current_move;
                        }
                    }
                    self.boxes.remove(&next_point);
                } else {
                    self.boxes.insert(next_point);
                    self.boxes.remove(&(self.robot.position + *current_move));
                }
                self.robot.position += *current_move;
                break;
            }
        }
    }

    fn can_mv_box(&self, current_move: &Point, current_point: Point) -> bool {
        let next_point = current_point + *current_move;
        if self.walls.contains(&current_point) {
            return false;
        }
        if !self.boxes.contains(&current_point)
            && !self.boxes.contains(&Point {
                x: current_point.x - 1,
                y: current_point.y,
            })
        {
            return true;
        }

        self.can_mv_box(
            current_move,
            Point {
                x: if self.boxes.contains(&current_point) {
                    next_point.x + 1
                } else {
                    next_point.x - 1
                },
                y: next_point.y,
            },
        ) && self.can_mv_box(
            current_move,
            Point {
                x: next_point.x,
                y: next_point.y,
            },
        )
    }

    fn recursive_mv_box(
        &mut self,
        current_move: &Point,
        current_point: Point,
        new_boxes: &mut HashSet<Point>,
    ) {
        let next_point = current_point + *current_move;
        let mut on_spot = false;
        if self.boxes.contains(&current_point) {
            if self.boxes.remove(&current_point) {
                on_spot = true;
            }
            new_boxes.insert(next_point);
        } else if self.boxes.contains(&Point {
            x: current_point.x - 1,
            y: current_point.y,
        }) {
            self.boxes.remove(&Point {
                x: current_point.x - 1,

                y: current_point.y,
            });
            new_boxes.insert(Point {
                x: next_point.x - 1,
                y: next_point.y,
            });
        } else {
            return;
        }
        self.recursive_mv_box(current_move, next_point, new_boxes);
        self.recursive_mv_box(
            current_move,
            Point {
                x: if on_spot {
                    next_point.x + 1
                } else {
                    next_point.x - 1
                },
                y: next_point.y,
            },
            new_boxes,
        );
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
        for y in 0..self.y_bounds {
            writeln!(f)?;
            for x in 0..self.x_bounds {
                let p = Point { x, y };
                if self.walls.contains(&p) {
                    write!(f, "#")?;
                } else if self.boxes.contains(&p) {
                    if self.fat {
                        write!(f, "[")?;
                    } else {
                        write!(f, "O")?;
                    }
                } else if p == self.robot.position {
                    write!(f, "@")?;
                } else if self.fat && self.boxes.contains(&Point { x: x - 1, y }) {
                    write!(f, "]")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}
