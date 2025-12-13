use advent_of_code_common::algorithms::dijkstra::{dijkstra, Graph};
use advent_of_code_common::file::lines_as_digits;
use advent_of_code_common::v2::{get_neighbors, V2};
use anyhow::Result;
use std::collections::BTreeMap;

const PATH: &str = "../inputs/2021/day15.txt";
const EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_default().insert(v2, c);
    graph.entry(v2).or_default();
}

#[allow(clippy::cast_possible_truncation)] // TODO: Still haven't found a safer way around this
fn make_graph(data: &V2<u32>) -> Graph<Point, u32> {
    let mut graph = BTreeMap::new();
    let (h, w) = (data.len(), data[0].len());
    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let neighbors = get_neighbors(x, y, w, h, false);
            let point = Point {
                x: x as u32,
                y: y as u32,
            };
            for (xi, yi) in neighbors {
                let other_point = Point {
                    x: xi as u32,
                    y: yi as u32,
                };
                add_edge(&mut graph, point, other_point, *value);
            }
        }
    }
    graph
}

/// Helper function to wrap around values
fn multi_wrap(v: u32, count: u32) -> u32 {
    let mut value = v;
    for _ in 0..count {
        if value < 9 {
            value += 1;
        } else {
            value = 1;
        }
    }
    value
}

/// Extend vectors horizontally n times (with value increments)
fn enlarge_horizontal(data: &V2<u32>, n: u32) -> V2<u32> {
    let mut new_data = data.clone();
    for y in 0..data.len() {
        for count in 1..n {
            new_data[y].extend(data[y].iter().map(|&v| multi_wrap(v, count)));
        }
    }
    new_data
}

/// Extend vectors vertically n times (with value increments)
fn enlarge_vertical(data: &V2<u32>, n: u32) -> V2<u32> {
    let mut new_data = data.clone();
    for count in 1..n {
        for row in data {
            new_data.push(row.iter().map(|&v| multi_wrap(v, count)).collect());
        }
    }
    new_data
}

#[allow(clippy::cast_possible_truncation)] // TODO: Still haven't found a safer way around this
fn lowest_risk_path(data: &V2<u32>) -> u32 {
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: data[0].len() as u32 - 1,
        y: data.len() as u32 - 1,
    };
    let graph = make_graph(data);
    dijkstra(&graph, &end).get(&start).unwrap().unwrap().1
}

fn input(example: bool) -> Result<V2<u32>> {
    let input = if example {
        EXAMPLE
    } else {
        &std::fs::read_to_string(PATH)?
    };
    let result = lines_as_digits(input).unwrap();
    Ok(result)
}

fn enlarged_risk_path(data: &V2<u32>) -> u32 {
    let mut data = data.clone();
    data = enlarge_horizontal(&data, 5);
    data = enlarge_vertical(&data, 5);
    lowest_risk_path(&data)
}

#[test]
fn example_1() {
    assert_eq!(lowest_risk_path(&input(true).unwrap()), 40);
}

#[test]
fn example_2() {
    assert_eq!(enlarged_risk_path(&input(true).unwrap()), 315);
}

#[test]
fn task_1() {
    assert_eq!(lowest_risk_path(&input(false).unwrap()), 403);
}

#[test]
#[ignore = "Takes too long"]
fn task_2() {
    assert_eq!(enlarged_risk_path(&input(true).unwrap()), 2840);
}

#[test]
fn test_enlarge_horizontal() {
    let start = vec![vec![2, 8], vec![5, 6]];
    let expected = vec![
        vec![2, 8, 3, 9, 4, 1, 5, 2, 6, 3],
        vec![5, 6, 6, 7, 7, 8, 8, 9, 9, 1],
    ];
    assert_eq!(enlarge_horizontal(&start, 5), expected);
}

#[test]
fn test_enlarge_vertical() {
    let start = vec![vec![1, 8], vec![5, 6]];
    let expected = vec![
        vec![1, 8],
        vec![5, 6],
        vec![2, 9],
        vec![6, 7],
        vec![3, 1],
        vec![7, 8],
        vec![4, 2],
        vec![8, 9],
        vec![5, 3],
        vec![9, 1],
    ];
    assert_eq!(enlarge_vertical(&start, 5), expected);
}
