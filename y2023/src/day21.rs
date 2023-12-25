use anyhow::{anyhow, Context, Result};
use petgraph::{algo::dijkstra, graphmap::UnGraphMap};
use std::collections::HashMap;
use tracing::instrument;

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

pub type Maze = UnGraphMap<(i32, i32), u8>;
pub type MazeMap = HashMap<(i32, i32), Tile>;

#[instrument(skip_all, level = "info")]
pub fn parse_input(s: &str) -> Result<((i32, i32), Maze)> {
    let width = i32::try_from(s.lines().next().context("No lines")?.len())?;
    let height = i32::try_from(s.lines().count())?;
    let map: MazeMap = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                Some((
                    (i32::try_from(x).ok()?, i32::try_from(y).ok()?),
                    Tile::try_from(c).ok()?,
                ))
            })
        })
        .collect();
    let graph = Maze::from_edges(
        map.iter()
            .filter(|(_, tile)| tile != &&Tile::Rock)
            .flat_map(|((x, y), _)| {
                let x = *x;
                let y = *y;
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height) // Valid neighbor coordinates
                    .filter(|n| {
                        map.get(n) == Some(&Tile::Grass) || map.get(n) == Some(&Tile::Start)
                    }) // Valid tiles
                    .map(|(n_x, n_y)| ((x, y), (*n_x, *n_y), 1))
                    .collect::<Vec<_>>() // As edge
            }),
    );
    let (start, _tile) = map
        .iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .ok_or_else(|| anyhow!("No start found"))?;
    // print_distance_map(start, &graph);
    Ok((*start, graph))
}

#[instrument(skip_all, level = "info")]
pub fn print_distance_map(start: (i32, i32), graph: &Maze) {
    let distances = dijkstra(graph, start, None, |(_, _, w)| *w);
    let width = distances
        .iter()
        .max_by_key(|((x, _), _)| x)
        .map_or(0, |((x, _), _)| x + 1);
    let height = distances
        .iter()
        .max_by_key(|((_, y), _)| y)
        .map_or(0, |((_, y), _)| y + 1);
    let red_color = "\x1b[31m";
    let green_color = "\x1b[32m";
    let reset_color = "\x1b[0m";
    for y in 0..height {
        for x in 0..width {
            if let Some(distance) = distances.get(&(x, y)) {
                if distance == &10 {
                    print!("{}{:3}{}", red_color, distance, reset_color);
                } else if distance == &20 {
                    print!("{}{:3}{}", green_color, distance, reset_color);
                } else {
                    print!("{:3}", distance);
                }
            } else {
                print!("   ");
            }
        }
        println!();
    }
}

#[instrument(skip_all, level = "info")]
pub fn find_even_steps(start: (i32, i32), graph: &Maze, max: i32) -> usize {
    dijkstra(graph, start, None, |_| 1)
        .values()
        .filter(|&&distance| distance % 2 == 0 && distance <= max)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tracing_subscriber::{fmt, fmt::format::FmtSpan};
    static TRACING_INIT: AtomicBool = AtomicBool::new(false);

    fn init_tracing() {
        let tracing =
            TRACING_INIT.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
        if let Ok(false) = tracing {
            fmt()
                .with_line_number(true)
                .with_span_events(FmtSpan::CLOSE)
                .init();
        }
    }

    #[test]
    fn example() {
        init_tracing();
        let (start, graph) = parse_input(EXAMPLE).unwrap();
        assert_eq!(find_even_steps(start, &graph, 6), 16);
    }

    #[test]
    fn part1() {
        init_tracing();
        let (start, graph) = parse_input(include_str!("day21.txt")).unwrap();
        assert_eq!(find_even_steps(start, &graph, 64), 3729);
    }
}
