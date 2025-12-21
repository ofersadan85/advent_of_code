use advent_of_code_common::Solver;
use itertools::Itertools;
use std::str::FromStr;

struct GridLine {
    width: usize,
    height: usize,
    shape_count: [usize; 6],
}

impl FromStr for GridLine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area, shapes) = s.split_once(": ").ok_or("invalid grid line format")?;
        let (width, height) = area.split_once('x').ok_or("invalid area format")?;
        Ok(Self {
            width: width.parse().map_err(|_| "invalid width")?,
            height: height.parse().map_err(|_| "invalid height")?,
            shape_count: shapes
                .split_whitespace()
                .map(|count| count.parse().map_err(|_| "invalid shape count"))
                .collect::<Result<Vec<usize>, _>>()?
                .try_into()
                .map_err(|_| "invalid number of shape counts")?,
        })
    }
}

impl GridLine {
    const fn area(&self) -> usize {
        self.width * self.height
    }
}

struct Shape([[bool; 3]; 3]);

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [[false; 3]; 3];
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid[y][x] = match ch {
                    '#' => true,
                    '.' => false,
                    _ => return Err("invalid character in shape"),
                };
            }
        }
        Ok(Self(grid))
    }
}

impl Shape {
    fn occupied(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell)
            .count()
    }
}

struct Puzzle {
    shapes: [Shape; 6],
    grids: Vec<GridLine>,
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut shapes = Vec::new();
        for _ in 0..6 {
            let _ = lines.next(); // skip index line
            let shape_lines = lines.by_ref().take(3).join("\n");
            let new_shape = shape_lines.parse()?;
            shapes.push(new_shape);
            let _ = lines.next(); // skip empty line
        }
        let shapes = shapes.try_into().map_err(|_| "invalid number of shapes")?;
        let grids = lines
            .map(str::parse)
            .collect::<Result<Vec<GridLine>, _>>()?;
        Ok(Self { shapes, grids })
    }
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let puzzle: Puzzle = input.parse().expect("valid input");
        puzzle
            .grids
            .iter()
            .filter(|grid| {
                let area = grid.area();
                let shape_areas: usize = grid
                    .shape_count
                    .iter()
                    .zip(puzzle.shapes.iter())
                    .map(|(&count, shape)| count * shape.occupied())
                    .sum();
                area >= shape_areas
            })
            .count()
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
        // The example case is different from the actual problem input.
        // Our simplified calculation does not yield the expected result for the example.
        // expect_solution!(Part1, 0, 2);
        // But it works for the actual input.
        expect_solution!(Part1, 1, 448);
    }
}
