use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_points(&self) -> Vec<Point> {
        let mut result = vec![];
        if self.start.x == self.end.x {
            let v = vec![self.start.y, self.end.y];
            let (min, max) = v.iter().minmax().into_option().unwrap();
            let points = (min.to_owned()..=max.to_owned()).map(|i| Point {
                x: self.start.x,
                y: i,
            });
            result.extend(points);
        }
        if self.start.y == self.end.y {
            let v = vec![self.start.x, self.end.x];
            let (min, max) = v.iter().minmax().into_option().unwrap();
            let points = (min.to_owned()..=max.to_owned()).map(|i| Point {
                x: i,
                y: self.start.x,
            });
            result.extend(points);
        }
        result
    }
}

fn point_count(data: Vec<Line>) -> HashMap<Point, usize> {
    let points: Vec<Point> = data.iter().flat_map(|line| line.get_points()).collect();
    let mut counter = HashMap::new();
    for p in points {
        counter.entry(p).and_modify(|v| *v += 1).or_insert(1);
    }
    counter
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::common::*;
    const PATH: &str = "inputs/aoc_2021_5.txt";
    const EXAMPLE: &str = "
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    const EXAMPLE_OUTPUT: &str = "
    .......1..
    ..1....1..
    ..1....1..
    .......1..
    .112111211
    ..........
    ..........
    ..........
    ..........
    222111....";

    fn setup_data(data: Vec<String>) -> Vec<Line> {
        data.iter()
            .map(|line| {
                let (a, b, c, d): (usize, usize, usize, usize) = line
                    .replace(" -> ", ",")
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Line {
                    start: Point { x: a, y: b },
                    end: Point { x: c, y: d },
                }
            })
            .collect()
    }

    fn setup_output(points: HashMap<Point, usize>, w: usize, h: usize) -> String {
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| points.get(&Point { x, y }).unwrap_or(&0).to_owned())
                    .join("")
            })
            .rev()
            .join("\n")
            .replace('0', ".")
    }

    #[test]
    fn example_1() {
        let lines = split_lines(EXAMPLE);
        let data = setup_data(lines.clone());
        let counter = point_count(data);
        println!("{:?}", counter);
        println!("{:?}", counter.values().sum::<usize>());
        let result = setup_output(counter, lines[0].len(), lines.len());
        assert_eq!(result, split_lines(EXAMPLE_OUTPUT).join("\n"));
    }

    // #[test]
    // fn example_2() {
    //     let data = setup_data(split_lines(EXAMPLE));
    //     let result = do_something(data);
    //     assert_eq!(result, EXAMPLE_OUTPUT);
    // }

    // #[test]
    // fn task_1() {
    //     let data = setup_data(get_data(PATH).unwrap());
    //     let result = do_something(data);
    //     assert_eq!(result, EXAMPLE_OUTPUT);
    // }

    // #[test]
    // fn task_2() {
    //     let data = setup_data(get_data(PATH).unwrap());
    //     let result = do_something(data);
    //     assert_eq!(result, EXAMPLE_OUTPUT);
    // }
}
