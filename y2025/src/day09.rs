use advent_of_code_common::Solver;
use advent_of_code_common::coords::{Direction, Point};
use colored::Colorize;
use itertools::Itertools;
use std::cmp::Ordering;

fn parse_points(input: &str) -> Vec<Point> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn valid_area(a: &Point, b: &Point, cy: &[Point], cb: &[Point]) -> Option<usize> {
    let xs = a.x.min(b.x)..=a.x.max(b.x);
    let ys = a.y.min(b.y)..=a.y.max(b.y);
    let has_y = cy.iter().any(|p| xs.contains(&p.x) && ys.contains(&p.y));
    let has_b = cb.iter().any(|p| xs.contains(&p.x) && ys.contains(&p.y));
    if has_y && has_b {
        return None;
    }
    let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
    if !cy.is_empty() && !cb.is_empty() && xs.end() - xs.start() > 1 && ys.end() - ys.start() > 1 {
        assert!(
            has_y || has_b,
            "at least one color must be present {xs:?}, {ys:?}, a={a}, b={b}"
        );
    }
    Some(area)
}

struct Part1(usize);
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_points(input)
            .iter()
            .tuple_combinations()
            .filter_map(|(a, b)| valid_area(a, b, &[], &[]))
            .max()
            .unwrap_or(0)
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

fn debug_grid(reds: &[Point], blues: &[Point], yellows: &[Point]) {
    use std::fmt::Write;
    // let mut greens = Vec::new();
    // for (a, b) in reds.iter().tuple_windows().chain(std::iter::once((&reds[reds.len() -1], &reds[0]))) {
    //     greens.extend(
    //         itertools::iproduct!(a.x.min(b.x)..=a.x.max(b.x), a.y.min(b.y)..=a.y.max(b.y))
    //             .map(|(x, y)| Point { x, y }),
    //     );
    // }
    let all_points = reds.iter().chain(blues).chain(yellows);
    let min_x = all_points.clone().map(|p| p.x).min().unwrap_or(0);
    let max_x = all_points.clone().map(|p| p.x).max().unwrap_or(0);
    let min_y = all_points.clone().map(|p| p.y).min().unwrap_or(0);
    let max_y = all_points.clone().map(|p| p.y).max().unwrap_or(0);

    let mut grid = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Point { x, y };
            if reds.contains(&p) {
                write!(&mut grid, "{}", "#".red()).unwrap();
            } else if blues.contains(&p) {
                write!(&mut grid, "{}", "#".blue()).unwrap();
            } else if yellows.contains(&p) {
                write!(&mut grid, "{}", "#".yellow()).unwrap();
            // } else if greens.contains(&p) {
            //     write!(&mut grid, "{}", "#".green()).unwrap();
            } else {
                grid.push('.');
            }
        }
        grid.push('\n');
    }
    println!("{grid}");
}

fn get_orthogonal_direction(a: Point, b: Point) -> Direction {
    match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
        (Ordering::Less, Ordering::Equal) => Direction::East,
        (Ordering::Greater, Ordering::Equal) => Direction::West,
        (Ordering::Equal, Ordering::Less) => Direction::South,
        (Ordering::Equal, Ordering::Greater) => Direction::North,
        _ => unreachable!("Invalid movement from {a} to {b}"),
    }
}

fn mark_corner(a: Point, b: Point, c: Point, yellows: &mut Vec<Point>, blues: &mut Vec<Point>) {
    let in_dir = get_orthogonal_direction(a, b);
    let out_dir = get_orthogonal_direction(b, c);
    match in_dir {
        Direction::North => {
            yellows.push(b + Direction::SouthEast);
            blues.push(b + Direction::SouthWest);
            let extras = [b + Direction::NorthWest, b + Direction::NorthEast];
            match out_dir {
                Direction::East => blues.extend_from_slice(&extras),
                Direction::West => yellows.extend_from_slice(&extras),
                _ => unreachable!("Invalid turn from {in_dir:?} to {out_dir:?}"),
            }
        }
        Direction::South => {
            yellows.push(b + Direction::NorthWest);
            blues.push(b + Direction::NorthEast);
            let extras = [b + Direction::SouthWest, b + Direction::SouthEast];
            match out_dir {
                Direction::East => yellows.extend_from_slice(&extras),
                Direction::West => blues.extend_from_slice(&extras),
                _ => unreachable!("Invalid turn from {in_dir:?} to {out_dir:?}"),
            }
        }
        Direction::East => {
            yellows.push(b + Direction::SouthWest);
            blues.push(b + Direction::NorthWest);
            let extras = [b + Direction::NorthEast, b + Direction::SouthEast];
            match out_dir {
                Direction::North => yellows.extend_from_slice(&extras),
                Direction::South => blues.extend_from_slice(&extras),
                _ => unreachable!("Invalid turn from {in_dir:?} to {out_dir:?}"),
            }
        }
        Direction::West => {
            yellows.push(b + Direction::NorthEast);
            blues.push(b + Direction::SouthEast);
            let extras = [b + Direction::NorthWest, b + Direction::SouthWest];
            match out_dir {
                Direction::North => blues.extend_from_slice(&extras),
                Direction::South => yellows.extend_from_slice(&extras),
                _ => unreachable!("Invalid turn from {in_dir:?} to {out_dir:?}"),
            }
        }
        _ => unreachable!("Invalid in_dir {in_dir:?} from {a} to {b}"),
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let points = parse_points(input);
        let mut yellow_points = Vec::with_capacity(points.len());
        let mut blue_points = Vec::with_capacity(points.len());
        for (a, b, c) in points.iter().tuple_windows() {
            mark_corner(*a, *b, *c, &mut yellow_points, &mut blue_points);
        }
        // Handle wrap-around case 1
        mark_corner(
            points[points.len() - 2],
            points[points.len() - 1],
            points[0],
            &mut yellow_points,
            &mut blue_points,
        );
        // Handle wrap-around case 2
        mark_corner(
            points[points.len() - 1],
            points[0],
            points[1],
            &mut yellow_points,
            &mut blue_points,
        );

        debug_grid(&points, &blue_points, &yellow_points);
        points
            .iter()
            .tuple_combinations()
            .filter_map(|(a, b)| valid_area(a, b, &yellow_points, &blue_points))
            .max()
            .unwrap_or(0)
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
        assert!(false, "Debugging needed for part 2");
        let input = Part2.read_file_chunk(1).unwrap();
        assert_ne!(Part2.solve(&input), 4589308260, "Too high");
        expect_solution!(Part2, 1, 2435100380);
    }
}
