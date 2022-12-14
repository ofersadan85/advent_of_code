use itertools::Itertools;
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
    fn neighbors(&self) -> HashSet<Cube> {
        let (x, y, z) = (self.x, self.y, self.z);
        [
            Cube { x, y, z: z + 1 },
            Cube { x, y: y + 1, z },
            Cube { x: x + 1, y, z },
            Cube { x: x - 1, y, z },
            Cube { x, y: y - 1, z },
            Cube { x, y, z: z - 1 },
        ]
        .iter()
        .copied()
        .collect()
    }
}

fn input(example: bool) -> HashSet<Cube> {
    if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .lines()
    .map(|row| {
        row.split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
    })
    .map(|v| Cube {
        x: v[0],
        y: v[1],
        z: v[2],
    })
    .collect()
}

fn part_1(cubes: &HashSet<Cube>) -> i32 {
    cubes
        .iter()
        .map(|c| 6 - c.neighbors().intersection(cubes).count())
        .sum::<usize>()
        .try_into()
        .unwrap()
}

fn part_2(cubes: &HashSet<Cube>) -> i32 {
    let (min_x, max_x) = cubes.iter().map(|c| c.x).minmax().into_option().unwrap();
    let (min_y, max_y) = cubes.iter().map(|c| c.y).minmax().into_option().unwrap();
    let (min_z, max_z) = cubes.iter().map(|c| c.z).minmax().into_option().unwrap();
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
    while !unvisited.is_empty() {
        let cube = unvisited.pop().unwrap();
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
    assert_eq!(part_1(&input(true)), 64);
}

#[test]
fn task_1() {
    assert_eq!(part_1(&input(false)), 4288);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 58);
}

#[test]
fn task_2() {
    assert_eq!(part_2(&input(false)), 2494);
}
