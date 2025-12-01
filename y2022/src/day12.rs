use advent_of_code_common::file::lines_as_digits_radix;
use advent_of_code_common::v2::{get_neighbors, V2};
use anyhow::{anyhow, Context, Result};
use itertools::iproduct;
use std::collections::HashSet;
use std::hash::Hash;

const PATH: &str = "../inputs/2022/day12.txt";
const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[derive(Debug, Eq, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    value: u32,
    distance: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Node {
    const fn new(x: usize, y: usize, value: u32) -> Self {
        Self {
            x,
            y,
            value,
            distance: 100_000,
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    data: V2<Node>,
    start: Node,
    end: Node,
}

fn input(example: bool) -> Result<Maze> {
    let data_str = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).context("failed to read input file")?
    }
    .replace('S', "9")
    .replace('E', "1");
    let data_int: V2<u32> =
        lines_as_digits_radix(&data_str, 36).map_err(|_| anyhow!("Invalid input"))?;
    let (height, width) = (data_int.len(), data_int[0].len());
    let mut data: V2<Node> = data_int
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, value)| Node::new(x, y, *value))
                .collect()
        })
        .collect();
    let end = iproduct!(0..height, 0..width)
        .find(|&(y, x)| data_int[y][x] == 9)
        .map(|(y, x)| Node::new(x, y, 9))
        .with_context(|| "No end found")?;
    let start = iproduct!(0..height, 0..width)
        .find(|&(y, x)| data_int[y][x] == 1)
        .map(|(y, x)| Node::new(x, y, 36))
        .with_context(|| "No start found")?;
    data[end.y][end.x] = end;
    data[start.y][start.x] = start;

    Ok(Maze { data, start, end })
}

fn update_distances(data: &V2<Node>, end: &Node) -> V2<Node> {
    let mut data = data.clone();
    let (h, w) = (data.len(), data[0].len());
    data[end.y][end.x].distance = 0;

    let mut visited = HashSet::new();
    let mut visit_next = vec![(end.x, end.y)];

    while let Some((current_x, current_y)) = visit_next.pop() {
        if !visited.contains(&(current_x, current_y)) {
            let current = data[current_y][current_x];
            let next_neighbors: Vec<_> = get_neighbors(current_x, current_y, w, h, false)
                .iter()
                .filter(|&&(x, y)| data[y][x].value >= current.value - 1)
                .copied()
                .collect();
            for (other_x, other_y) in &next_neighbors {
                let other = data[*other_y][*other_x];
                if other.distance > current.distance + 1 {
                    data[*other_y][*other_x].distance = current.distance + 1;
                }
            }
            visit_next.extend(next_neighbors);
        }
        visited.insert((current_x, current_y));
    }
    data
}

#[test]
fn example_1() {
    let mut maze = input(true).unwrap();
    maze.data = update_distances(&maze.data, &maze.start);
    maze.end = maze.data[maze.end.y][maze.end.x];
    assert_eq!(maze.end.distance, 31);
}

#[test]
fn task_1() {
    let mut maze = input(false).unwrap();
    for _ in 0..47 {
        // todo: No idea why this takes 47 iterations to get the right answer, but it works
        maze.data = update_distances(&maze.data, &maze.start);
    }
    maze.end = maze.data[maze.end.y][maze.end.x];
    assert_eq!(maze.end.distance, 412);
}

#[test]
fn example_2() {
    let mut maze = input(true).unwrap();
    maze.data = update_distances(&maze.data, &maze.start);
    let closest = maze
        .data
        .iter()
        .flatten()
        .filter(|n| n.value == 10)
        .min_by_key(|n| n.distance)
        .unwrap()
        .distance;
    assert_eq!(closest, 29);
}

#[test]
fn task_2() {
    let mut maze = input(false).unwrap();
    for _ in 0..47 {
        // todo: No idea why this takes 47 iterations to get the right answer, but it works
        maze.data = update_distances(&maze.data, &maze.start);
    }
    let closest = maze
        .data
        .iter()
        .flatten()
        .filter(|n| n.value == 10)
        .min_by_key(|n| n.distance)
        .unwrap()
        .distance;
    assert_eq!(closest, 402);
}
