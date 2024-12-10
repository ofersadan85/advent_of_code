use std::collections::HashSet;

use advent_of_code_common::grid::{Grid, PositionedCell};

fn count_paths(grid: &Grid<char>, unique: bool) -> usize {
    let mut sum = 0;
    let trail_heads: Vec<&PositionedCell<char>> =
        grid.cells.iter().filter(|&x| x.state == '0').collect();
    for head in trail_heads {
        let mut paths = vec![head];
        for step in '1'..='9' {
            paths = paths
                .iter()
                .flat_map(|c| grid.neighbors_orthogonal_cells(c.x, c.y))
                .filter_map(|c| {
                    if let Some(cell) = c {
                        if cell.state == step {
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
                .map(|c| (c.x, c.y))
                .collect::<HashSet<_>>()
                .len();
        } else {
            sum += paths.len();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
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
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        assert_eq!(count_paths(&grid, true), 36);
    }

    #[test]
    fn part_1() {
        let grid: Grid<char> = read_to_string("../inputs/2024/day10.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_paths(&grid, true), 593);
    }

    #[test]
    fn test_count_paths() {
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        assert_eq!(count_paths(&grid, false), 81);
    }

    #[test]
    fn part_2() {
        let grid: Grid<char> = read_to_string("../inputs/2024/day10.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_paths(&grid, false), 1192);
    }
}
