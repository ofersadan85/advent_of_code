use std::{collections::{HashMap, HashSet}, path::Display};
use tracing::{info, instrument};

pub const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::{Down, Left, Right, Up};
        match self {
            Up => write!(f, "↑"),
            Down => write!(f, "↓"),
            Left => write!(f, "←"),
            Right => write!(f, "→"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mirror {
    Slash,
    Backslash,
    SplitterHorizontal,
    SplitterVertical,
}

impl TryFrom<char> for Mirror {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '/' => Ok(Self::Slash),
            '\\' => Ok(Self::Backslash),
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            _ => Err("Invalid mirror type"),
        }
    }
}

impl std::fmt::Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mirror::{Backslash, Slash, SplitterHorizontal, SplitterVertical};
        match self {
            Slash => write!(f, "/"),
            Backslash => write!(f, "\\"),
            SplitterHorizontal => write!(f, "-"),
            SplitterVertical => write!(f, "|"),
        }
    }
}

impl Mirror {
    fn split_light(&self, dir: Direction) -> (Direction, Option<Direction>) {
        use Direction::{Down, Left, Right, Up};
        use Mirror::{Backslash, Slash, SplitterHorizontal, SplitterVertical};
        match (self, dir) {
            (Slash, Up) => (Right, None),
            (Slash, Down) => (Left, None),
            (Slash, Left) => (Down, None),
            (Slash, Right) => (Up, None),
            (Backslash, Up) => (Left, None),
            (Backslash, Down) => (Right, None),
            (Backslash, Left) => (Up, None),
            (Backslash, Right) => (Down, None),
            (SplitterHorizontal, Up | Down) => (Left, Some(Right)),
            (SplitterHorizontal, Left | Right) => (dir, None),
            (SplitterVertical, Up | Down) => (dir, None),
            (SplitterVertical, Left | Right) => (Up, Some(Down)),
        }
    }
}

type Grid = HashMap<(usize, usize), Mirror>;

fn closest_mirror(x: usize, y: usize, grid: &Grid, dir: Direction) -> Option<(usize, usize)> {
    grid.keys()
        .filter(|(x_o, y_o)| match dir {
            Direction::Up => *y_o < y && *x_o == x,
            Direction::Down => *y_o > y && *x_o == x,
            Direction::Left => *x_o < x && *y_o == y,
            Direction::Right => *x_o > x && *y_o == y,
        })
        .min_by_key(|(x_o, y_o)| match dir {
            Direction::Up | Direction::Down => y_o.abs_diff(y),
            Direction::Left | Direction::Right => x_o.abs_diff(x),
        })
        .copied()
}

pub fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| Mirror::try_from(c).ok().map(|m| ((x, y), m)))
        })
        .collect()
}

#[instrument(skip_all, level = "info")]
pub fn energize(grid: &Grid, width: usize, height: usize) -> HashSet<(usize, usize)> {
    use Direction::{Down, Left, Right, Up};
    let mut visited = HashSet::new();
    let mut visited_mirrors = HashSet::new();
    let first_direction = grid
        .get(&(0, 0))
        .map(|m| match m {
            Mirror::Slash => Up,
            Mirror::Backslash => Down,
            Mirror::SplitterHorizontal => Right,
            Mirror::SplitterVertical => Down,
        })
        .unwrap_or(Right);
    let mut queue = vec![((0, 0), first_direction)];
    while let Some((pos, dir)) = queue.pop() {
        if visited_mirrors.contains(&(pos, dir)) {
            // info!("Already visited {:?}, {:?}", pos, dir);
            continue;
        }

        // info!("Visiting {:?}, {:?}", pos, dir);
        visited.insert(pos);
        visited_mirrors.insert((pos, dir));
        let (x, y) = pos;
        let next_pos = closest_mirror(x, y, grid, dir);
        // info!("Next pos: {:?}", next_pos);
        let (next_x, next_y) = next_pos.unwrap_or_else(|| match dir {
            Up => (x, 0),
            Down => (x, height - 1),
            Left => (0, y),
            Right => (width - 1, y),
        });
        let next_pos: HashSet<(usize, usize)> = match dir {
            Up => (next_y..=y).map(|y| (x, y)).collect(),
            Down => (y..=next_y).map(|y| (x, y)).collect(),
            Left => (next_x..=x).map(|x| (x, y)).collect(),
            Right => (x..=next_x).map(|x| (x, y)).collect(),
        };
        // info!("Next pos: {:?}", next_pos);
        visited.extend(next_pos);
        if let Some(next_mirror) = grid.get(&(next_x, next_y)) {
            let (new_dir, new_dir2) = next_mirror.split_light(dir);
            queue.push(((next_x, next_y), new_dir));
            // info!("Splitting light: {:?}, {:?}", new_dir, new_dir2);
            if let Some(new_dir2) = new_dir2 {
                queue.push(((next_x, next_y), new_dir2));
            }
        }
        // print_grid(&visited, width, height);
        // println!("press enter to continue");
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }
    print_grid(&visited, &visited_mirrors, grid, width, height);
    visited
}

fn print_grid(
    map: &HashSet<(usize, usize)>,
    mirrors: &HashSet<((usize, usize), Direction)>,
    grid: &Grid,
    width: usize,
    height: usize,
) {
    for y in 0..height {
        for x in 0..width {
            if let Some((_, dir)) = mirrors
                .iter()
                .find(|((x_m, y_m), _)| *x_m == x && *y_m == y)
            {
                if let Some(_) = grid.get(&(x, y)) {
                    print!("{}{}{}", "\x1b[1;31m", dir, "\x1b[0m");
                } else {
                    print!("{}*{}", "\x1b[1;31m", "\x1b[0m");
                }
            } else if grid.contains_key(&(x, y)) {
                print!("{}", grid[&(x, y)]);
            } else if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::init_tracing;

    #[test]
    // #[ignore]
    fn example1() {
        // init_tracing();
        let grid = parse_input(EXAMPLE);
        let width = EXAMPLE.lines().next().unwrap().len();
        let height = EXAMPLE.lines().count();
        let energized = energize(&grid, width, height);
        assert_eq!(energized.len(), 46);
    }

    #[test]
    fn part1() {
        init_tracing();
        let input = include_str!("day16.txt");
        let grid = parse_input(input);
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let energized = energize(&grid, width, height);
        for (x, y) in &energized {
            if *y >= height || *x >= width {
                println!("({}, {})", x, y);
            }
        }
        println!("Width: {}, Height: {}", width, height);
        assert_ne!(energized.len(), 7156); // Too high
        assert_ne!(energized.len(), 7386); // Too high
    }
}
