use super::{AOCError, Result};
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn _main(data: PathBuf, _out: PathBuf) -> Result<()> {
    let mut field_map = Field::parse(data)?;
    while field_map.update().is_ok() {
        //println!("{}", field_map);
    }
    let res = field_map.count_visited();
    println!("{}", res);
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

#[derive(Debug, Default)]
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
            Self::Left => write!(f, ">")?,
            Self::Right => write!(f, "<")?,
        }
        Ok(())
    }
}

type FieldMap = Vec<Vec<FieldState>>;

#[derive(Debug, Default)]
struct Field {
    field: FieldMap,
    guard_pos: (usize, usize),
    guard_state: GuardState,
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
                            state.guard_state = GuardState::Left;
                            Ok(FieldState::Visited)
                        }
                        '<' => {
                            state.guard_state = GuardState::Right;
                            Ok(FieldState::Visited)
                        }
                        'v' => {
                            state.guard_state = GuardState::Down;
                            Ok(FieldState::Visited)
                        }
                        '^' => {
                            state.guard_state = GuardState::Up;
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
                    state.guard_pos = (i, j);
                    break;
                }
            }
        }
        Ok(state)
    }

    fn update(&mut self) -> Result<()> {
        match self.guard_state {
            GuardState::Up => {
                if self.guard_pos.0 == 0 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard_pos.0 - 1][self.guard_pos.1] == FieldState::Obstacle {
                    self.guard_state = GuardState::Right;
                    return Ok(());
                }
                self.guard_pos.0 -= 1;
                self.field[self.guard_pos.0][self.guard_pos.1] = FieldState::Visited;
            }
            GuardState::Down => {
                if self.guard_pos.0 == self.field.len() - 1 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard_pos.0 + 1][self.guard_pos.1] == FieldState::Obstacle {
                    self.guard_state = GuardState::Left;
                    return Ok(());
                }
                self.guard_pos.0 += 1;
                self.field[self.guard_pos.0][self.guard_pos.1] = FieldState::Visited;
            }
            GuardState::Left => {
                if self.guard_pos.1 == 0 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard_pos.0][self.guard_pos.1 - 1] == FieldState::Obstacle {
                    self.guard_state = GuardState::Up;
                    return Ok(());
                }
                self.guard_pos.1 -= 1;
                self.field[self.guard_pos.0][self.guard_pos.1] = FieldState::Visited;
            }
            GuardState::Right => {
                if self.guard_pos.1 == self.field[0].len() - 1 {
                    return Err(AOCError::SolverError("Guard out of bounds".into()));
                }
                if self.field[self.guard_pos.0][self.guard_pos.1 + 1] == FieldState::Obstacle {
                    self.guard_state = GuardState::Down;
                    return Ok(());
                }
                self.guard_pos.1 += 1;
                self.field[self.guard_pos.0][self.guard_pos.1] = FieldState::Visited;
            }
        }
        Ok(())
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
                if self.guard_pos == (i, j) {
                    write!(f, "{}", self.guard_state)?;
                    continue;
                }
                write!(f, "{} ", field)?;
            }
        }
        Ok(())
    }
}
