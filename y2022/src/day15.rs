use anyhow::{Context, Result};
use itertools::iproduct;
use regex::Regex;
use std::collections::HashSet;

const PATH: &str = "inputs/day15.txt";
const EXAMPLE: &str = "inputs/day15_example.txt";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
}

impl TryFrom<&str> for Sensor {
    type Error = &'static str;

    fn try_from(row: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(-?\d+)").map_err(|_| "invalid regex")?;
        let caps: Vec<i64> = re
            .captures_iter(row)
            .filter_map(|cap| cap.get(1)?.as_str().parse().ok())
            .collect();
        if caps.len() == 4 {
            Ok(Self {
                x: caps[0],
                y: caps[1],
                distance: (caps[0] - caps[2]).abs() + (caps[1] - caps[3]).abs(),
            })
        } else {
            Err("invalid row")
        }
    }
}

impl Sensor {
    fn points_in_row(&self, y: i64, min_x: i64, max_x: i64) -> HashSet<(i64, i64)> {
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

    const fn can_detect(&self, x: i64, y: i64) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.distance
    }
}

fn input(example: bool) -> Result<Vec<Sensor>> {
    let path = if example { EXAMPLE } else { PATH };
    let sensors = std::fs::read_to_string(path)
        .context("Error reading input file")?
        .lines()
        .filter_map(|row| Sensor::try_from(row).ok())
        .collect();
    Ok(sensors)
}

fn part_1(sensors: &[Sensor], y: i64) -> Result<i64> {
    let points: HashSet<_> = sensors
        .iter()
        .flat_map(|s| s.points_in_row(y, i64::MIN, i64::MAX))
        .collect();
    let result = points.iter().filter(|(_, yi)| *yi == y).count().try_into();
    Ok(result?)
}

fn part_2(sensors: &[Sensor]) -> Result<i64> {
    let n2 = sensors.len() * 2;
    let positive_lines: Vec<_> = sensors
        .iter()
        .flat_map(|s| [s.x - s.y - s.distance, s.x - s.y + s.distance])
        .collect();
    let negative_lines: Vec<_> = sensors
        .iter()
        .flat_map(|s| [s.x + s.y - s.distance, s.x + s.y + s.distance])
        .collect();
    let p_options: Vec<_> = iproduct!(0..n2, 0..n2)
        .filter(|(i, j)| i < j)
        .filter(|&(i, j)| positive_lines[i].abs_diff(positive_lines[j]) == 2)
        .map(|(i, j)| positive_lines[i].min(positive_lines[j]) + 1)
        .collect();
    let n_options: Vec<_> = iproduct!(0..n2, 0..n2)
        .filter(|(i, j)| i < j)
        .filter(|&(i, j)| negative_lines[i].abs_diff(negative_lines[j]) == 2)
        .map(|(i, j)| negative_lines[i].min(negative_lines[j]) + 1)
        .collect();
    let (x, y) = iproduct!(p_options, n_options)
        .map(|(p, n)| ((p + n) / 2, (p - n).abs() / 2))
        .find(|(x, y)| sensors.iter().all(|s| !s.can_detect(*x, *y)))
        .ok_or_else(|| anyhow::anyhow!("No solution found"))?;

    Ok(x * 4_000_000 + y)
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap(), 10).unwrap() - 1, 26);
}

#[test]
#[ignore = "Takes too long"]
fn task_1() {
    assert_eq!(
        part_1(&input(false).unwrap(), 2_000_000).unwrap() - 1,
        5_716_881
    );
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()).unwrap(), 56_000_011);
}

#[test]
fn task_2() {
    assert_eq!(part_2(&input(false).unwrap()).unwrap(), 10_852_583_132_904);
}
