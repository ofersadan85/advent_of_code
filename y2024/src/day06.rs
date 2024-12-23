use advent_of_code_common::grid::{Coords, Direction, Grid, Point};
use advent_of_code_macros::aoc_tests;
use std::collections::HashSet;
use tracing::instrument;

#[instrument(skip(grid), level = "info")]
fn patrol(grid: &Grid, mut direction: Direction) -> Option<HashSet<Point>> {
    let mut visited = HashSet::new();
    let mut turns = HashSet::new();
    let mut current = grid.values().find(|cell| cell.data == '^')?;
    loop {
        visited.insert(current.as_point());
        let mut sight_line = grid.sight_line(current, &direction, &['#']);
        if sight_line.last().unwrap_or(&current).data == '#' {
            sight_line.pop();
            direction = direction.turn_cw_90();
            if !turns.insert((current.as_point(), direction)) {
                return None;
            }
            visited.extend(sight_line.iter().map(Coords::as_point));
            current = sight_line.last().unwrap_or(&current);
        } else {
            visited.extend(sight_line.iter().map(Coords::as_point));
            break;
        }
    }
    Some(visited)
}

#[instrument(skip_all, level = "info")]
fn count_possible_obstacles(grid: &mut Grid) -> usize {
    let visited = patrol(grid, Direction::North);
    visited
        .into_iter()
        .flatten()
        .filter(|p| {
            if grid.get(p).is_some_and(|c| c.data != '.') {
                return false;
            }
            grid.set(p, '#');
            let result = patrol(grid, Direction::North).is_none();
            grid.set(p, '.');
            result
        })
        .count()
}

#[aoc_tests]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn example_1() {
        let grid: Grid = read_to_string("../inputs/2024/day06_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(patrol(&grid, Direction::North).unwrap().len(), 41);
    }

    #[test]
    fn part_1() {
        let grid: Grid = read_input().parse().unwrap();
        assert_eq!(patrol(&grid, Direction::North).unwrap().len(), 5329);
    }

    #[test]
    fn detect_loop() {
        let mut grid: Grid = read_to_string("../inputs/2024/day06_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        let guard = grid.values().find(|cell| cell.data == '^').unwrap();
        grid.set(&(guard.x() - 1, guard.y()), '#');
        assert!(patrol(&grid, Direction::North).is_none());
    }

    #[test]
    fn example_2() {
        let mut grid: Grid = read_to_string("../inputs/2024/day06_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_possible_obstacles(&mut grid), 6);
    }

    #[test]
    fn part_2() {
        let mut grid: Grid = read_input().parse().unwrap();
        assert_eq!(count_possible_obstacles(&mut grid), 2162);
    }
}
