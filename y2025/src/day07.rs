use advent_of_code_common::Solver;
use advent_of_code_common::grid::{Direction, Grid, Point};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellData {
    Empty,
    Splitter,
    Beam(usize),
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

fn propagate_beams(input: &str) -> (usize, usize) {
    let mut grid: Grid<CellData> = input.parse().expect("valid grid");
    let mut split_count = 0;
    let mut is_last_step = false;
    while !is_last_step {
        let current_beams: Vec<(Point, usize)> = grid
            .iter()
            .filter_map(|(point, cell)| match cell.data {
                CellData::Empty | CellData::Splitter => None,
                CellData::Beam(n) => Some((*point, n)),
            })
            .collect();
        for (beam, n) in current_beams {
            if beam.y >= grid.height() - 2 {
                is_last_step = true;
            }
            if let Some(cell) = grid.get_mut(&beam) {
                cell.data = CellData::Empty;
            }
            let below = beam + Direction::South;
            if let Some(cell) = grid.get_mut(&below) {
                match cell.data {
                    CellData::Empty => cell.data = CellData::Beam(n),
                    CellData::Splitter => {
                        split_count += 1;
                        for dir in [Direction::SouthWest, Direction::SouthEast] {
                            if let Some(cell) = grid.get_mut(&(beam + dir)) {
                                match cell.data {
                                    CellData::Empty => cell.data = CellData::Beam(n),
                                    CellData::Beam(count) => cell.data = CellData::Beam(count + n),
                                    CellData::Splitter => unreachable!("touching splitters"),
                                }
                            }
                        }
                    }
                    CellData::Beam(below_n) => cell.data = CellData::Beam(below_n + n),
                }
            }
        }
    }
    let timelines = grid
        .iter()
        .filter_map(|(_, cell)| match cell.data {
            CellData::Beam(n) => Some(n),
            _ => None,
        })
        .sum();
    (split_count, timelines)
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        propagate_beams(input).0
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        propagate_beams(input).1
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
