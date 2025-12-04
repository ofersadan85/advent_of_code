use advent_of_code_common::Solver;
use advent_of_code_common::grid::{Grid, Point};

fn remove_points(grid: &mut Grid) -> usize {
    let removable: Vec<Point> = grid
        .iter()
        .filter_map(|(p, c)| if c.data == '@' { Some(p) } else { None })
        .filter(|&p| {
            grid.neighbors_box(p)
                .iter()
                .filter(|c| c.is_some_and(|c| c.data == '@'))
                .count()
                <= 4 // including self
        })
        .copied()
        .collect();
    for p in &removable {
        grid.set(p, '.');
    }
    removable.len()
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut grid: Grid = input.parse().expect("valid grid");
        remove_points(&mut grid)
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut grid: Grid = input.parse().expect("valid grid");
        let mut count = 0;
        loop {
            let removed = remove_points(&mut grid);
            if removed == 0 {
                break count;
            }
            count += removed;
        }
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
        expect_solution!(Part1, 0, 13);
        expect_solution!(Part1, 1, 1480);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 43);
        expect_solution!(Part2, 1, 8899);
    }
}
