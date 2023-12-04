use anyhow::{Context, Result};
use std::collections::HashSet;

pub const PATH: &str = "inputs/day18.txt";
pub const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn neighbors(&self) -> HashSet<Self> {
        let (x, y, z) = (self.x, self.y, self.z);
        [
            Self { x, y, z: z + 1 },
            Self { x, y: y + 1, z },
            Self { x: x + 1, y, z },
            Self { x: x - 1, y, z },
            Self { x, y: y - 1, z },
            Self { x, y, z: z - 1 },
        ]
        .iter()
        .copied()
        .collect()
    }
}

fn input(example: bool) -> Result<HashSet<Cube>> {
    let set = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).context("failed to read input file")?
    }
    .lines()
    .map(|row| {
        row.split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i32>>()
    })
    .map(|v| Cube {
        x: v[0],
        y: v[1],
        z: v[2],
    })
    .collect();
    Ok(set)
}

fn part_1(cubes: &HashSet<Cube>) -> Result<i32> {
    cubes
        .iter()
        .map(|c| 6 - c.neighbors().intersection(cubes).count())
        .sum::<usize>()
        .try_into()
        .context("Could not convert to i32")
}

fn part_2(cubes: &HashSet<Cube>) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;
    for c in cubes {
        min_x = min_x.min(c.x);
        max_x = max_x.max(c.x);
        min_y = min_y.min(c.y);
        max_y = max_y.max(c.y);
        min_z = min_z.min(c.z);
        max_z = max_z.max(c.z);
    }
    let x_range = min_x - 1..max_x + 2;
    let y_range = min_y - 1..max_y + 2;
    let z_range = min_z - 1..max_z + 2;
    let first_water = Cube {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    };
    let mut unvisited = vec![first_water];
    let mut water = HashSet::new();
    let mut result = 0;
    while let Some(cube) = unvisited.pop() {
        if !water.contains(&cube) {
            water.insert(cube);
            let neighbors = cube.neighbors();
            for n in neighbors {
                if cubes.contains(&n) {
                    result += 1;
                } else if !water.contains(&n)
                    && x_range.contains(&n.x)
                    && y_range.contains(&n.y)
                    && z_range.contains(&n.z)
                {
                    unvisited.push(n);
                }
            }
        }
    }
    result
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap()).unwrap(), 64);
}

#[test]
fn task_1() {
    assert_eq!(part_1(&input(false).unwrap()).unwrap(), 4288);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()), 58);
}

#[test]
fn task_2() {
    assert_eq!(part_2(&input(false).unwrap()), 2494);
}
