use super::{AOCError, Result};
use std::{cmp::Ordering, fmt::Display, path::PathBuf};

pub fn _main(data: PathBuf, _out: PathBuf, verbosity: u8) -> Result<()> {
    println!("part1");
    let mut fs = FileSystem::parse(&data)?;
    fs.compact(verbosity);
    let res = fs.get_checksum();
    if verbosity > 3 {
        println!("part1:");
        println!("{}", fs);
        println!();
    }
    println!("full");
    let mut fs2 = FileSystem::parse(&data)?;
    fs2.compact_stable(verbosity);
    let res2 = fs2.get_checksum();
    if verbosity > 3 {
        println!("full:");
        println!("{}", fs2);
        println!();
    }
    println!("part2");
    let mut fs3 = FileSystem::parse(&data)?;
    fs3.compact_once_stable(verbosity);
    if verbosity > 3 {
        println!("part2: ");
        println!("{}", fs3);
        println!();
    }
    let res3 = fs3.get_checksum();
    println!("part1 res: {}, res2: {}, part2 res: {}", res, res2, res3);
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
    fn parse(data: &PathBuf) -> Result<Self> {
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
            let last_file = self.nth_last_non_empty(0);
            let first_free = self.nth_empty_with_size(0, 0);
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

    fn nth_empty_with_size(&self, mut n: usize, size: i8) -> usize {
        for (i, f) in self.files.iter().enumerate() {
            match f {
                File::Empty { length: _ } => {
                    if n == 0 {
                        if let Some(File::Empty { length }) =
                            self.files.get(self.files.len() - (i + 1))
                        {
                            if *length < size {
                                continue;
                            }
                        }
                        return i;
                    } else {
                        n -= 1;
                    }
                }
                File::NonEmpty { length: _, id: _ } => {}
            }
        }
        self.files.len() - 1
    }

    fn nth_last_non_empty(&self, mut n: usize) -> usize {
        for (i, f) in self.files.iter().rev().enumerate() {
            match f {
                File::Empty { length: _ } => {}
                File::NonEmpty { length: _, id: _ } => {
                    if n == 0 {
                        return self.files.len() - (i + 1);
                    } else {
                        n -= 1;
                    }
                }
            }
        }
        0
    }

    fn get_checksum(&self) -> u64 {
        let mut idx: u64 = 0;
        let mut tot = 0;
        for f in self.files.iter() {
            match f {
                File::Empty { length: l } => {
                    idx += *l as u64;
                }
                File::NonEmpty { length: l, id: i } => {
                    let null_idx = idx;
                    while ((idx - null_idx) as i8) < *l {
                        tot += idx * *i as u64;
                        idx += 1;
                    }
                }
            }
        }
        tot
    }

    fn compact_once_stable(&mut self, verbosity: u8) {
        let mut i = 0;
        while i < self.files.len() {
            // get file n
            if verbosity > 4 {
                println!("{self}");
            }
            self.combine_buffers(verbosity);
            let next_file = self.nth_last_non_empty(i);
            if next_file == 0 {
                return;
            }
            for j in 0..next_file {
                // find a fitting buffer left of it
                let current_buf = self.nth_empty_with_size(j, 0);
                if current_buf >= next_file {
                    i += 1;
                    break;
                }
                let (delta, len) = match (self.files.get(next_file), self.files.get(current_buf)) {
                    (
                        Some(File::NonEmpty {
                            length: last_length,
                            id: _file_id,
                        }),
                        Some(File::Empty {
                            length: free_length,
                        }),
                    ) => {
                        if *last_length > *free_length {
                            // buffer to small
                            continue;
                        }
                        (*free_length - *last_length, *last_length)
                    }
                    (None, None) => panic!("both none returned"),
                    (None, _) => panic!("no file"),
                    (_, None) => panic!("no buff"),
                    _ => panic!("Unexpected file type during compaction"),
                };
                if delta > 0 {
                    // buffer to large -> file gets moved here and a new buffer gets initialized at the old file position
                    if let Some(File::Empty { length }) = self.files.get_mut(current_buf) {
                        *length -= len;
                    }
                    let f = self.files.remove(next_file);
                    self.files.insert(current_buf, f);
                    self.files.insert(next_file, File::Empty { length: len });
                    break;
                } else {
                    // the buffer gets filled completely
                    self.files[current_buf] = self.files.remove(next_file);
                    self.files.insert(next_file, File::Empty { length: len });
                    break;
                }
            }
        }
    }

    fn combine_buffers(&mut self, verbosity: u8) {
        let mut i = 0;
        while i < self.files.len() - 1 {
            if verbosity > 6 {
                println!(" w: {}", self);
            }
            let l1 = if let Some(File::Empty { length: l1 }) = self.files.get(i) {
                *l1
            } else {
                i += 1;
                continue;
            };
            if let Some(File::Empty { length: l2 }) = self.files.get_mut(i + 1) {
                *l2 += l1;
            } else {
                i += 1;
                continue;
            }
            self.files.remove(i);
        }
    }

    fn compact_stable(&mut self, verbosity: u8) {
        let mut n: usize = 0;
        let mut size = 0;
        loop {
            self.combine_buffers(verbosity);
            if verbosity > 5 {
                println!("{}", self);
            }
            let current_buf = self.nth_empty_with_size(n, size);
            if current_buf == self.files.len() - 1 {
                break;
            }
            for i in 0..self.files.len() {
                let next_file = self.nth_last_non_empty(i);
                if next_file <= current_buf {
                    n += 1;
                    size += 1;
                    break;
                }

                let (delta, len) = match (self.files.get(next_file), self.files.get(current_buf)) {
                    (
                        Some(File::NonEmpty {
                            length: last_length,
                            id: _file_id,
                        }),
                        Some(File::Empty {
                            length: free_length,
                        }),
                    ) => {
                        if *last_length > *free_length {
                            continue;
                        }
                        (*free_length - *last_length, *last_length)
                    }
                    (None, None) => panic!("both none returned"),
                    (None, _) => panic!("no file"),
                    (_, None) => panic!("no buff"),
                    _ => panic!("Unexpected file type during compaction"),
                };
                if delta > 0 {
                    if let Some(File::Empty { length }) = self.files.get_mut(current_buf) {
                        *length -= len;
                    }
                    if let Some(File::Empty { length }) = self.files.get_mut(next_file) {
                        *length += len;
                    }
                    let f = self.files.remove(next_file);
                    self.files.insert(current_buf, f);
                    break;
                } else {
                    self.files[current_buf] = self.files.remove(next_file);
                    if let Some(File::Empty { length }) = self.files.get_mut(next_file) {
                        *length += len;
                    }
                    break;
                }
            }
        }
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
