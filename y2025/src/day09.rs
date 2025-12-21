use advent_of_code_common::Solver;
use advent_of_code_common::coords::Point;
use itertools::Itertools;

fn parse_points(input: &str) -> Vec<Point> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

const fn area(a: &Point, b: &Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

struct Part1(usize);
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_points(input)
            .iter()
            .tuple_combinations()
            .map(|(a, b)| area(a, b))
            .max()
            .unwrap_or(0)
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let points = parse_points(input);
        points
            .iter()
            .tuple_combinations()
            .map(|(a, b)| (a, b, area(a, b)))
            .sorted_by_key(|(_, _, area)| *area)
            .rev()
            .find(|(a, b, _)| {
                let min_x = a.x.min(b.x);
                let max_x = a.x.max(b.x);
                let min_y = a.y.min(b.y);
                let max_y = a.y.max(b.y);
                for (c, d) in points.iter().circular_tuple_windows() {
                    let to_the_left = c.x <= min_x && d.x <= min_x;
                    let to_the_right = c.x >= max_x && d.x >= max_x;
                    let above = c.y <= min_y && d.y <= min_y;
                    let below = c.y >= max_y && d.y >= max_y;
                    if !(to_the_left || to_the_right || above || below) {
                        return false;
                    }
                }
                true
            })
            .map_or(0, |(_, _, area)| area)
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
        expect_solution!(Part1(10), 0, 50);
        expect_solution!(Part1(1000), 1, 4741451444);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 24);
        expect_solution!(Part2, 1, 1562459680);
    }
}
