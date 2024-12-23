use advent_of_code_common::grid::{Coords, Grid, Point};
use advent_of_code_macros::aoc_tests;
use itertools::Itertools;
use std::collections::HashSet;
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellData {
    state: char,
    is_anti: bool,
}

impl From<char> for CellData {
    fn from(value: char) -> Self {
        Self {
            state: value,
            is_anti: false,
        }
    }
}

fn tag_anti_nodes(grid: &mut Grid<CellData>, p1: &Point, p2: &Point, ignore_distance: bool) {
    let range = if ignore_distance {
        0..=isize::MAX
    } else {
        1..=1
    };
    let diff = p1 - *p2;
    let mut changed = false;
    for distance in range.clone() {
        let p3 = p1 + diff * distance;
        if let Some(cell) = grid.get_mut(&p3) {
            cell.data.is_anti = true;
            changed = true;
        }
        let p4 = p2 - diff * distance;
        if let Some(cell) = grid.get_mut(&p4) {
            cell.data.is_anti = true;
            changed = true;
        }
        if !changed {
            break;
        }
    }
}

#[instrument(skip(grid), level = "info")]
fn count_unique_anti_nodes(mut grid: Grid<CellData>, ignore_distance: bool) -> usize {
    let states: HashSet<char> = grid
        .values()
        .map(|cell| cell.data.state)
        .filter(|cell| cell != &'.')
        .collect();
    for state in states {
        let combos: Vec<(Point, Point)> = grid
            .values()
            .filter(|cell| cell.data.state == state)
            .map(|cell| cell.as_point())
            .tuple_combinations()
            .collect();
        for (p1, p2) in &combos {
            tag_anti_nodes(&mut grid, p1, p2, ignore_distance);
        }
    }
    grid.values().filter(|cell| cell.data.is_anti).count()
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "............
                            ........0...
                            .....0......
                            .......0....
                            ....0.......
                            ......A.....
                            ............
                            ............
                            ........A...
                            .........A..
                            ............
                            ............";

    #[test]
    fn example_1() {
        let grid = EXAMPLE1.parse().unwrap();
        assert_eq!(count_unique_anti_nodes(grid, false), 14);
    }

    #[test]
    fn part_1() {
        let grid = read_input().parse().unwrap();
        assert_eq!(count_unique_anti_nodes(grid, false), 344);
    }

    #[test]
    fn example_2() {
        let grid = EXAMPLE1.parse().unwrap();
        assert_eq!(count_unique_anti_nodes(grid, true), 34);
    }

    #[test]
    fn part_2() {
        let grid = read_input().parse().unwrap();
        assert_eq!(count_unique_anti_nodes(grid, true), 1182);
    }
}
