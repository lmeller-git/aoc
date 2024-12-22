use std::{cmp::Ordering, collections::BinaryHeap, fs, path::PathBuf};

use super::Result;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let codes = parse(data)?;
    let res1 = search(&codes);
    println!("res1: {}", res1);
    Ok(())
}
// find shortest path for each keypad recursively

fn search(targets: &Vec<Code>) -> usize {
    let mut tot = 0;
    for target in targets {
        let n0 = shortest_path(target, true);
        let n1 = shortest_path(&n0, false);
        let n2 = shortest_path(&n1, false);
        tot += get_complexity(target, n2.len());
    }
    tot
}

fn shortest_path(target: &Code, is_numeric: bool) -> Code {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        g_cost: 0,
        current_seq: Vec::new(),
        position: Button::Enter,
        moves: Vec::new(),
    });

    while let Some(current) = queue.pop() {
        if current.current_seq == *target {
            return current.moves;
        }

        for next in next_pos(&current, target, is_numeric) {
            queue.push(next);
        }
    }

    Vec::new()
}

fn next_pos(current: &State, target: &Code, is_numeric: bool) -> Vec<State> {
    let mut next_states = Vec::new();
    for direction in [Button::Up, Button::Down, Button::Left, Button::Right] {
        if let Some(next_pos) = current.position.get_valid(&direction, is_numeric) {
            let mut next_moves = current.moves.clone();
            next_moves.push(direction);
            let mut next_seq = current.current_seq.clone();
            let cost = if next_pos == target[current.current_seq.len()] {
                next_moves.push(Button::Enter);
                next_seq.push(next_pos);
                current.g_cost + 2
            } else {
                current.g_cost + 1
            };

            next_states.push(State {
                g_cost: cost,
                current_seq: next_seq,
                position: next_pos,
                moves: next_moves,
            });
        }
    }
    next_states
}

fn get_complexity(_code: &Code, _length: usize) -> usize {
    0
}

type Code = Vec<Button>;

#[derive(Debug, PartialEq, Eq)]
struct State {
    g_cost: usize,
    current_seq: Code,
    position: Button, //hovering
    moves: Code,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.g_cost.cmp(&self.g_cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Button {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Num(i8),
}

impl Button {
    fn get_valid(&self, other: &Button, is_numeric: bool) -> Option<Button> {
        match self {
            Self::Num(n) => {
                if *n == 0 {
                    return match other {
                        Self::Up => Some(Self::Num(2)),
                        Self::Right => Some(Self::Enter),
                        _ => None,
                    };
                }
                match other {
                    Self::Up => {
                        if n + 3 < 10 {
                            Some(Self::Num(n + 1))
                        } else {
                            None
                        }
                    }
                    Self::Down => match (n - 3).cmp(&0) {
                        Ordering::Greater => Some(Self::Num(n - 3)),
                        Ordering::Equal => Some(Self::Enter),
                        Ordering::Less => None,
                    },
                    Self::Right => match n {
                        1 | 2 | 4 | 5 | 7 | 8 => Some(Self::Num(n + 1)),
                        _ => None,
                    },
                    Self::Left => match n {
                        9 | 6 | 3 | 2 | 5 | 8 => Some(Self::Num(n - 1)),
                        _ => None,
                    },
                    _ => None,
                }
            }
            Self::Up => match other {
                Self::Down => Some(Self::Down),
                Self::Right => Some(Self::Enter),
                _ => None,
            },
            Self::Down => match other {
                Self::Down => None,
                Self::Up | Self::Left | Self::Right => Some(*other),
                _ => None,
            },
            Self::Left => match other {
                Self::Right => Some(Self::Down),
                _ => None,
            },
            Self::Right => match other {
                Self::Up => Some(Self::Enter),
                Self::Left => Some(Self::Down),
                _ => None,
            },
            Self::Enter => match other {
                Self::Up => {
                    if is_numeric {
                        Some(Self::Num(3))
                    } else {
                        None
                    }
                }
                Self::Down => {
                    if is_numeric {
                        None
                    } else {
                        Some(Self::Right)
                    }
                }
                Self::Left => {
                    if is_numeric {
                        Some(Self::Num(0))
                    } else {
                        Some(Self::Up)
                    }
                }
                _ => None,
            },
        }
    }
}

fn parse(data: PathBuf) -> Result<Vec<Code>> {
    let f = fs::read_to_string(data)?;
    Ok(f.lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        c.to_digit(10).map(|digit| Button::Num(digit as i8))
                    } else if c == 'A' {
                        Some(Button::Enter)
                    } else {
                        None
                    }
                })
                .collect::<Code>()
        })
        .collect::<Vec<Code>>())
}
