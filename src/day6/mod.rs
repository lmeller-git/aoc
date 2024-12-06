use super::{AOCError, Result};
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn _main(data: PathBuf, _out: PathBuf, verbosity: u8) -> Result<()> {
    let mut field_map = Field::parse(data)?;
    let loops = field_map.count_loops(verbosity);
    while field_map.update().is_ok() {
        if verbosity > 1 {
            println!("{}", field_map);
        }
    }
    let res = field_map.count_visited();
    println!("part1: {}, part2: {}", res, loops);
    Ok(())
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum FieldState {
    Obstacle,
    Visited,
    #[default]
    Empty,
}

impl Display for FieldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Obstacle => write!(f, "#")?,
            Self::Visited => write!(f, "X")?,
            Self::Empty => write!(f, ".")?,
        }

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum GuardState {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Display for GuardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^")?,
            Self::Down => write!(f, "v")?,
            Self::Left => write!(f, "<")?,
            Self::Right => write!(f, ">")?,
        }
        Ok(())
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct GuardInfo {
    state: GuardState,
    position: (usize, usize),
}

type FieldMap = Vec<Vec<FieldState>>;

#[derive(Debug, Default, Clone)]
struct Field {
    field: FieldMap,
    guard: GuardInfo,
}

impl Field {
    fn parse(data: PathBuf) -> Result<Self> {
        let mut state = Field::default();
        let f = fs::File::open(data)?;
        let mut reader = io::BufReader::new(f);
        let mut buf = String::new();
        while reader.read_line(&mut buf)? > 0 {
            buf.retain(|c| c != '\n');
            state.field.push(
                buf.chars()
                    .map(|c| match c {
                        '.' => Ok(FieldState::Empty),
                        '#' => Ok(FieldState::Obstacle),
                        '>' => {
                            state.guard.state = GuardState::Left;
                            Ok(FieldState::Visited)
                        }
                        '<' => {
                            state.guard.state = GuardState::Right;
                            Ok(FieldState::Visited)
                        }
                        'v' => {
                            state.guard.state = GuardState::Down;
                            Ok(FieldState::Visited)
                        }
                        '^' => {
                            state.guard.state = GuardState::Up;
                            Ok(FieldState::Visited)
                        }
                        _ => Err(AOCError::ParseError("could not parse line".into())),
                    })
                    .collect::<Result<Vec<FieldState>>>()?,
            );
            buf.clear();
        }
        for (i, row) in state.field.iter().enumerate() {
            for (j, f) in row.iter().enumerate() {
                if *f == FieldState::Visited {
                    state.guard.position = (i, j);
                    break;
                }
            }
        }
        Ok(state)
    }

    fn update(&mut self) -> Result<()> {
        match self.guard.state {
            GuardState::Up => {
                if self.guard.position.0 == 0 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard.position.0 - 1][self.guard.position.1]
                    == FieldState::Obstacle
                {
                    self.guard.state = GuardState::Right;
                    return Ok(());
                }
                self.guard.position.0 -= 1;
                self.field[self.guard.position.0][self.guard.position.1] = FieldState::Visited;
            }
            GuardState::Down => {
                if self.guard.position.0 == self.field.len() - 1 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard.position.0 + 1][self.guard.position.1]
                    == FieldState::Obstacle
                {
                    self.guard.state = GuardState::Left;
                    return Ok(());
                }
                self.guard.position.0 += 1;
                self.field[self.guard.position.0][self.guard.position.1] = FieldState::Visited;
            }
            GuardState::Left => {
                if self.guard.position.1 == 0 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard.position.0][self.guard.position.1 - 1]
                    == FieldState::Obstacle
                {
                    self.guard.state = GuardState::Up;
                    return Ok(());
                }
                self.guard.position.1 -= 1;
                self.field[self.guard.position.0][self.guard.position.1] = FieldState::Visited;
            }
            GuardState::Right => {
                if self.guard.position.1 == self.field[0].len() - 1 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard.position.0][self.guard.position.1 + 1]
                    == FieldState::Obstacle
                {
                    self.guard.state = GuardState::Down;
                    return Ok(());
                }
                self.guard.position.1 += 1;
                self.field[self.guard.position.0][self.guard.position.1] = FieldState::Visited;
            }
        }
        Ok(())
    }

    fn count_loops(&self, verbosity: u8) -> u64 {
        let mut n_loops = 0;
        for i in 0..self.field.len() {
            for j in 0..self.field[0].len() {
                if self.field[i][j] == FieldState::Empty {
                    let mut new_map = self.clone();
                    new_map.field[i][j] = FieldState::Obstacle;
                    if new_map.is_loop(verbosity) {
                        n_loops += 1;
                    }
                }
            }
        }
        n_loops
    }

    fn is_loop(&mut self, verbosity: u8) -> bool {
        let mut current_guard = vec![self.guard.clone()];
        while self.update().is_ok() {
            if current_guard.contains(&self.guard) {
                if verbosity > 2 {
                    println!("loop found");
                    println!("{}", self);
                }
                return true;
            }
            current_guard.push(self.guard.clone());
        }
        false
    }

    fn count_visited(&self) -> u64 {
        self.field
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|field| **field == FieldState::Visited)
                    .count()
            })
            .sum::<usize>() as u64
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.field.iter().enumerate() {
            writeln!(f)?;
            for (j, field) in row.iter().enumerate() {
                if self.guard.position == (i, j) {
                    write!(f, "{} ", self.guard.state)?;
                    continue;
                }
                write!(f, "{} ", field)?;
            }
        }
        Ok(())
    }
}
