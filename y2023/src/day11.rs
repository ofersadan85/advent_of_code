use itertools::{iproduct, Itertools};
use petgraph::{algo::astar, graphmap::UnGraphMap};
use std::{collections::HashSet, str::FromStr};

pub const EXAMPLE: &str = "...#......
                           .......#..
                           #.........
                           ..........
                           ......#...
                           .#........
                           .........#
                           ..........
                           .......#..
                           #...#.....";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Galaxy,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Galaxy),
            _ => Err("Invalid tile"),
        }
    }
}

#[derive(Debug)]
pub struct Space {
    pub tiles: Vec<Vec<Tile>>,
    pub galaxies: Vec<(usize, usize)>, // (x, y)
    pub graph: UnGraphMap<(usize, usize), usize>,
    pub expansion: usize,
    pub galaxy_rows: HashSet<usize>,
    pub galaxy_columns: HashSet<usize>,
}

impl FromStr for Space {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .filter_map(|c| Tile::try_from(c).ok())
                    .collect_vec()
            })
            .collect_vec();
        let galaxies: Vec<(usize, usize)> = tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, tile)| match tile {
                        Tile::Galaxy => Some((x, y)),
                        Tile::Empty => None,
                    })
                    .map(move |(x, _)| (x, y))
            })
            .collect();
        let width = tiles[0].len();
        let height = tiles.len();
        if !tiles.iter().all(|row| row.len() == width) {
            return Err("Invalid width");
        }
        let mut graph = UnGraphMap::new();
        let step_weight = 1;
        for (x, y) in iproduct!(0..width, 0..height) {
            if x > 0 {
                graph.add_edge((x, y), (x - 1, y), step_weight);
            }
            if x < width - 1 {
                graph.add_edge((x, y), (x + 1, y), step_weight);
            }
            if y > 0 {
                graph.add_edge((x, y), (x, y - 1), step_weight);
            }
            if y < height - 1 {
                graph.add_edge((x, y), (x, y + 1), step_weight);
            }
        }

        Ok(Self {
            tiles,
            galaxies: galaxies.clone(),
            graph,
            expansion: 0,
            galaxy_rows: galaxies.iter().map(|(_, y)| *y).collect(),
            galaxy_columns: galaxies.iter().map(|(x, _)| *x).collect(),
        })
    }
}

impl Space {
    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn expand_fast(&mut self, factor: usize) {
        self.expansion += factor;
    }

    pub fn expand(&mut self) {
        let width = self.width();
        let height = self.height();
        let non_galaxy_rows = (0..height - 1).filter(|y| !self.galaxy_rows.contains(y));
        let non_galaxy_columns = (0..width - 1).filter(|x| !self.galaxy_columns.contains(x));
        iproduct!(non_galaxy_columns, 0..height).for_each(|(x, y)| {
            // println!("Changing weight of exiting column {x}");
            if let Some(w) = self.graph.edge_weight_mut((x, y), (x + 1, y)) {
                *w += 1;
            }
        });
        iproduct!(0..width, non_galaxy_rows).for_each(|(x, y)| {
            // println!("Changing weight of ({}, {}) -> ({}, {})", x, y, x, y + 1);
            if let Some(w) = self.graph.edge_weight_mut((x, y), (x, y + 1)) {
                *w += 1;
            }
        });
        self.expansion += 1;
    }

    fn pairs_of_galaxies(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(&(x1, y1), &(x2, y2))| {
                if x1 < x2 {
                    ((x1, y1), (x2, y2))
                } else {
                    ((x2, y2), (x1, y1))
                }
            })
    }

    fn distance(&self, galaxy1: (usize, usize), galaxy2: (usize, usize)) -> usize {
        let path = astar(
            &self.graph,
            galaxy1,
            |finish| finish == galaxy2,
            |(_, _, &w)| w,
            |_| 0,
        )
        .expect("There must always be a path");
        // Draw all the tiles with the path
        let mut previous = path.1[0];
        let mut output = vec![];
        for (y, row) in self.tiles.iter().enumerate() {
            let mut row_str = String::new();
            for (x, tile) in row.iter().enumerate() {
                if path.1.contains(&(x, y)) {
                    match self.graph.edge_weight(previous, (x, y)) {
                        Some(w) => row_str.push_str(&w.to_string()),
                        None => row_str.push('S'),
                    }
                    previous = (x, y);
                } else {
                    match tile {
                        Tile::Empty => row_str.push('.'),
                        Tile::Galaxy => row_str.push('#'),
                    }
                }
            }
            output.push(row_str);
        }

        for row in &output {
            println!("{row}");
        }
        println!("Distance: {}", path.0);
        path.0
    }

    pub fn total_distance(&self) -> usize {
        self.pairs_of_galaxies()
            .map(|(galaxy1, galaxy2)| self.distance(galaxy1, galaxy2))
            .sum()
    }

    pub fn distance_estimate(&self, galaxy1: (usize, usize), galaxy2: (usize, usize)) -> usize {
        let min_x = galaxy1.0.min(galaxy2.0);
        let max_x = galaxy1.0.max(galaxy2.0);
        let min_y = galaxy1.1.min(galaxy2.1);
        let max_y = galaxy1.1.max(galaxy2.1);
        let dx: usize = (min_x..max_x)
            .map(|x| {
                if self.galaxy_columns.contains(&x) {
                    1
                } else {
                    self.expansion + 1
                }
            })
            .sum();
        let dy: usize = (min_y..max_y)
            .map(|y| {
                if self.galaxy_rows.contains(&y) {
                    1
                } else {
                    self.expansion + 1
                }
            })
            .sum();
        dx + dy
    }

    pub fn total_distance_estimate(&self) -> usize {
        self.pairs_of_galaxies()
            .map(|(galaxy1, galaxy2)| self.distance_estimate(galaxy1, galaxy2))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let space = Space::from_str(EXAMPLE).unwrap();
        assert_eq!(space.width(), 10);
        assert_eq!(space.height(), 10);
        assert_eq!(space.galaxies.len(), 9);
    }

    #[test]
    fn expand() {
        let mut space = Space::from_str(EXAMPLE).unwrap();
        let mut twos = space.graph.all_edges().filter(|(_, _, w)| **w == 2).count();
        assert_eq!(twos, 0);
        space.expand();
        twos = space.graph.all_edges().filter(|(_, _, w)| **w == 2).count();
        assert_eq!(twos, 50);
    }

    #[test]
    fn pairs_of_galaxies() {
        let space = Space::from_str(EXAMPLE).unwrap();
        let pairs = space.pairs_of_galaxies().collect_vec();
        assert_eq!(pairs.len(), 36);
    }

    #[test]
    fn distance() {
        let mut space = Space::from_str(EXAMPLE).unwrap();
        let distance = space.distance_estimate((1, 5), (4, 9));
        assert_eq!(distance, 7, "Before expansion");
        space.expand_fast(1);
        let distance = space.distance_estimate((1, 5), (4, 9));
        assert_eq!(distance, 9, "After expansion");
    }

    #[test]
    fn part1_example() {
        let mut space = Space::from_str(EXAMPLE).unwrap();
        space.expand_fast(1);
        assert_eq!(space.total_distance_estimate(), 374);
    }

    #[test]
    fn part1() {
        let mut space = Space::from_str(include_str!("day11.txt")).unwrap();
        space.expand_fast(1);
        assert_eq!(space.total_distance_estimate(), 9918828);
    }

    #[test]
    fn part2_example() {
        let mut space = Space::from_str(EXAMPLE).unwrap();
        space.expand_fast(9);
        assert_eq!(space.total_distance_estimate(), 1030);
        space.expand_fast(90);
        assert_eq!(space.total_distance_estimate(), 8410);
    }

    #[test]
    fn part2() {
        let mut space = Space::from_str(include_str!("day11.txt")).unwrap();
        space.expand_fast(999_999);
        assert_eq!(space.total_distance_estimate(), 692506533832);
    }
}
