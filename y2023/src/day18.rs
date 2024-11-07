use anyhow::Result;
use geo::{area::Area, Coord, LineString, Polygon};

pub const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Invalid direction"),
        }
    }
}

impl From<Direction> for Coord {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { x: 0.0, y: 1.0 },
            Direction::Down => Self { x: 0.0, y: -1.0 },
            Direction::Left => Self { x: -1.0, y: 0.0 },
            Direction::Right => Self { x: 1.0, y: 0.0 },
        }
    }
}

pub fn line_parser_1(s: &str) -> Option<(Direction, f64)> {
    let mut parts = s.split_whitespace();
    let direction = parts
        .next()
        .and_then(|s| s.chars().next())
        .and_then(|c| Direction::try_from(c).ok())?;
    let distance: f64 = parts.next().and_then(|s| s.parse().ok())?;
    Some((direction, distance))
}

pub fn line_parser_2(s: &str) -> Option<(Direction, f64)> {
    let (hex, dir) = s.split_once('#')?.1.split_at(5);
    let distance = f64::from(u32::from_str_radix(hex, 16).ok()?);
    let direction = dir.chars().next().and_then(|c| match c {
        '0' => Some(Direction::Right),
        '1' => Some(Direction::Down),
        '2' => Some(Direction::Left),
        '3' => Some(Direction::Up),
        _ => None,
    })?;
    Some((direction, distance))
}

#[allow(clippy::float_cmp)] // We're not doing any floating point math here, other that the assert_eq! below
#[allow(clippy::cast_sign_loss)] // We know the result is always positive
#[allow(clippy::cast_possible_truncation)] // We're checking that the result is an integer
pub fn polygon_area<F>(s: &str, line_parser: F) -> usize
where
    F: Fn(&str) -> Option<(Direction, f64)>,
{
    let mut points: Vec<Coord> = vec![Coord { x: 0.0, y: 0.0 }];
    let mut perimeter = 0.0;
    s.lines()
        .filter_map(line_parser)
        .for_each(|(direction, distance)| {
            let prev_point = points.last().copied().expect("There are always points");
            let travel = Coord::from(direction) * distance;
            let next_point = prev_point + travel;
            perimeter += distance;
            points.push(next_point);
        });
    let polygon = Polygon::new(LineString(points), vec![]);
    let area = polygon.unsigned_area();
    // Pick's theorem: A = i + b/2 - 1  (https://en.wikipedia.org/wiki/Pick%27s_theorem)
    // However in our case, i = A + P/2 + 1 where P is the perimeter of the polygon
    // This is because the result we want is i + P, not just i
    let inner_points = area + perimeter / 2.0 + 1.0;
    assert_eq!(inner_points, inner_points.floor()); // This should always be an integer
    inner_points as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = polygon_area(EXAMPLE, line_parser_1);
        assert_eq!(result, 62);
    }

    #[test]
    fn part1() {
        let input = include_str!("../../inputs/2023/day18.txt");
        let result = polygon_area(input, line_parser_1);
        assert_eq!(result, 62500);
    }

    #[test]
    fn example2() {
        let result = polygon_area(EXAMPLE, line_parser_2);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../inputs/2023/day18.txt");
        let result = polygon_area(input, line_parser_2);
        assert_eq!(result, 122109860712709);
    }
}
