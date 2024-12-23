use advent_of_code_common::grid::{Coords, Grid};
use advent_of_code_macros::aoc_tests;
use std::collections::HashSet;
use tracing::instrument;

#[instrument(skip(grid), level = "info")]
fn count_paths(grid: &Grid, unique: bool) -> usize {
    let mut sum = 0;
    let trail_heads: Vec<_> = grid.values().filter(|&x| x.data == '0').collect();
    for head in trail_heads {
        let mut paths = vec![head];
        for step in '1'..='9' {
            paths = paths
                .iter()
                .flat_map(|c| grid.neighbors_orthogonal(&c.as_point()))
                .filter_map(|c| {
                    if let Some(cell) = c {
                        if cell.data == step {
                            return c;
                        }
                    }
                    None
                })
                .collect();
        }
        if unique {
            sum += paths
                .iter()
                .map(Coords::as_point)
                .collect::<HashSet<_>>()
                .len();
        } else {
            sum += paths.len();
        }
    }
    sum
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "89010123
                           78121874
                           87430965
                           96549874
                           45678903
                           32019012
                           01329801
                           10456732";

    #[test]
    fn test_count_paths_unique() {
        let grid: Grid = EXAMPLE.parse().unwrap();
        assert_eq!(count_paths(&grid, true), 36);
    }

    #[test]
    fn part_1() {
        let grid: Grid = read_input().parse().unwrap();
        assert_eq!(count_paths(&grid, true), 593);
    }

    #[test]
    fn test_count_paths() {
        let grid: Grid = EXAMPLE.parse().unwrap();
        assert_eq!(count_paths(&grid, false), 81);
    }

    #[test]
    fn part_2() {
        let grid: Grid = read_input().parse().unwrap();
        assert_eq!(count_paths(&grid, false), 1192);
    }
}
