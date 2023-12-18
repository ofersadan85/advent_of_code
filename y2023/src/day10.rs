use anyhow::{anyhow, Context, Result};
use geo::{area::Area, Coord, CoordsIter, Polygon};
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PipeShape {
    Vertical,
    Horizontal,
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
    #[default]
    Unknown,
    NoPipe,
}

impl PipeShape {
    fn connections(self) -> Vec<Coord<i32>> {
        use PipeShape::{
            BottomLeft, BottomRight, Horizontal, NoPipe, TopLeft, TopRight, Unknown, Vertical,
        };
        match self {
            Vertical => vec![Coord { x: 0, y: 1 }, Coord { x: 0, y: -1 }],
            Horizontal => vec![Coord { x: 1, y: 0 }, Coord { x: -1, y: 0 }],
            BottomRight => vec![Coord { x: 0, y: -1 }, Coord { x: -1, y: 0 }],
            BottomLeft => vec![Coord { x: 0, y: -1 }, Coord { x: 1, y: 0 }],
            TopRight => vec![Coord { x: 0, y: 1 }, Coord { x: -1, y: 0 }],
            TopLeft => vec![Coord { x: 0, y: 1 }, Coord { x: 1, y: 0 }],
            Unknown | NoPipe => vec![],
        }
    }
}

impl TryFrom<char> for PipeShape {
    type Error = &'static str;

    #[allow(clippy::enum_glob_use)]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use PipeShape::*;
        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'J' => Ok(BottomRight),
            'L' => Ok(BottomLeft),
            '7' => Ok(TopRight),
            'F' => Ok(TopLeft),
            'S' => Ok(Unknown),
            '.' | ' ' => Ok(NoPipe),
            _ => Err("Invalid pipe shape"),
        }
    }
}

impl PipeShape {
    pub const fn to_char(&self) -> char {
        match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::BottomRight => 'J',
            Self::BottomLeft => 'L',
            Self::TopRight => '7',
            Self::TopLeft => 'F',
            Self::Unknown => '*',
            Self::NoPipe => ' ',
        }
    }

    pub const fn to_ascii_border(&self) -> char {
        match self {
            Self::TopLeft => '┌',
            Self::TopRight => '┐',
            Self::BottomRight => '┘',
            Self::BottomLeft => '└',
            Self::Horizontal => '─',
            Self::Vertical => '│',
            Self::Unknown => '*',
            Self::NoPipe => ' ',
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    coord: Coord<i32>,
    shape: PipeShape,
}

impl Pipe {
    fn connections(&self) -> Vec<Coord<i32>> {
        self.shape
            .connections()
            .iter()
            .map(|&c| c + self.coord)
            .collect()
    }
}

fn fix_start_node(pipes: &mut HashMap<Coord<i32>, Pipe>) -> Result<Coord<i32>> {
    let sp = pipes
        .values()
        .find(|pipe| pipe.shape == PipeShape::Unknown)
        .copied()
        .context("No start found")?;
    let connected_pipes: Vec<_> = pipes
        .values()
        .filter(|pipe| pipe.connections().contains(&sp.coord))
        .map(|pipe| pipe.coord)
        .collect();
    if connected_pipes.len() != 2 {
        return Err(anyhow!(
            "Start node has {} connections",
            connected_pipes.len()
        ));
    }

    let up = connected_pipes.contains(&(sp.coord + Coord { x: 0, y: -1 }));
    let down = connected_pipes.contains(&(sp.coord + Coord { x: 0, y: 1 }));
    let left = connected_pipes.contains(&(sp.coord + Coord { x: -1, y: 0 }));
    let right = connected_pipes.contains(&(sp.coord + Coord { x: 1, y: 0 }));
    let new_shape = match (up, down, left, right) {
        (true, true, false, false) => PipeShape::Vertical,
        (false, false, true, true) => PipeShape::Horizontal,
        (true, false, true, false) => PipeShape::BottomRight,
        (true, false, false, true) => PipeShape::BottomLeft,
        (false, true, true, false) => PipeShape::TopRight,
        (false, true, false, true) => PipeShape::TopLeft,
        _ => return Err(anyhow!("Couldn't resolve start node shape")),
    };
    pipes
        .get_mut(&sp.coord)
        .expect("We know start is here")
        .shape = new_shape;
    Ok(sp.coord)
}

type Pipes = HashMap<Coord<i32>, Pipe>;

fn parse_input(s: &str) -> Result<Polygon<i32>> {
    let mut pipes: HashMap<Coord<i32>, Pipe> = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| (x, y, c))
                .filter_map(|(x, y, c)| {
                    let shape = PipeShape::try_from(c).ok()?;
                    let coord = Coord {
                        x: i32::try_from(x).ok()?,
                        y: i32::try_from(y).ok()?,
                    };
                    Some((coord, Pipe { coord, shape }))
                })
        })
        .collect();
    let start = fix_start_node(&mut pipes)?;
    pipes_polygon(&pipes, start)
}

fn pipes_polygon(pipes: &Pipes, start: Coord<i32>) -> Result<Polygon<i32>> {
    let mut points = vec![start];
    let mut next_coords = pipes[&start].connections()[0];
    while next_coords != start {
        points.push(next_coords);
        let next_connections = pipes
            .get(&next_coords)
            .context("No pipe found")?
            .connections();
        if points.contains(&next_connections[0]) {
            next_coords = next_connections[1];
        } else {
            next_coords = next_connections[0];
        }
    }
    points.push(start);
    Ok(Polygon::new(points.into(), vec![]))
}

pub fn furthest_node(input: &str) -> Result<usize> {
    Ok(parse_input(input)?.exterior().points().len() / 2)
}

#[allow(clippy::float_cmp)] // We're not doing any floating point math here, other that the assert_eq! below
#[allow(clippy::cast_sign_loss)] // We know the result is always positive
#[allow(clippy::cast_precision_loss)] // Tested it, and it's fine
#[allow(clippy::cast_possible_truncation)] // We're checking that the result is an integer
pub fn inner_area(input: &str) -> Result<usize> {
    let coords: Vec<_> = parse_input(input)?
        .exterior_coords_iter()
        .map(|c| Coord {
            x: f64::from(c.x),
            y: f64::from(c.y),
        })
        .collect();
    let perimeter = coords.len() - 1; // The last point is the same as the first
    let polygon = Polygon::new(coords.into(), vec![]);
    let area = polygon.unsigned_area();
    // Pick's theorem: A = i + b/2 - 1  (https://en.wikipedia.org/wiki/Pick%27s_theorem)
    // So i = A - b/2 - 1
    let inner_points = area - perimeter as f64 / 2.0 + 1.0;
    assert_eq!(inner_points, inner_points.floor()); // This should always be an integer
    Ok(inner_points as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        // cSpell:disable
        let input = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";
        // cSpell:enable
        assert_eq!(furthest_node(input).unwrap(), 4);
    }

    #[test]
    fn part1_example2() {
        // cSpell:disable
        let input = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";
        // cSpell:enable
        assert_eq!(furthest_node(input).unwrap(), 8);
    }

    #[test]
    fn part1() {
        let input = include_str!("day10.txt");
        assert_eq!(furthest_node(input).unwrap(), 6812);
    }

    #[test]
    fn part2_example1() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        // cSpell:enable
        let result = inner_area(input).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_example2() {
        // cSpell:disable
        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        // cSpell:enable
        let result = inner_area(input).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn part2() {
        let input = include_str!("day10.txt");
        let result = inner_area(input).unwrap();
        assert_eq!(result, 527);
    }
}
