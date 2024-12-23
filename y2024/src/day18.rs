use advent_of_code_common::grid::{Coords, Grid};
use advent_of_code_macros::aoc_tests;
use std::cell::Cell;
use tracing::instrument;

type Maze = Grid<CellData>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CellData {
    state: char,
    distance: std::cell::Cell<u16>,
}

impl From<char> for CellData {
    fn from(value: char) -> Self {
        Self {
            state: value,
            distance: Cell::new(u16::MAX),
        }
    }
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            state: '.',
            distance: Cell::new(u16::MAX),
        }
    }
}

fn shortest_distance(input: &str, size: isize, count: usize) -> u16 {
    let mut maze = Maze::new_default(size, size);
    for line in input.lines().take(count) {
        let (x, y) = line.trim().split_once(',').expect("split");
        let x: isize = x.parse().expect("x");
        let y: isize = y.parse().expect("y");
        maze.set(&(x, y), CellData::from('#'));
    }
    let start = (0_isize, 0).as_point();
    maze.get_mut(&start)
        .expect("starting cell")
        .data
        .distance
        .set(0);
    let mut to_visit = vec![start];
    while let Some(p) = to_visit.pop() {
        let current = maze.get(&p).expect("current cell").data.distance.get();
        for neighbor in maze.neighbors_orthogonal(&p).into_iter().flatten() {
            if neighbor.data.state != '.' {
                continue;
            }
            let old_value = neighbor.data.distance.get();
            if old_value > current + 1 {
                neighbor.data.distance.set(current + 1);
                to_visit.push(neighbor.as_point());
            }
        }
    }
    maze.cells
        .last_entry()
        .expect("last cell")
        .get()
        .data
        .distance
        .get()
}

#[instrument(skip(input))]
fn first_block(input: &str, size: isize) -> Option<String> {
    let mut end = input.lines().count();
    let mut next_diff = end / 2;
    loop {
        let blocked_start = shortest_distance(input, size, end - 1) == u16::MAX;
        let blocked_end = shortest_distance(input, size, end) == u16::MAX;
        match (blocked_start, blocked_end) {
            (true, false) => unreachable!("Blocked before {end} but not at {end}"),
            (false, true) => {
                return Some(input.lines().nth(end - 1).expect("line").trim().to_string())
            }
            (true, true) => {
                end -= next_diff;
                next_diff = (next_diff + 1) / 2;
            }
            (false, false) => {
                end += next_diff;
                next_diff = (next_diff + 1) / 2;
            }
        }
    }
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "5,4
                            4,2
                            4,5
                            3,0
                            2,1
                            6,3
                            2,4
                            1,5
                            0,6
                            3,3
                            2,6
                            5,1
                            1,2
                            5,5
                            2,5
                            6,5
                            1,4
                            0,4
                            6,4
                            1,1
                            6,1
                            1,0
                            0,5
                            1,6
                            2,0";

    #[test]
    fn example_1() {
        assert_eq!(shortest_distance(EXAMPLE1, 7, 12), 22);
    }

    #[test]
    fn part_1() {
        assert_eq!(shortest_distance(&read_input(), 71, 1024), 372);
    }

    #[test]
    fn example_2() {
        assert_eq!(first_block(EXAMPLE1, 7).unwrap(), "6,1");
    }

    #[test]
    fn part_2() {
        assert_eq!(first_block(&read_input(), 71).unwrap(), "25,6");
    }
}
