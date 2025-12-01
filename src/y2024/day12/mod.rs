use super::Result;
use std::{collections::HashMap, fmt::Display, path::PathBuf};

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let mut garden = parse(data)?;
    let (res1, res2) = solve(&mut garden, verbosity);
    println!("res1: {}, res2: {}", res1, res2);
    Ok(())
}

type Garden = Vec<Vec<PlantField>>;
type Plant = u8;
type Point = (usize, usize);
type IPoint = (isize, isize);

#[derive(Debug, Default, PartialEq, Eq, Hash)]
enum RelativePosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

struct PlantField {
    plant_type: Plant,
    is_in_mask: bool,
}

#[derive(Default, Debug)]
struct Mask {
    area: usize,
    perimeter: usize,
    plant_type: Plant,
    sides: usize,
    outer: HashMap<RelativePosition, Vec<IPoint>>,
}

impl Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "area: {}", self.area)?;
        writeln!(f, "sides: {}", self.sides)?;
        writeln!(f, "perimeter: {}", self.perimeter)?;
        //writeln!(f, "outer: {:#?}", self.outer)?;
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

fn expand_selection(mask: &mut Mask, garden: &mut Garden, point: Point, parent: &Point) {
    if garden[point.0][point.1].plant_type != mask.plant_type {
        mask.perimeter += 1;
        if parent.0 > point.0 {
            mask.outer
                .entry(RelativePosition::Bottom)
                .and_modify(|e| e.push((point.0 as isize, point.1 as isize)))
                .or_insert(vec![(point.0 as isize, point.1 as isize)]);
        } else if parent.0 < point.0 {
            mask.outer
                .entry(RelativePosition::Top)
                .and_modify(|e| e.push((point.0 as isize, point.1 as isize)))
                .or_insert(vec![(point.0 as isize, point.1 as isize)]);
        } else if parent.1 > point.1 {
            mask.outer
                .entry(RelativePosition::Left)
                .and_modify(|e| e.push((point.0 as isize, point.1 as isize)))
                .or_insert(vec![(point.0 as isize, point.1 as isize)]);
        } else {
            mask.outer
                .entry(RelativePosition::Right)
                .and_modify(|e| e.push((point.0 as isize, point.1 as isize)))
                .or_insert(vec![(point.0 as isize, point.1 as isize)]);
        }
        return;
    }
    if garden[point.0][point.1].is_in_mask {
        return;
    }
    mask.area += 1;

    garden[point.0][point.1].is_in_mask = true;
    for p in get_next_points(garden, &point, mask) {
        expand_selection(mask, garden, p, &point);
    }
}

fn get_next_points(garden: &Garden, point: &Point, mask: &mut Mask) -> Vec<Point> {
    let mut points = Vec::new();
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
    } else {
        mask.perimeter += 1;
        let p = (point.0 as isize - 1, point.1 as isize);
        mask.outer
            .entry(RelativePosition::Top)
            .and_modify(|e| e.push(p))
            .or_insert(vec![p]);
    }
    if point.0 < garden.len() - 1 {
        points.push((point.0 + 1, point.1));
    } else {
        mask.perimeter += 1;

        let p = (point.0 as isize + 1, point.1 as isize);
        mask.outer
            .entry(RelativePosition::Bottom)
            .and_modify(|e| e.push(p))
            .or_insert(vec![p]);
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1));
    } else {
        mask.perimeter += 1;

        let p = (point.0 as isize, point.1 as isize - 1);
        mask.outer
            .entry(RelativePosition::Left)
            .and_modify(|e| e.push(p))
            .or_insert(vec![p]);
    }
    if point.1 < garden[point.0].len() - 1 {
        points.push((point.0, point.1 + 1));
    } else {
        mask.perimeter += 1;

        let p = (point.0 as isize, point.1 as isize + 1);
        mask.outer
            .entry(RelativePosition::Right)
            .and_modify(|e| e.push(p))
            .or_insert(vec![p]);
    }
    points
}

fn count_sides(mask: &mut Mask) {
    for (key, values) in &mut mask.outer {
        if values.len() == 1 {
            mask.sides += 1;
            continue;
        }
        values.sort_by(|a, b| match key {
            RelativePosition::Top | RelativePosition::Bottom => {
                a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
            }
            RelativePosition::Left | RelativePosition::Right => {
                a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
            }
        });
        let mut count = 0;
        for pair in values.windows(2) {
            let is_matching = match key {
                RelativePosition::Top | RelativePosition::Bottom => {
                    pair[1].1.abs_diff(pair[0].1) == 1 && pair[1].0 == pair[0].0
                }
                RelativePosition::Right | RelativePosition::Left => {
                    pair[1].0.abs_diff(pair[0].0) == 1 && pair[1].1 == pair[0].1
                }
            };
            if !is_matching {
                count += 1;
            }
        }
        mask.sides += count + 1;
    }
}

fn cost(mask: &Mask) -> usize {
    mask.perimeter * mask.area
}

fn reduced_cost(mask: &Mask) -> usize {
    mask.area * mask.sides
}

fn solve(garden: &mut Garden, verbosity: u8) -> (usize, usize) {
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
                sides: 0,
                outer: HashMap::new(),
            };
            expand_selection(&mut new_mask, garden, (y, x), &(y, x));
            count_sides(&mut new_mask);
            if verbosity > 2 {
                println!("new mask: {}", new_mask);
            }
            masks.push(new_mask);
        }
    }
    (
        masks.iter().map(cost).sum(),
        masks.iter().map(reduced_cost).sum(),
    )
}
