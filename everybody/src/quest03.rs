use advent_of_code_common::grid::{Coords, Grid};

#[derive(Debug, Default, Clone)]
struct Cell {
    dig_site: bool,
    edge_distance: std::cell::Cell<usize>,
}

impl Cell {
    fn new(dig_site: bool) -> Self {
        Self {
            dig_site,
            ..Default::default()
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        let dig_site = matches!(value, '#');
        Self::new(dig_site)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.dig_site {
            self.edge_distance.get().to_string()
        } else {
            ".".to_string()
        };
        write!(f, "{c}")
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.dig_site == other.dig_site
    }
}

fn calc_slopes(grid: &Grid<Cell>, diagonal: bool) -> usize {
    let mut to_visit = grid
        .cells
        .iter()
        .filter(|(_, c)| c.data.dig_site)
        .map(|(p, _)| p)
        .copied()
        .collect::<Vec<_>>();
    let mut edges = Vec::with_capacity(to_visit.len());

    // Mark edges
    while let Some(current) = to_visit.pop() {
        let neighbors = if diagonal {
            grid.neighbors(&current)
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        } else {
            grid.neighbors_orthogonal(&current)
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        };
        if neighbors.into_iter().any(|c| !c.data.dig_site) {
            grid.get(&current)
                .expect("Known cell")
                .data
                .edge_distance
                .set(1);
            edges.push(current);
        }
    }
    std::mem::swap(&mut to_visit, &mut edges);
    let mut visit_next = edges;
    visit_next.clear(); // Reuse the vector to avoid reallocation

    // Flood fill other distances
    loop {
        while let Some(current) = to_visit.pop() {
            let neighbors = if diagonal {
                grid.neighbors(&current)
                    .into_iter()
                    .flatten()
                    .filter(|c| c.data.dig_site && c.data.edge_distance.get() == 0)
                    .collect::<Vec<_>>()
            } else {
                grid.neighbors_orthogonal(&current)
                    .into_iter()
                    .flatten()
                    .filter(|c| c.data.dig_site && c.data.edge_distance.get() == 0)
                    .collect::<Vec<_>>()
            };

            for neighbor in neighbors {
                neighbor.data.edge_distance.set(
                    grid.get(&current)
                        .expect("Known cell")
                        .data
                        .edge_distance
                        .get()
                        + 1,
                );
                visit_next.push(neighbor.as_point());
            }
        }
        if visit_next.is_empty() {
            break;
        }
        std::mem::swap(&mut to_visit, &mut visit_next);
    }

    grid.values()
        .filter(|c| c.data.dig_site)
        .map(|c| c.data.edge_distance.get())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::trim_lines;
    use std::fs::read_to_string;
    const EXAMPLE1: &str = "..........
                            ..###.##..
                            ...####...
                            ..######..
                            ..######..
                            ...####...
                            ..........";

    #[test]
    fn part1() {
        let grid: Grid<Cell> = trim_lines(EXAMPLE1).parse().expect("parsing input");
        assert_eq!(calc_slopes(&grid, false), 35);
        let input = read_to_string("../inputs/everybody/quest03part1.txt").expect("input file");
        let grid: Grid<Cell> = input.parse().expect("parsing input");
        assert_eq!(calc_slopes(&grid, false), 128);
    }

    #[test]
    fn part2() {
        let input = read_to_string("../inputs/everybody/quest03part2.txt").expect("input file");
        let grid: Grid<Cell> = input.parse().expect("parsing input");
        assert_eq!(calc_slopes(&grid, false), 2618);
    }

    #[test]
    fn part3() {
        let input = read_to_string("../inputs/everybody/quest03part3.txt").expect("input file");
        let mut grid: Grid<Cell> = input.parse().expect("parsing input");
        grid.expand();
        assert_eq!(calc_slopes(&grid, true), 9860);
    }
}
