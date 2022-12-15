use itertools::iproduct;
use regex::Regex;
use std::collections::HashSet;

const PATH: &str = "inputs/day15.txt";
const EXAMPLE: &str = "inputs/day15_example.txt";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sensor {
    x: i32,
    y: i32,
    distance: i32,
}

impl Sensor {
    fn from_row(row: &str) -> Self {
        let re = Regex::new(r"(-?\d+)").unwrap();
        let caps: Vec<i32> = re
            .captures_iter(row)
            .map(|cap| cap[1].parse().unwrap())
            .collect();
        Self {
            x: caps[0],
            y: caps[1],
            distance: (caps[0] - caps[2]).abs() + (caps[1] - caps[3]).abs(),
        }
    }

    fn points_in_row(&self, y: i32, min_x: i32, max_x: i32) -> HashSet<(i32, i32)> {
        if (self.y - y).abs() > self.distance {
            return HashSet::new();
        }
        let distance = self.distance;
        let y_diff = (self.y - y).abs();
        let x_diff = distance - y_diff;
        (0..=x_diff)
            .flat_map(|x| [(self.x + x, y), (self.x - x, y)])
            .filter(|(x, _)| (min_x..=max_x).contains(x))
            .collect()
    }

    fn can_detect(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.distance
    }
}

fn input(example: bool) -> Vec<Sensor> {
    let path = if example { EXAMPLE } else { PATH };
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(Sensor::from_row)
        .collect()
}

fn part_1(sensors: &[Sensor], y: i32) -> i32 {
    let points: HashSet<_> = sensors
        .iter()
        .flat_map(|s| s.points_in_row(y, i32::MIN, i32::MAX))
        .collect();
    points
        .iter()
        .filter(|(_, yi)| *yi == y)
        .count()
        .try_into()
        .unwrap()
}

fn part_2(sensors: &[Sensor], max_distance: i32) -> i32 {
    // todo: needs to be optimized to not iterate over all points
    let (x, y) = iproduct!(0..=max_distance, 0..=max_distance)
        .filter(|(x, y)| sensors.iter().all(|s| !s.can_detect(*x, *y)))
        .next()
        .unwrap();
    x * 4_000_000 + y
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true), 10) - 1, 26);
}

#[test]
#[ignore = "Takes too long"]
fn task_1() {
    assert_eq!(part_1(&input(false), 2_000_000) - 1, 5_716_881);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true), 20), 56_000_011);
}

#[test]
#[ignore = "Takes too long"]
fn task_2() {
    assert_eq!(part_2(&input(false), 4_000_000), 0);
}
