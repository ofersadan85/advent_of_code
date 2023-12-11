use anyhow::{anyhow, Context, Result};
use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PipeShape {
    Vertical,
    Horizontal,
    CornerBottomRight,
    CornerBottomLeft,
    CornerTopRight,
    CornerTopLeft,
    #[default]
    Unknown,
    NoPipe,
}

impl TryFrom<char> for PipeShape {
    type Error = &'static str;

    #[allow(clippy::enum_glob_use)]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use PipeShape::*;
        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'J' => Ok(CornerBottomRight),
            'L' => Ok(CornerBottomLeft),
            '7' => Ok(CornerTopRight),
            'F' => Ok(CornerTopLeft),
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
            Self::CornerBottomRight => 'J',
            Self::CornerBottomLeft => 'L',
            Self::CornerTopRight => '7',
            Self::CornerTopLeft => 'F',
            Self::Unknown => '*',
            Self::NoPipe => ' ',
        }
    }

    pub const fn to_ascii_border(&self) -> char {
        match self {
            Self::CornerTopLeft => '┌',
            Self::CornerTopRight => '┐',
            Self::CornerBottomRight => '┘',
            Self::CornerBottomLeft => '└',
            Self::Horizontal => '─',
            Self::Vertical => '│',
            Self::Unknown => '*',
            Self::NoPipe => ' ',
        }
    }
}

type PipeGraph = UnGraphMap<Pipe, usize>;

pub struct PipeMap {
    pub start: Pipe,
    pub pipes: HashMap<(usize, usize), Pipe>,
    pub graph: PipeGraph,
}

impl ToString for PipeMap {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut width = 0;
        let mut height = 0;
        for (x, y) in self.pipes.keys() {
            width = width.max(*x);
            height = height.max(*y);
        }
        for y in 0..=height {
            for x in 0..=width {
                if (x, y) == (self.start.x, self.start.y) {
                    s.push('█');
                } else {
                    let pipe = self.pipes.get(&(x, y)).copied().unwrap_or(Pipe {
                        shape: PipeShape::NoPipe,
                        x,
                        y,
                    });
                    s.push(pipe.shape.to_ascii_border());
                }
            }
            s.push('\n');
        }
        s
    }
}

impl Debug for PipeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PipeMap ({:?}):", self.start)?;
        write!(f, "{}", self.to_string())
    }
}

impl PipeMap {
    fn get_distances(&self) -> HashMap<Pipe, usize> {
        dijkstra(&self.graph, self.start, None, |_| 1)
    }

    pub fn furthest_node(&self) -> usize {
        self.get_distances().values().max().unwrap_or(&0).to_owned()
    }

    fn prune_non_loop_nodes(&mut self) {
        let distances = self.get_distances();
        let mut remove = vec![];
        for ((x, y), pipe) in &self.pipes {
            if !distances.contains_key(pipe) {
                self.graph.remove_node(*pipe);
                remove.push((*x, *y));
            }
        }
        for (x, y) in remove {
            self.pipes.remove(&(x, y));
        }
    }

    fn fix_start_node(pipes: &mut HashMap<(usize, usize), Pipe>) -> Result<PipeShape> {
        let sp = pipes
            .values()
            .find(|pipe| pipe.shape == PipeShape::Unknown)
            .context("No start found")?;
        let conn = sp.connections(pipes);
        let connected_right = conn.contains(&(sp.x + 1, sp.y));
        let connected_left = sp.x > 0 && conn.contains(&(sp.x - 1, sp.y));
        let connected_up = sp.y > 0 && conn.contains(&(sp.x, sp.y - 1));
        let connected_down = conn.contains(&(sp.x, sp.y + 1));
        let new_shape = if connected_down && connected_left {
            PipeShape::CornerTopRight
        } else if connected_down && connected_right {
            PipeShape::CornerTopLeft
        } else if connected_up && connected_left {
            PipeShape::CornerBottomRight
        } else if connected_up && connected_right {
            PipeShape::CornerBottomLeft
        } else if connected_down && connected_up {
            PipeShape::Vertical
        } else if connected_left && connected_right {
            PipeShape::Horizontal
        } else {
            return Err(anyhow!("Couldn't resolve start node"));
        };
        pipes
            .get_mut(&(sp.x, sp.y))
            .expect("We know start is here")
            .shape = new_shape;
        Ok(new_shape)
    }
}

impl FromStr for PipeMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pipes: HashMap<(usize, usize), Pipe> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| (x, y, c))
                    .filter_map(|(x, y, c)| {
                        let shape = PipeShape::try_from(c).ok()?;
                        Some(((x, y), Pipe::new(shape, x, y)))
                    })
            })
            .collect();
        let (start_x, start_y) = pipes
            .values()
            .find(|pipe| pipe.shape == PipeShape::Unknown)
            .map(|pipe| (pipe.x, pipe.y))
            .ok_or("No start found")?;
        Self::fix_start_node(&mut pipes).map_err(|_| "Couldn't resolve start node")?;
        let start = pipes[&(start_x, start_y)];
        let mut graph: PipeGraph = UnGraphMap::new();
        for pipe in pipes.values() {
            for other in pipe.connections(&pipes) {
                graph.add_edge(*pipe, pipes[&other], 1);
            }
        }
        let mut result = Self {
            start,
            pipes,
            graph,
        };
        result.prune_non_loop_nodes();
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Pipe {
    pub shape: PipeShape,
    pub x: usize,
    pub y: usize,
}

impl Ord for Pipe {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Pipe {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Pipe {
    const fn new(shape: PipeShape, x: usize, y: usize) -> Self {
        Self { shape, x, y }
    }

    #[allow(clippy::enum_glob_use)]
    const fn has_connection_bottom(&self) -> bool {
        use PipeShape::*;
        match self.shape {
            Unknown | Vertical | CornerTopRight | CornerTopLeft => true,
            NoPipe | Horizontal | CornerBottomRight | CornerBottomLeft => false,
        }
    }

    #[allow(clippy::enum_glob_use)]
    const fn has_connection_top(&self) -> bool {
        use PipeShape::*;
        match self.shape {
            Unknown | Vertical | CornerBottomRight | CornerBottomLeft => true,
            NoPipe | Horizontal | CornerTopRight | CornerTopLeft => false,
        }
    }

    #[allow(clippy::enum_glob_use)]
    const fn has_connection_left(&self) -> bool {
        use PipeShape::*;
        match self.shape {
            Unknown | Horizontal | CornerBottomRight | CornerTopRight => true,
            NoPipe | Vertical | CornerBottomLeft | CornerTopLeft => false,
        }
    }

    #[allow(clippy::enum_glob_use)]
    const fn has_connection_right(&self) -> bool {
        use PipeShape::*;
        match self.shape {
            Unknown | Horizontal | CornerBottomLeft | CornerTopLeft => true,
            NoPipe | Vertical | CornerBottomRight | CornerTopRight => false,
        }
    }

    pub fn connections(&self, map: &HashMap<(usize, usize), Self>) -> Vec<(usize, usize)> {
        let mut connections = Vec::new();
        if self.has_connection_bottom() && self.y < usize::MAX {
            let other = map.get(&(self.x, self.y + 1));
            match other {
                Some(other) if other.has_connection_top() => connections.push((self.x, self.y + 1)),
                _ => (),
            }
        }
        if self.has_connection_top() && self.y > 0 {
            let other = map.get(&(self.x, self.y - 1));
            match other {
                Some(other) if other.has_connection_bottom() => {
                    connections.push((self.x, self.y - 1));
                }
                _ => (),
            }
        }
        if self.has_connection_left() && self.x > 0 {
            let other = map.get(&(self.x - 1, self.y));
            match other {
                Some(other) if other.has_connection_right() => {
                    connections.push((self.x - 1, self.y));
                }
                _ => (),
            }
        }
        if self.has_connection_right() && self.x < usize::MAX {
            let other = map.get(&(self.x + 1, self.y));
            match other {
                Some(other) if other.has_connection_left() => {
                    connections.push((self.x + 1, self.y));
                }
                _ => (),
            }
        }
        connections
    }
}

pub fn loop_contains(input: &str) -> Result<usize> {
    // Pad the input with empty cells, so we can flood fill from the start
    let map: PipeMap = input
        .parse()
        .map_err(|e| anyhow!("Couldn't parse input: {}", e))?;
    let loop_nodes: Vec<_> = dijkstra(&map.graph, map.start, None, |_| 1)
        .keys()
        .copied()
        .map(|pipe| (pipe.x + 1, pipe.y + 1)) // +1 for the padding
        .collect();
    let height = input.lines().count() + 2; // +2 for the padding
    let width = input.lines().next().unwrap_or("").chars().count() + 2; // +2 for the padding
    let mut cells = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            if loop_nodes.contains(&(x, y)) {
                let input_cell = input
                    .lines()
                    .nth(y - 1)
                    .unwrap_or("")
                    .trim()
                    .chars()
                    .nth(x - 1)
                    .unwrap_or('.');
                row.push(input_cell);
            } else {
                row.push('.');
            }
        }
        cells.push(row);
    }

    // Flood fill from the start (0, 0) and count the number of cells that are filled
    // We know 0,0 is not in the loop because we padded the input
    // We know the loop is a single connected component, so we can stop when we hit the edge
    let mut count = 0;
    let mut queue = vec![(0, 0)];
    let mut visited: HashSet<(usize, usize)> = loop_nodes.iter().copied().collect();
    while let Some((x, y)) = queue.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        count += 1;
        if x + 1 < width && cells[y][x + 1] == '.' {
            queue.push((x + 1, y));
        }
        if x > 0 && cells[y][x - 1] == '.' {
            queue.push((x - 1, y));
        }
        if y + 1 < height && cells[y + 1][x] == '.' {
            queue.push((x, y + 1));
        }
        if y > 0 && cells[y - 1][x] == '.' {
            queue.push((x, y - 1));
        }
    }

    Ok(height * width - loop_nodes.len() - count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // cSpell:disable
        let input = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";
        // cSpell:enable
        let map = input.parse::<PipeMap>().unwrap();
        assert_eq!(map.furthest_node(), 4);
    }

    #[test]
    fn example2() {
        // cSpell:disable
        let input = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";
        // cSpell:enable
        let map = input.parse::<PipeMap>().unwrap();
        assert_eq!(map.furthest_node(), 8);
    }

    #[test]
    fn part1() {
        let input = include_str!("day10.txt");
        let map = input.parse::<PipeMap>().unwrap();
        assert_eq!(map.furthest_node(), 6812);
    }

    #[test]
    fn loop_contains_example1() {
        // cSpell:disable
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
        let result = loop_contains(input).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn loop_contains_example2() {
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
        let result = loop_contains(input).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn part2() {
        let input = include_str!("day10.txt");
        let result = loop_contains(input).unwrap();
        assert!(result < 796);
    }
}

// pub fn loop_contains2(input: &str) -> usize {
//     let mut pipes = HashMap::new();
//     for (y, line) in input.lines().enumerate() {
//         for (x, c) in line.trim().chars().enumerate() {
//             if let Ok(shape) = PipeShape::try_from(c) {
//                 let pipe = Pipe::new(shape, x, y);
//                 if pipe.shape == PipeShape::Unknown {
//                     start = (x, y);
//                 }
//                 pipes.insert((x, y), pipe);
//             }
//         }
//     }
//     let new_shape = fix_start_node(&mut pipes).expect("Couldn't resolve start node");

//     // Pad the input with empty cells, so we can flood fill from the start
//     let (start, graph) = parse_input_graph(input);
//     let loop_nodes: Vec<_> = dijkstra(&graph, start, None, |_| 1)
//         .keys()
//         .copied()
//         .map(|pipe| (pipe.x + 1, pipe.y + 1)) // +1 for the padding
//         .collect();
//     let height = input.lines().count() + 2; // +2 for the padding
//     let width = input.lines().next().unwrap_or("").chars().count() + 2; // +2 for the padding
//     let mut cells = vec![];
//     for y in 0..height {
//         let mut row = vec![];
//         for x in 0..width {
//             if (x, y) == (start.x + 1, start.y + 1) {
//                 row.push(new_shape.to_char());
//             } else if !loop_nodes.contains(&(x, y)) {
//                 row.push('.');
//             } else {
//                 let input_cell = input
//                     .lines()
//                     .nth(y - 1)
//                     .unwrap_or("")
//                     .trim()
//                     .chars()
//                     .nth(x - 1)
//                     .unwrap_or('.');
//                 row.push(input_cell);
//             }
//         }
//         cells.push(row);
//     }

//     let mut count = 0;
// let mut in_loop = false;
// for row in cells.iter() {
//     let mut borders = vec![];
//     let mut row_count = 0;
//     for (x, cell) in row.iter().enumerate() {
//         use PipeShape::*;
//         match (borders.last(), PipeShape::try_from(*cell), in_loop) {
//             (_, Err(_), true) => row_count += 1,
//             (_, Err(_), false) | (_, Ok(Horizontal), _) => (),
//             (_, Ok(Vertical), _) => in_loop = !in_loop,
//             (None, Ok(p), _) => borders.push(p),
//             (Some(Vertical), _, _) => unreachable!("Vertical not pushed to stack"),
//             (Some(Horizontal), _, _) => unreachable!("Horizontal not pushed to stack"),
//             (Some(CornerTopLeft), Ok(CornerTopRight), _)
//             | (Some(CornerBottomLeft), Ok(CornerBottomRight), _) => {
//                 borders.clear();
//                 in_loop = !in_loop;
//             }
//             (Some(CornerTopLeft), Ok(CornerBottomRight), _)
//             | (Some(CornerBottomLeft), Ok(CornerTopRight), _) => borders.clear(),
//             (Some(last), Ok(next), _) => {
//                 let status1 = format!("last: {last:?} next: {next:?} {row_count}");
//                 let status2 = format!("{}", row.iter().collect::<String>());
//                 dbg!(status1);
//                 dbg!(status2);
//                 borders.pop();
//                 in_loop = !in_loop;
//             }
//         }
//         count += row_count;
//     }
// }
//     dbg!(cells);
//     count
// }
