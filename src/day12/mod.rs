use super::Result;
use std::{fmt::Display, path::PathBuf};

pub fn _main(data: PathBuf, _out: PathBuf, verbosity: u8) -> Result<()> {
    let mut garden = parse(data)?;
    let res1 = solve(&mut garden, verbosity);
    println!("res1: {}", res1);
    Ok(())
}

type Garden = Vec<Vec<PlantField>>;
type Plant = u8;
type Point = (usize, usize);

struct PlantField {
    plant_type: Plant,
    is_in_mask: bool,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Mask {
    area: usize,
    perimeter: usize,
    plant_type: Plant,
}

impl Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "area: {}", self.area)?;
        writeln!(f, "perimeter: {}", self.perimeter)?;
        writeln!(f, "type: {}", self.plant_type as char)?;
        Ok(())
    }
}

fn parse(data: PathBuf) -> Result<Garden> {
    let f = std::fs::read(data)?;
    Ok(f.split(|item| *item == b'\n')
        .map(|line| {
            line.iter()
                .map(|plant| PlantField {
                    plant_type: *plant,
                    is_in_mask: false,
                })
                .collect::<Vec<PlantField>>()
        })
        .filter(|line| !line.is_empty())
        .collect::<Garden>())
}

fn expand_selection(mask: &mut Mask, garden: &mut Garden, point: Point) {
    if garden[point.0][point.1].plant_type != mask.plant_type {
        mask.perimeter += 1;
        return;
    }
    if garden[point.0][point.1].is_in_mask {
        return;
    }
    mask.area += 1;
    garden[point.0][point.1].is_in_mask = true;
    for p in get_next_points(garden, &point, mask) {
        expand_selection(mask, garden, p);
    }
}

fn get_next_points(garden: &Garden, point: &(usize, usize), mask: &mut Mask) -> Vec<Point> {
    let mut points = Vec::new();
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
    } else {
        mask.perimeter += 1;
    }
    if point.0 < garden.len() - 1 {
        points.push((point.0 + 1, point.1));
    } else {
        mask.perimeter += 1;
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1));
    } else {
        mask.perimeter += 1;
    }
    if point.1 < garden[point.0].len() - 1 {
        points.push((point.0, point.1 + 1));
    } else {
        mask.perimeter += 1;
    }
    points
}

fn cost(mask: &Mask) -> usize {
    mask.perimeter * mask.area
}

fn solve(garden: &mut Garden, verbosity: u8) -> usize {
    let mut masks = Vec::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if garden[y][x].is_in_mask {
                continue;
            }
            let mut new_mask = Mask {
                plant_type: garden[y][x].plant_type,
                area: 0,
                perimeter: 0,
            };
            expand_selection(&mut new_mask, garden, (y, x));
            if verbosity > 2 {
                println!("new mask: {}", new_mask);
            }
            masks.push(new_mask);
        }
    }
    masks.iter().map(cost).sum()
}
