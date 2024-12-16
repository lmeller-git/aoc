use super::{AOCError, Result};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    fs,
    ops::Add,
    path::PathBuf,
};

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let maze = Maze::parse(data)?;
    if verbosity > 2 {
        println!("{}", maze);
    }
    let res1 = astar(&maze);
    println!("res1: {}", res1);
    Ok(())
}

fn astar(maze: &Maze) -> u32 {
    let mut queue: BinaryHeap<Reindeer> = BinaryHeap::new();
    queue.push(maze.start.clone());
    let mut visited: HashMap<(Point, Direction), u32> = HashMap::new();
    while let Some(next_state) = queue.pop() {
        if maze.is_solved(&next_state) {
            return next_state.g_cost;
        }
        if let Some(old_cost) = visited.get(&(next_state.position, next_state.direction)) {
            if *old_cost <= next_state.g_cost {
                continue;
            }
        }
        visited.insert(
            (next_state.position, next_state.direction),
            next_state.g_cost,
        );
        if let Some(r1) = rotate(maze, &next_state, Rotation::Clockwise) {
            queue.push(r1);
        }
        if let Some(r2) = rotate(maze, &next_state, Rotation::Counterclockwise) {
            queue.push(r2);
        }
        if let Some(w) = walk(maze, &next_state) {
            queue.push(w);
        }
    }
    0
}

fn rotate(maze: &Maze, reindeer: &Reindeer, rot: Rotation) -> Option<Reindeer> {
    if reindeer.last_move == Move::Rot {
        return None;
    }
    let mut r: Reindeer = match rot {
        Rotation::Clockwise => Reindeer {
            position: reindeer.position,
            direction: match reindeer.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            g_cost: reindeer.g_cost + 1000,
            h_cost: 0,
            last_move: Move::Rot,
        },

        Rotation::Counterclockwise => Reindeer {
            position: reindeer.position,
            direction: match reindeer.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            g_cost: reindeer.g_cost + 1000,
            h_cost: 0,
            last_move: Move::Rot,
        },
    };
    r.h_cost = maze.get_cost(&r);
    Some(r)
}

fn walk(maze: &Maze, reindeer: &Reindeer) -> Option<Reindeer> {
    let next_pos = reindeer.position.add_direction(&reindeer.direction);
    if maze.walls.contains(&next_pos) {
        return None;
    }
    let mut r = Reindeer {
        position: next_pos,
        direction: reindeer.direction,
        g_cost: reindeer.g_cost + 1,
        h_cost: 0,
        last_move: Move::Walk,
    };
    r.h_cost = maze.get_cost(&r);
    Some(r)
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
enum Move {
    Walk,
    Rot,
    #[default]
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    #[default]
    East,
    South,
    West,
}

#[derive(Hash, Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Default, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add_direction(&self, rhs: &Direction) -> Self {
        match rhs {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
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

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Reindeer {
    position: Point,
    direction: Direction,
    g_cost: u32,
    h_cost: u32,
    last_move: Move,
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g_cost + other.h_cost).cmp(&(self.g_cost + self.h_cost))
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
struct Maze {
    walls: HashSet<Point>,
    target: Point,
    start: Reindeer,
    max_x: usize,
    max_y: usize,
}

impl Maze {
    fn parse(data: PathBuf) -> Result<Self> {
        let f = fs::read(data)?;
        let mut maze = Maze::default();
        for (y, row) in f.split(|c| *c == b'\n').enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    b'#' => {
                        maze.walls.insert(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    b'E' => {
                        maze.target = Point {
                            x: x as i32,
                            y: y as i32,
                        };
                    }
                    b'S' => {
                        maze.start.position = Point {
                            x: x as i32,
                            y: y as i32,
                        };
                    }
                    b'.' => {}
                    _ => return Err(AOCError::ParseError("could not parse maze".into())),
                }
                maze.max_x = maze.max_x.max(x);
            }
            maze.max_y = maze.max_y.max(y);
        }
        maze.start.h_cost = maze.get_cost(&maze.start);
        Ok(maze)
    }

    fn is_solved(&self, reindeer: &Reindeer) -> bool {
        reindeer.position == self.target
    }

    fn get_cost(&self, reindeer: &Reindeer) -> u32 {
        self.target.x.abs_diff(reindeer.position.x) + self.target.y.abs_diff(reindeer.position.y)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            writeln!(f)?;
            for x in 0..=self.max_x {
                if self.walls.contains(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}
