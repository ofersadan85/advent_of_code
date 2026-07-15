use advent_of_code_common::grid::{Coords, Grid, Point};
use advent_of_code_macros::aoc_solver;
use std::collections::{BTreeMap, BTreeSet};

const PRECISION: f64 = 100_000.0;

#[expect(clippy::cast_possible_truncation)]
fn parse_input(input: &str) -> BTreeMap<Point, BTreeSet<(i64, Point)>> {
    let grid: Grid = input.parse().expect("Failed to parse input");
    let mut angles = BTreeMap::new();
    for pos in grid.iter().filter(|(_, v)| v.data == '#').map(|(p, _)| p) {
        for other in grid.iter().filter(|(p, v)| v.data == '#' && p != &pos) {
            let angle = pos.angle_to(other.0);
            // Normalize angles to start at the top and go clockwise, with 0 degrees at the top.
            let angle =
                (angle + std::f64::consts::FRAC_PI_2).rem_euclid(2.0 * std::f64::consts::PI);
            let int_angle = (angle * PRECISION).round() as i64;
            angles
                .entry(*pos)
                .or_insert_with(BTreeSet::new)
                .insert((int_angle, *other.0));
        }
    }
    angles
}

fn find_best(angles: &BTreeMap<Point, BTreeSet<(i64, Point)>>) -> (Point, usize) {
    let angles: BTreeMap<Point, BTreeSet<i64>> = angles
        .iter()
        .map(|(p, s)| (*p, s.iter().map(|(angle, _)| *angle).collect()))
        .collect();
    angles
        .iter()
        .map(|(p, s)| (*p, s.len()))
        .max_by_key(|(_, s)| *s)
        .unwrap_or_default()
}

#[aoc_solver(file = "inputs/2019/day10.txt", expected = 260)]
fn part_1(input: &str) -> usize {
    let angles = parse_input(input);
    find_best(&angles).1
}

#[aoc_solver(file = "inputs/2019/day10.txt", expected = 608)]
fn part_2(input: &str) -> isize {
    let angles = parse_input(input);
    let best = find_best(&angles).0;
    let angles = angles.get(&best).expect("Best position not found");
    let mut vaporized = Vec::new();
    let mut visited_angles = BTreeSet::new();
    let mut visited_asteroids = BTreeSet::new();
    while vaporized.len() < 200 {
        visited_angles.clear();
        for (int_angle, point) in angles {
            if visited_asteroids.contains(point) {
                continue;
            }
            if visited_angles.insert(int_angle) {
                vaporized.push(point);
                visited_asteroids.insert(point);
            }
        }
    }
    let p = vaporized[199];
    p.x * 100 + p.y
}
