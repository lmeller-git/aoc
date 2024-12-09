use super::{AOCError, Result};
use std::{cmp::Ordering, fmt::Display, path::PathBuf};

pub fn _main(data: PathBuf, _out: PathBuf, verbosity: u8) -> Result<()> {
    let mut fs = FileSystem::parse(data)?;
    fs.compact(verbosity);
    let res = fs.get_checksum();
    if verbosity > 3 {
        println!("{}", fs);
    }
    println!("res: {}", res);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum File {
    Empty { length: i8 },
    NonEmpty { length: i8, id: usize },
}
#[derive(Debug)]
struct FileSystem {
    files: Vec<File>,
}

impl FileSystem {
    fn parse(data: PathBuf) -> Result<Self> {
        let f = std::fs::read_to_string(data)?;
        let mut files =
            f.chars()
                .filter(|c| *c != '\n')
                .enumerate()
                .map(|(i, c)| {
                    let n =
                        c.to_digit(10).ok_or(c).map_err(|e| {
                            AOCError::ParseError(format!("could not parse length, {e}"))
                        })? as i8;
                    if i % 2 == 0 {
                        Ok(File::NonEmpty {
                            length: n,
                            id: i / 2,
                        })
                    } else {
                        Ok(File::Empty { length: n })
                    }
                })
                .collect::<Result<Vec<File>>>()?;
        if let Some(File::NonEmpty { length: _, id: _ }) = files.last() {
            files.push(File::Empty { length: 0 });
        }

        Ok(Self { files })
    }

    fn compact(&mut self, verbosity: u8) {
        while !self.is_compact() {
            if verbosity > 4 {
                println!("{}", self);
            }
            let last_file = self.last_non_empty();
            let first_free = self.first_empty();
            let (delta, id, len) = match (self.files.get(last_file), self.files.get(first_free)) {
                (
                    Some(File::NonEmpty {
                        length: last_length,
                        id: file_id,
                    }),
                    Some(File::Empty {
                        length: free_length,
                    }),
                ) => (*free_length - *last_length, *file_id, *last_length),
                _ => panic!("Unexpected file type during compaction"),
            };
            match delta.cmp(&0) {
                Ordering::Greater => {
                    if let Some(File::Empty { length }) = self.files.get_mut(first_free) {
                        *length -= len;
                    }
                    if let Some(File::Empty { length }) = self.files.last_mut() {
                        *length += len;
                    }
                    let f = self.files.remove(last_file);
                    self.files.insert(first_free, f);
                }
                Ordering::Equal => {
                    self.files[first_free] = self.files.remove(last_file);
                    if let Some(File::Empty { length }) = self.files.last_mut() {
                        *length += len;
                    }
                }
                Ordering::Less => {
                    let new_file = File::NonEmpty {
                        length: len + delta,
                        id,
                    };
                    if let Some(File::Empty { length }) = self.files.last_mut() {
                        *length += len + delta;
                    }
                    if let Some(File::NonEmpty { length, id: _ }) = self.files.get_mut(last_file) {
                        *length -= len + delta;
                    }
                    self.files[first_free] = new_file;
                }
            }
        }
    }

    fn is_compact(&self) -> bool {
        let mut last_empty = false;
        for f in self.files.iter() {
            match f {
                File::Empty { length: _ } => {
                    last_empty = true;
                }
                File::NonEmpty { length: _, id: _ } => {
                    if last_empty {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn first_empty(&self) -> usize {
        for (i, f) in self.files.iter().enumerate() {
            match f {
                File::Empty { length: _ } => return i,
                File::NonEmpty { length: _, id: _ } => {}
            }
        }
        self.files.len() - 1
    }

    fn last_non_empty(&self) -> usize {
        for (i, f) in self.files.iter().rev().enumerate() {
            match f {
                File::Empty { length: _ } => {}
                File::NonEmpty { length: _, id: _ } => return self.files.len() - (i + 1),
            }
        }
        0
    }

    fn get_checksum(&self) -> u64 {
        let mut idx: u64 = 0;
        let mut tot = 0;
        let mut file_iter = self.files.iter();
        while let Some(File::NonEmpty {
            length: len,
            id: file_id,
        }) = file_iter.next()
        {
            let null_idx = idx;
            while ((idx - null_idx) as i8) < *len {
                tot += idx * *file_id as u64;
                idx += 1;
            }
        }
        tot
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file in self.files.iter() {
            match file {
                File::Empty { length: l } => {
                    for _ in 0..*l {
                        write!(f, ".")?;
                    }
                }
                File::NonEmpty { length: l, id: i } => {
                    for _ in 0..*l {
                        write!(f, "{i}")?;
                    }
                }
            }
        }
        Ok(())
    }
}
