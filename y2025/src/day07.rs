use advent_of_code_common::Solver;
use advent_of_code_common::grid::{Coords, Direction, Grid, Point};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellData {
    Empty,
    Splitter,
    Beam(usize),
}

impl CellData {
    const fn get_beam_count(&self) -> usize {
        match self {
            Self::Empty | Self::Splitter => 0,
            Self::Beam(n) => *n,
        }
    }
}

impl TryFrom<char> for CellData {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Splitter),
            'S' => Ok(Self::Beam(1)),
            _ => Err(()),
        }
    }
}

fn grid_step(grid: &mut Grid<CellData>, point: Point) -> usize {
    let beam = grid
        .get(&point)
        .map_or(0, |cell| cell.data.get_beam_count());
    if beam == 0 {
        return 0;
    }
    let mut split_count = 0;
    grid.get_mut(&point).expect("cell exists").data = CellData::Empty;
    let below = point + Direction::South;
    if let Some(cell) = grid.get_mut(&below) {
        match cell.data {
            CellData::Empty => cell.data = CellData::Beam(beam),
            CellData::Splitter => {
                split_count += 1;
                for dir in [Direction::SouthWest, Direction::SouthEast] {
                    let next_point = point + dir;
                    if let Some(cell) = grid.get_mut(&next_point) {
                        cell.data = match cell.data {
                            CellData::Empty => CellData::Beam(beam),
                            CellData::Beam(count) => CellData::Beam(count + beam),
                            CellData::Splitter => unreachable!("touching splitters"),
                        }
                    }
                }
            }
            CellData::Beam(value) => cell.data = CellData::Beam(value + beam),
        }
    }
    split_count
}

fn propagate_beams(input: &str) -> (Grid<CellData>, usize) {
    let mut grid: Grid<CellData> = input.parse().expect("valid grid");
    let mut split_count = 0;
    for y in grid.y_range.start..grid.y_range.end - 1 {
        for x in grid.x_range.clone() {
            split_count += grid_step(&mut grid, (x, y).as_point());
        }
    }
    (grid, split_count)
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        propagate_beams(input).1
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let grid = propagate_beams(input).0;
        grid.x_range
            .clone()
            .filter_map(|x| grid.get(&(x, grid.y_range.end - 1)))
            .map(|cell| cell.data.get_beam_count())
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 21);
        expect_solution!(Part1, 1, 1646);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 40);
        expect_solution!(Part2, 1, 32451134474991);
    }
}
