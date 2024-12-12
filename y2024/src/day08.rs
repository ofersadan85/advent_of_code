use advent_of_code_common::grid::{Grid, PositionedCell};
use itertools::Itertools;
use tracing::instrument;
use std::collections::HashSet;

type Cell = PositionedCell<char, bool>;

fn tag_anti_nodes(grid: &mut Grid<char, bool>, p1: &Cell, p2: &Cell, ignore_distance: bool) {
    let range = if ignore_distance {
        0..=isize::MAX
    } else {
        1..=1
    };
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    for distance in range.clone() {
        let x = p1.x + distance * x_diff;
        let y = p1.y + distance * y_diff;
        if let Some(cell) = grid.get_cell_mut(x, y) {
            cell.data = true;
        } else {
            break;
        }
    }
    for distance in range {
        let x = p2.x - distance * x_diff;
        let y = p2.y - distance * y_diff;
        if let Some(cell) = grid.get_cell_mut(x, y) {
            cell.data = true;
        } else {
            break;
        }
    }
}

#[instrument(skip(grid), level = "info")]
fn count_unique_anti_nodes(mut grid: Grid<char, bool>, ignore_distance: bool) -> usize {
    let states: HashSet<char> = grid
        .cells
        .iter()
        .filter(|cell| cell.state != '.')
        .map(|cell| cell.state)
        .collect();
    for state in states {
        let combos: Vec<(Cell, Cell)> = grid
            .cells
            .iter()
            .filter(|cell| cell.state == state)
            .cloned()
            .tuple_combinations()
            .collect();
        for (p1, p2) in &combos {
            tag_anti_nodes(&mut grid, p1, p2, ignore_distance);
        }
    }
    grid.cells.iter().filter(|cell| cell.data).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use test_log::test;

    const EXAMPLE: &str = "............
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
        let grid = EXAMPLE.parse().unwrap();
        assert_eq!(count_unique_anti_nodes(grid, false), 14);
    }

    #[test]
    fn part_1() {
        let grid = read_to_string("../inputs/2024/day08.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_unique_anti_nodes(grid, false), 344);
    }

    #[test]
    fn example_2() {
        let grid = EXAMPLE.parse().unwrap();
        assert_eq!(count_unique_anti_nodes(grid, true), 34);
    }

    #[test]
    fn part_2() {
        let grid = read_to_string("../inputs/2024/day08.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_unique_anti_nodes(grid, true), 1182);
    }
}
