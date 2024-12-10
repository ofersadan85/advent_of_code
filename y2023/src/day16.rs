use advent_of_code_common::grid::{Direction, DxDy, Grid};
use std::collections::HashSet;
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mirror {
    Slash,
    Backslash,
    SplitterHorizontal,
    SplitterVertical,
    Empty,
}

impl TryFrom<char> for Mirror {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '/' => Ok(Self::Slash),
            '\\' => Ok(Self::Backslash),
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            '.' => Ok(Self::Empty),
            _ => Err("Invalid mirror type"),
        }
    }
}

impl std::fmt::Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slash => write!(f, "/"),
            Self::Backslash => write!(f, "\\"),
            Self::SplitterHorizontal => write!(f, "-"),
            Self::SplitterVertical => write!(f, "|"),
            Self::Empty => write!(f, "."),
        }
    }
}

impl Mirror {
    fn reflect(self, dir: Direction) -> (Direction, Option<Direction>) {
        use Direction::{East, North, South, West};
        use Mirror::{Backslash, Empty, Slash, SplitterHorizontal, SplitterVertical};
        match (self, dir) {
            (Slash, North) | (Backslash, South) => (East, None),
            (Slash, South) | (Backslash, North) => (West, None),
            (Slash, West) | (Backslash, East) => (South, None),
            (Slash, East) | (Backslash, West) => (North, None),
            (SplitterHorizontal, North | South) => (West, Some(East)),
            (SplitterHorizontal, West | East) | (SplitterVertical, North | South) | (Empty, _) => {
                (dir, None)
            }
            (SplitterVertical, West | East) => (North, Some(South)),
            _ => panic!("Invalid mirror and direction combination {self:?}, {dir:?}"),
        }
    }
}

#[instrument(skip_all, level = "info")]
fn max_energize(grid: Grid<Mirror, HashSet<Direction>>) -> usize {
    let mut entry_options = HashSet::new();
    grid.x_range.clone().for_each(|x| {
        entry_options.insert(((x, 0), Direction::South));
        entry_options.insert(((x, grid.y_range.end - 1), Direction::North));
    });
    grid.y_range.clone().for_each(|y| {
        entry_options.insert(((0, y), Direction::East));
        entry_options.insert(((grid.x_range.end - 1, y), Direction::West));
    });
    entry_options
        .iter()
        .map(|((x, y), dir)| {
            let mut grid = grid.clone();
            energize(&mut grid, *x, *y, *dir)
        })
        .max()
        .unwrap_or(0)
}

#[instrument(skip_all, level = "info")]
pub fn energize(
    grid: &mut Grid<Mirror, HashSet<Direction>>,
    x: isize,
    y: isize,
    dir: Direction,
) -> usize {
    let mut to_visit = HashSet::new();
    to_visit.insert(((x, y), dir));
    while let Some(((x, y), direction)) = to_visit.iter().next().copied() {
        to_visit.remove(&((x, y), direction));
        if let Some(cell) = grid.get_cell_mut(x, y) {
            if cell.data.insert(direction) {
                let (new_dir1, new_dir2) = cell.state.reflect(direction);
                to_visit.insert(((x + new_dir1.dx(), y + new_dir1.dy()), new_dir1));
                if let Some(new_dir2) = new_dir2 {
                    to_visit.insert(((x + new_dir2.dx(), y + new_dir2.dy()), new_dir2));
                }
            }
        }
    }
    grid.cells
        .iter()
        .filter(|cell| !cell.data.is_empty())
        .count()
}

fn print_grid(grid: &Grid<Mirror, HashSet<Direction>>) {
    for y in grid.y_range.clone() {
        for x in grid.x_range.clone() {
            if let Some(cell) = grid.get_cell(x, y) {
                print!("{}", if cell.data.is_empty() { '.' } else { '#' });
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use advent_of_code_common::init_tracing;
    use std::fs::read_to_string;
    const EXAMPLE: &str = r#".|...\....
                             |.-.\.....
                             .....|-...
                             ........|.
                             ..........
                             .........\
                             ..../.\\..
                             .-.-/..|..
                             .|....-|.\
                             ..//.|...."#;

    #[test]
    fn example_1() {
        let mut grid: Grid<Mirror, HashSet<Direction>> = EXAMPLE.parse().unwrap();
        assert_eq!(energize(&mut grid, 0, 0, Direction::East), 46);
    }

    #[test]
    fn part_1() {
        let mut grid: Grid<Mirror, HashSet<Direction>> = read_to_string("../inputs/2023/day16.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(energize(&mut grid, 0, 0, Direction::East), 7067);
    }

    #[test]
    fn example_2() {
        let grid: Grid<Mirror, HashSet<Direction>> = EXAMPLE.parse().unwrap();
        assert_eq!(max_energize(grid), 51);
    }

    #[test]
    fn part_2() {
        // init_tracing();
        let grid: Grid<Mirror, HashSet<Direction>> = read_to_string("../inputs/2023/day16.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(max_energize(grid), 7324);
    }
}
