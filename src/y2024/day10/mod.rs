use super::Result;
use std::{collections::HashSet, path::PathBuf};

type HeightMap = Vec<Vec<u8>>;
type Position = (usize, usize);

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let height_map = parse(data)?;
    if verbosity > 2 {
        print_heightmap(&height_map);
    }
    let res = solve(&height_map);
    println!("res1: {}, res2: {}", res.0, res.1);
    Ok(())
}

fn print_heightmap(heights: &HeightMap) {
    for row in heights {
        println!();
        for height in row {
            print!("{}", height);
        }
    }
    println!();
}

fn parse(data: PathBuf) -> Result<HeightMap> {
    let f: Vec<u8> = std::fs::read(data)?;
    Ok(f.split(|item| *item == b'\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.iter().map(|c| c - 48).collect::<Vec<u8>>())
        .collect::<HeightMap>())
}

fn next_path(heights: &HeightMap, start: Position) -> Vec<Position> {
    let current_height = heights[start.0][start.1];
    let mut next_fields = Vec::new();
    if start.0 > 0 && heights[start.0 - 1][start.1] == current_height + 1 {
        next_fields.push((start.0 - 1, start.1));
    }
    if start.0 < heights.len() - 1 && heights[start.0 + 1][start.1] == current_height + 1 {
        next_fields.push((start.0 + 1, start.1));
    }
    if start.1 > 0 && heights[start.0][start.1 - 1] == current_height + 1 {
        next_fields.push((start.0, start.1 - 1));
    }

    if start.1 < heights[0].len() - 1 && heights[start.0][start.1 + 1] == current_height + 1 {
        next_fields.push((start.0, start.1 + 1));
    }
    next_fields
}

fn get_paths_unique(heights: &HeightMap, start: Position) -> Vec<Position> {
    if heights[start.0][start.1] == 9 {
        return vec![start];
    }
    let mut tot = HashSet::new();
    for next_start in next_path(heights, start) {
        for p in get_paths_unique(heights, next_start) {
            tot.insert(p);
        }
    }
    tot.into_iter().collect::<Vec<Position>>()
}

fn get_paths(heights: &HeightMap, start: Position) -> usize {
    if heights[start.0][start.1] == 9 {
        return 1;
    }
    let mut tot = 0;
    for next_start in next_path(heights, start) {
        tot += get_paths(heights, next_start);
    }
    tot
}

fn solve(heights: &HeightMap) -> (usize, usize) {
    let mut tot = 0;
    let mut tot2 = 0;
    for (i, row) in heights.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if *height == 0 {
                tot += get_paths_unique(heights, (i, j)).len();
                tot2 += get_paths(heights, (i, j));
            }
        }
    }
    (tot, tot2)
}
