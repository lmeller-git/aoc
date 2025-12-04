use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{AOCError, Result};

pub fn _main(data: PathBuf, _verbosity: u8) -> Result<()> {
    let grid = parse(data)?;
    let accessible = part1(&grid);
    let res2 = part2(grid);
    println!("part1: {accessible}, part2: {res2}");
    Ok(())
}

type WareHouseGrid = Vec<Vec<bool>>;

fn part1(grid: &WareHouseGrid) -> u64 {
    let mut accessible = 0;
    for (i, row) in grid.iter().enumerate().skip(1).rev().skip(1) {
        'cols: for (j, cell) in row.iter().enumerate().skip(1).rev().skip(1) {
            if !cell {
                continue;
            }
            let mut neighbour_rolls = 0;
            for k in -1..=1 {
                for l in -1..=1 {
                    if k == 0 && l == 0 {
                        continue;
                    }
                    let i_idx = i as isize + k;
                    let j_idx = j as isize + l;
                    if grid[i_idx as usize][j_idx as usize] {
                        neighbour_rolls += 1;
                        if neighbour_rolls >= 4 {
                            continue 'cols;
                        }
                    }
                }
            }
            accessible += 1;
        }
    }
    accessible
}

fn part2(mut grid: WareHouseGrid) -> u64 {
    let mut accessible = 0;
    let mut has_removed;
    loop {
        has_removed = false;
        for i in 1..grid.len() - 1 {
            'cols: for j in 1..grid[0].len() - 1 {
                if !grid[i][j] {
                    continue;
                }
                let mut neighbour_rolls = 0;
                for k in -1..=1 {
                    for l in -1..=1 {
                        if k == 0 && l == 0 {
                            continue;
                        }
                        let i_idx = i as isize + k;
                        let j_idx = j as isize + l;
                        if grid[i_idx as usize][j_idx as usize] {
                            neighbour_rolls += 1;
                            if neighbour_rolls >= 4 {
                                continue 'cols;
                            }
                        }
                    }
                }
                accessible += 1;
                grid[i][j] = false;
                has_removed = true;
            }
        }
        if !has_removed {
            break;
        }
    }
    accessible
}
fn parse(data: PathBuf) -> Result<WareHouseGrid> {
    let f = File::open(data).map_err(|_| AOCError::ParseError("".into()))?;
    let reader = BufReader::new(f);
    let mut grid = WareHouseGrid::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let mut new_line = Vec::new();
        new_line.push(false);
        for c in line.chars() {
            if c == '@' {
                new_line.push(true);
            } else {
                new_line.push(false);
            }
        }
        new_line.push(false);
        grid.push(new_line);
    }
    grid.insert(0, vec![false; grid[0].len()]);
    grid.push(vec![false; grid[0].len()]);
    Ok(grid)
}
