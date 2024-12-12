use advent_of_code_common::grid::Grid;
use tracing::instrument;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn dx_dy(self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[instrument(skip(grid), level = "info")]
fn patrol(grid: &Grid<char>, mut direction: Direction) -> Option<HashSet<(isize, isize)>> {
    let mut visited = HashSet::new();
    let mut turns = HashSet::new();
    let mut current = grid.cells.iter().find(|cell| cell.state == '^')?;
    loop {
        visited.insert((current.x, current.y));
        let (dx, dy) = direction.dx_dy();
        let mut sight_line = grid.sight_line_cells(current.x, current.y, dx, dy, &['#']);
        if sight_line.last().unwrap_or(&current).state == '#' {
            sight_line.pop();
            direction = direction.turn_right();
            if !turns.insert((current.x, current.y, direction)) {
                return None;
            }
            visited.extend(sight_line.iter().map(|cell| (cell.x, cell.y)));
            current = sight_line.last().unwrap_or(&current);
        } else {
            visited.extend(sight_line.iter().map(|cell| (cell.x, cell.y)));
            break;
        }
    }
    Some(visited)
}

#[instrument(skip_all, level = "info")]
fn count_possible_obstacles(grid: &mut Grid<char>) -> usize {
    let visited = patrol(grid, Direction::Up);
    visited
        .iter()
        .flatten()
        .filter(|cell| {
            if grid.get(cell.0, cell.1) != Some('.') {
                return false;
            }
            grid.set(cell.0, cell.1, '#');
            let result = patrol(grid, Direction::Up).is_none();
            grid.set(cell.0, cell.1, '.');
            result
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use test_log::test;

    #[test]
    fn example_1() {
        let grid: Grid<char> = read_to_string("../inputs/2024/day06_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(patrol(&grid, Direction::Up).unwrap().len(), 41);
    }

    #[test]
    fn part_1() {
        let grid: Grid<char> = read_to_string("../inputs/2024/day06.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(patrol(&grid, Direction::Up).unwrap().len(), 5329);
    }

    #[test]
    fn detect_loop() {
        let mut grid: Grid<char> = read_to_string("../inputs/2024/day06_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        let guard = grid.cells.iter().find(|cell| cell.state == '^').unwrap();
        grid.set(guard.x - 1, guard.y, '#');
        assert!(patrol(&grid, Direction::Up).is_none());
    }

    #[test]
    fn example_2() {
        let mut grid: Grid<char> = read_to_string("../inputs/2024/day06_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_possible_obstacles(&mut grid), 6);
    }

    #[test]
    fn part_2() {
        let mut grid: Grid<char> = read_to_string("../inputs/2024/day06.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_possible_obstacles(&mut grid), 2162);
    }
}
