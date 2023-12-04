use advent_of_code_common::file::split_lines_trim;
use anyhow::{Context, Result};
use itertools::{iproduct, Itertools};
use std::{collections::HashSet, hash::Hash};

const PATH: &str = "inputs/day14.txt";
const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (x, y) = s
            .trim()
            .split(',')
            .filter_map(|v| v.trim().parse().ok())
            .collect_tuple()
            .ok_or("Invalid point")?;
        Ok(Self { x, y })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(&self) -> HashSet<Point> {
        let (min_x, max_x, min_y, max_y) = if self.start.x == self.end.x {
            (
                self.start.x,
                self.start.x,
                self.start.y.min(self.end.y),
                self.start.y.max(self.end.y),
            )
        } else {
            (
                self.start.x.min(self.end.x),
                self.start.x.max(self.end.x),
                self.start.y,
                self.start.y,
            )
        };
        iproduct!(min_x..=max_x, min_y..=max_y)
            .map(|(x, y)| Point { x, y })
            .collect()
    }
}

#[derive(Debug)]
struct Rock {
    lines: Vec<Line>,
}

impl Rock {
    fn from_str(s: &str) -> Self {
        let corners: Vec<Point> = s
            .split("->")
            .filter_map(|p| Point::try_from(p).ok())
            .collect();
        Self {
            lines: corners
                .windows(2)
                .map(|corner| Line {
                    start: corner[0],
                    end: corner[1],
                })
                .collect(),
        }
    }

    fn points(&self) -> HashSet<Point> {
        self.lines.iter().flat_map(Line::points).collect()
    }
}

struct Cave {
    rocks: HashSet<Point>,
    sand: HashSet<Point>,
    max_y: usize,
}

impl Cave {
    fn points(&self) -> HashSet<Point> {
        let mut sand = self.sand.clone();
        sand.extend(self.rocks.clone());
        sand
    }

    fn drop_sand(&mut self, origin: &Point) -> Point {
        let mut sand = *origin;
        let points = self.points();
        loop {
            let y = sand.y + 1;
            let down = Point { x: sand.x, y };
            let left = Point { x: sand.x - 1, y };
            let right = Point { x: sand.x + 1, y };
            let targets = (
                points.contains(&left),
                points.contains(&down),
                points.contains(&right),
            );
            sand = match targets {
                (_, false, _) => down,
                (false, true, _) => left,
                (true, true, false) => right,
                (true, true, true) => {
                    self.sand.insert(sand);
                    return sand;
                }
            };
            if sand.y == self.max_y {
                self.sand.insert(sand);
                return sand;
            }
        }
    }
}

fn input(example: bool) -> Result<Cave> {
    let text = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).context("Failed to read input file")?
    };
    let rocks: HashSet<_> = split_lines_trim(&text)
        .iter()
        .flat_map(|row| Rock::from_str(row).points())
        .collect();
    let sand = HashSet::new();
    let max_y = rocks
        .iter()
        .max_by_key(|p| p.y)
        .ok_or_else(|| anyhow::anyhow!("No rocks"))?
        .y;
    Ok(Cave { rocks, sand, max_y })
}

fn part_1(cave: &mut Cave) -> usize {
    let origin = Point { x: 500, y: 0 };
    let mut last_drop = origin;
    while last_drop.y < cave.max_y {
        last_drop = cave.drop_sand(&origin);
    }
    cave.sand.len() - 1
}

fn part_2(cave: &mut Cave) -> usize {
    cave.max_y += 1;
    let origin = Point { x: 500, y: 0 };
    let mut last_drop = Point { x: 0, y: 0 };
    let mut counter = 0;
    while last_drop != origin {
        counter += 1;
        last_drop = cave.drop_sand(&origin);
    }
    counter
}

#[test]
fn example_1() {
    let mut cave = input(true).unwrap();
    assert_eq!(part_1(&mut cave), 24);
}

#[test]
fn task_1() {
    let mut cave = input(false).unwrap();
    assert_eq!(part_1(&mut cave), 825);
}

#[test]
fn example_2() {
    let mut cave = input(true).unwrap();
    assert_eq!(part_2(&mut cave), 93);
}

#[test]
#[ignore = "Taking too long, needs alternative math"] // todo
fn task_2() {
    let mut cave = input(false).unwrap();
    assert_eq!(part_2(&mut cave), 26729);
}
