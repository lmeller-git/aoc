use std::{fs, path::PathBuf};

use super::Result;

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let (keys, locks) = parse(data)?;
    let res1 = matching_keylocks(&keys, &locks);
    println!("res1: {}", res1);
    Ok(())
}

fn matching_keylocks(keys: &[KeyLock], locks: &[KeyLock]) -> usize {
    let mut tot = 0;
    for k in keys.iter() {
        for l in locks.iter() {
            if is_matching(k, l) {
                tot += 1;
            }
        }
    }
    tot
}

fn is_matching(key: &KeyLock, lock: &KeyLock) -> bool {
    for (k, l) in key.iter().zip(lock.iter()) {
        if k + l > 5 {
            return false;
        }
    }
    true
}

type KeyLock = [u8; 5];

fn parse(data: PathBuf) -> Result<(Vec<KeyLock>, Vec<KeyLock>)> {
    let f = fs::read_to_string(data)?;
    let f = f.lines().collect::<Vec<&str>>();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in f.split(|line| line.is_empty()) {
        let mut keylock = [0, 0, 0, 0, 0];
        for line in block.iter() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    keylock[i] += 1;
                }
            }
        }
        keylock.iter_mut().for_each(|item| *item -= 1);
        if block[0].contains('.') {
            locks.push(keylock);
        } else {
            keys.push(keylock);
        }
    }
    Ok((keys, locks))
}
