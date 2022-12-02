use advent_of_code_common::algorithms::dijkstra::{dijkstra, Graph};
use advent_of_code_common::v2::{get_neighbors, V2};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    graph.entry(v2).or_insert_with(BTreeMap::new);
}

// BTreeMap<Point, BTreeMap<Point, usize>>
fn make_graph(data: &V2<usize>) -> Graph<Point, usize> {
    let mut graph = BTreeMap::new();
    let (h, w) = (data.len(), data[0].len());
    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let neighbors = get_neighbors(x, y, w, h, false);
            let point = Point { x, y };
            for (xi, yi) in neighbors {
                let other_point = Point { x: xi, y: yi };
                add_edge(&mut graph, point, other_point, *value);
            }
        }
    }
    graph
}

fn lowest_risk_path(data: &V2<usize>) -> usize {
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: data[0].len() - 1,
        y: data.len() - 1,
    };
    let graph = make_graph(data);
    dijkstra(&graph, &end).get(&start).unwrap().unwrap().1
}

fn multi_wrap(v: usize, count: usize) -> usize {
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

fn enlarge_horizontal(data: &V2<usize>, n: usize) -> V2<usize> {
    let mut new_data = data.clone();
    for y in 0..data.len() {
        for count in 1..n {
            new_data[y].extend(data[y].iter().map(|&v| multi_wrap(v, count)));
        }
    }
    new_data
}

fn enlarge_vertical(data: &V2<usize>, n: usize) -> V2<usize> {
    let mut new_data = data.clone();
    for count in 1..n {
        for row in data {
            new_data.push(row.iter().map(|&v| multi_wrap(v, count)).collect());
        }
    }
    new_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::file::{get_data, parse_digit_lines};
    const PATH: &str = "inputs/day15.txt";
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

    fn setup_data() -> V2<usize> {
        let data = get_data(PATH).unwrap().join("\n");
        parse_digit_lines(&data, 10)
    }

    #[test]
    fn example_1() {
        let data = parse_digit_lines(EXAMPLE, 10);
        let result: usize = lowest_risk_path(&data);
        assert_eq!(result, 40);
    }

    #[test]
    fn example_2() {
        let mut data = parse_digit_lines(EXAMPLE, 10);
        data = enlarge_horizontal(&data, 5);
        data = enlarge_vertical(&data, 5);
        let result: usize = lowest_risk_path(&data);
        assert_eq!(result, 315);
    }

    #[test]
    fn task_1() {
        let data = setup_data();
        let result: usize = lowest_risk_path(&data);
        assert_eq!(result, 403);
    }

    #[test]
    #[ignore = "Takes too long"]
    fn task_2() {
        let mut data = setup_data();
        data = enlarge_horizontal(&data, 5);
        data = enlarge_vertical(&data, 5);
        let result: usize = lowest_risk_path(&data);
        assert_eq!(result, 2840);
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
}
