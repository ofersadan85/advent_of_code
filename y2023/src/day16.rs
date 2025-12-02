use advent_of_code_common::grid::{Coords, Direction, Grid, Point};
use advent_of_code_macros::aoc_tests;
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

impl TryFrom<char> for CellData {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let mirror = match value {
            '/' => Ok(Mirror::Slash),
            '\\' => Ok(Mirror::Backslash),
            '|' => Ok(Mirror::SplitterVertical),
            '-' => Ok(Mirror::SplitterHorizontal),
            '.' => Ok(Mirror::Empty),
            _ => Err("Invalid mirror type"),
        };
        Ok(Self {
            mirror: mirror?,
            directions: HashSet::new(),
        })
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

#[derive(Debug, Clone)]
struct CellData {
    mirror: Mirror,
    directions: HashSet<Direction>,
}

#[instrument(skip_all, level = "info")]
fn max_energize(grid: &Grid<CellData>) -> usize {
    let mut entry_options = HashSet::new();
    grid.x_range.clone().for_each(|x| {
        entry_options.insert(((x, 0).as_point(), Direction::South));
        entry_options.insert(((x, grid.y_range.end - 1).as_point(), Direction::North));
    });
    grid.y_range.clone().for_each(|y| {
        entry_options.insert(((0, y).as_point(), Direction::East));
        entry_options.insert(((grid.x_range.end - 1, y).as_point(), Direction::West));
    });
    entry_options
        .iter()
        .map(|(point, dir)| {
            let mut grid = grid.clone();
            energize(&mut grid, point, *dir)
        })
        .max()
        .unwrap_or(0)
}

#[instrument(skip_all, level = "info")]
fn energize(grid: &mut Grid<CellData>, point: &Point, dir: Direction) -> usize {
    let mut to_visit = vec![(*point, dir)];
    while let Some((point, direction)) = to_visit.pop() {
        if let Some(cell) = grid.get_mut(&point) {
            if cell.data.directions.insert(direction) {
                let (new_dir1, new_dir2) = cell.data.mirror.reflect(direction);
                to_visit.push((
                    (point.x() + new_dir1.x(), point.y() + new_dir1.y()).as_point(),
                    new_dir1,
                ));
                if let Some(new_dir2) = new_dir2 {
                    to_visit.push((
                        (point.x() + new_dir2.x(), point.y() + new_dir2.y()).as_point(),
                        new_dir2,
                    ));
                }
            }
        }
    }
    grid.cells
        .values()
        .filter(|cell| !cell.data.directions.is_empty())
        .count()
}

fn print_grid(grid: &Grid<CellData>) {
    for y in grid.y_range.clone() {
        for x in grid.x_range.clone() {
            if let Some(cell) = grid.get(&(x, y)) {
                let c = if cell.data.directions.is_empty() {
                    '.'
                } else {
                    '#'
                };
                print!("{c}");
            }
        }
        println!();
    }
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn example_1() {
        let mut grid: Grid<CellData> = EXAMPLE.parse().unwrap();
        assert_eq!(
            energize(&mut grid, &(0_isize, 0).as_point(), Direction::East),
            46
        );
    }

    #[test]
    fn part_1() {
        let mut grid: Grid<CellData> = read_input().parse().unwrap();
        assert_eq!(
            energize(&mut grid, &(0_isize, 0).as_point(), Direction::East),
            7067
        );
    }

    #[test]
    fn example_2() {
        let grid: Grid<CellData> = EXAMPLE.parse().unwrap();
        assert_eq!(max_energize(&grid), 51);
    }

    #[test]
    fn part_2() {
        // init_tracing();
        let grid: Grid<CellData> = read_input().parse().unwrap();
        assert_eq!(max_energize(&grid), 7324);
    }
}
