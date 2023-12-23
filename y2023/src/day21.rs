use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use geo::Coord;
use petgraph::{algo::dijkstra, graphmap::UnGraphMap};

pub const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Rock,
    Grass,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Coord<i32> {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => Self { x: 0, y: -1 },
            Direction::Down => Self { x: 0, y: 1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Grass),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            _ => Err("Invalid tile"),
        }
    }
}

pub fn neighbors(coord: Coord<i32>, width: i32, height: i32) -> Vec<Coord<i32>> {
    use Direction::{Down, Left, Right, Up};
    let mut neighbors = Vec::new();
    if coord.x > 0 {
        neighbors.push(coord + Left.into());
    }
    if coord.x < width - 1 {
        neighbors.push(coord + Right.into());
    }
    if coord.y > 0 {
        neighbors.push(coord + Up.into());
    }
    if coord.y < height - 1 {
        neighbors.push(coord + Down.into());
    }
    neighbors
}

pub type Maze = UnGraphMap<(i32, i32), u8>;
pub type MazeMap = HashMap<Coord<i32>, Tile>;

pub fn parse_input(s: &str) -> Result<(Coord<i32>, Maze)> {
    let mut graph = Maze::new();
    let width = i32::try_from(s.lines().next().context("No lines")?.len())?;
    let height = i32::try_from(s.lines().count())?;
    let mut map = MazeMap::new();
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let tile = Tile::try_from(c).ok()?;
                Some((
                    Coord {
                        x: i32::try_from(x).ok()?,
                        y: i32::try_from(y).ok()?,
                    },
                    tile,
                ))
            })
        })
        .for_each(|(coord, tile)| {
            map.insert(coord, tile);
        });
    for (coord, tile) in &map {
        if *tile == Tile::Rock {
            continue;
        }
        neighbors(*coord, width, height)
            .iter()
            .filter(|n| map.get(n) == Some(&Tile::Grass))
            .for_each(|n| {
                graph.add_edge(coord.x_y(), n.x_y(), 1);
            });
    }
    let start = map
        .iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .map(|(coord, _)| *coord)
        .ok_or_else(|| anyhow!("No start found"))?;
    // print_distance_map(start, &graph);
    Ok((start, graph))
}

pub fn print_distance_map(start: Coord<i32>, graph: &Maze) {
    let distances = dijkstra(graph, start.x_y(), None, |(_, _, w)| *w);
    let width = distances
        .iter()
        .max_by_key(|((x, _), _)| x)
        .map_or(0, |((x, _), _)| x + 1);
    let height = distances
        .iter()
        .max_by_key(|((_, y), _)| y)
        .map_or(0, |((_, y), _)| y + 1);
    for y in 0..height {
        for x in 0..width {
            let distance = distances.get(&(x, y)).unwrap_or(&0);
            print!("{distance:3} ");
        }
        println!();
    }
}

pub fn find_even_steps(start: Coord<i32>, graph: &Maze, max: i32) -> usize {
    let distances = dijkstra(graph, start.x_y(), None, |_| 1);
    dbg!(&distances[&(0, 4)]);
    distances
        .iter()
        .filter(|(_, &distance)| distance % 2 == 0 && distance <= max)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (start, graph) = parse_input(EXAMPLE).unwrap();
        assert_eq!(find_even_steps(start, &graph, 6), 16);
    }

    #[test]
    fn part1() {
        let (start, graph) = parse_input(include_str!("day21.txt")).unwrap();
        assert_eq!(find_even_steps(start, &graph, 64), 3729);
    }
}
