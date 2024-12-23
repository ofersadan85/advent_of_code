use advent_of_code_common::grid::{Coords, Direction, Grid, Point};
use advent_of_code_macros::aoc_tests;
use std::collections::{HashMap, HashSet};
use tracing::instrument;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CellData {
    state: char,
    region_id: std::cell::Cell<usize>,
}

impl From<char> for CellData {
    fn from(value: char) -> Self {
        Self {
            state: value,
            region_id: std::cell::Cell::new(0),
        }
    }
}

#[derive(Debug)]
struct Fence {
    point: Point,
    direction: Direction,
}

impl Coords for Fence {
    fn x(&self) -> isize {
        self.point.x()
    }

    fn y(&self) -> isize {
        self.point.y()
    }
}

#[derive(Debug)]
struct Region {
    state: char,
    area: usize,
    perimeter: Vec<Fence>,
}

impl Region {
    fn cost(&self) -> usize {
        self.area * self.perimeter.len()
    }

    fn cost_discount(&self) -> usize {
        let mut h_sides_map: HashMap<(Direction, isize), Vec<&Fence>> = HashMap::new();
        let mut total_sides = 0;
        self.perimeter
            .iter()
            .filter(|fence| {
                fence.direction == Direction::North || fence.direction == Direction::South
            })
            .for_each(|fence| {
                h_sides_map
                    .entry((fence.direction, fence.y()))
                    .or_default()
                    .push(fence);
            });
        for v in h_sides_map.values_mut() {
            v.sort_unstable_by_key(|f| f.x());
            total_sides += v
                .windows(2)
                .filter(|w| w[0].x().abs_diff(w[1].x()) != 1)
                .count()
                + 1;
        }
        let mut v_sides_map: HashMap<(Direction, isize), Vec<&Fence>> = HashMap::new();
        self.perimeter
            .iter()
            .filter(|fence| {
                fence.direction == Direction::East || fence.direction == Direction::West
            })
            .for_each(|fence| {
                v_sides_map
                    .entry((fence.direction, fence.x()))
                    .or_default()
                    .push(fence);
            });
        for v in v_sides_map.values_mut() {
            v.sort_unstable_by_key(|f| f.y());
            total_sides += v
                .windows(2)
                .filter(|w| w[0].y().abs_diff(w[1].y()) != 1)
                .count()
                + 1;
        }
        self.area * total_sides
    }
}

fn next_by_region_id(grid: &Grid<CellData>, region_id: usize) -> Option<Point> {
    grid.values().find_map(|cell| {
        if cell.data.region_id.get() == region_id {
            Some(cell.as_point())
        } else {
            None
        }
    })
}

fn mark_regions(grid: &Grid<CellData>) {
    let mut next_region_id = 0;
    while let Some(p) = next_by_region_id(grid, 0) {
        next_region_id += 1;
        let mut visited: HashSet<Point> = HashSet::new();
        let mut to_visit: Vec<Point> = vec![p];
        while let Some(p) = to_visit.pop() {
            if !visited.insert(p) {
                continue;
            }
            if let Some(cell) = grid.get(&p) {
                cell.data.region_id.set(next_region_id);
                for n in grid.neighbors_orthogonal(&p).into_iter().flatten() {
                    if n.data.state == cell.data.state {
                        to_visit.push(p);
                    }
                }
            }
        }
    }
}

fn map_regions(grid: &Grid<CellData>) -> HashMap<usize, Region> {
    mark_regions(grid);
    let mut result = HashMap::with_capacity(grid.cells.len());
    grid.values().for_each(|cell| {
        let region_id = cell.data.region_id.get();
        let region = result.entry(region_id).or_insert(Region {
            state: cell.data.state,
            area: 0,
            perimeter: Vec::new(),
        });
        region.area += 1;
        for (direction, neighbor) in grid
            .neighbors_orthogonal(&cell.as_point())
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let state = n.map(|n| n.data.state).unwrap_or_default();
                match i {
                    0 => (Direction::North, state),
                    1 => (Direction::East, state),
                    2 => (Direction::South, state),
                    3 => (Direction::West, state),
                    _ => unreachable!("Invalid direction"),
                }
            })
        {
            if neighbor != cell.data.state {
                region.perimeter.push(Fence {
                    point: cell.as_point(),
                    direction,
                });
            }
        }
    });
    result
}

#[instrument(skip(input), level = "info")]
fn part_1(input: &str) -> usize {
    let grid: Grid<CellData> = input.parse().unwrap();
    map_regions(&grid).values().map(Region::cost).sum()
}

#[instrument(skip(input), level = "info")]
fn part_2(input: &str) -> usize {
    let grid: Grid<CellData> = input.parse().unwrap();
    map_regions(&grid).values().map(Region::cost_discount).sum()
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "AAAA\nBBCD\nBBCC\nEEEC";
    const EXAMPLE2: &str = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
    const EXAMPLE3: &str = "RRRRIICCFF
                            RRRRIICCCF
                            VVRRRCCFFF
                            VVRCCCJFFF
                            VVVVCJJCFE
                            VVIVCCJJEE
                            VVIIICJJEE
                            MIIIIIJJEE
                            MIIISIJEEE
                            MMMISSJEEE";
    const EXAMPLE4: &str = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";

    #[test]
    fn test_costs() {
        assert_eq!(part_1(EXAMPLE1), 140, "EXAMPLE1");
        assert_eq!(part_1(EXAMPLE2), 772, "EXAMPLE2");
        assert_eq!(part_1(EXAMPLE3), 1930, "EXAMPLE3");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&read_input()), 1370100);
    }

    #[test]
    fn test_costs_discounted() {
        assert_eq!(part_2(EXAMPLE1), 80, "EXAMPLE1");
        assert_eq!(part_2(EXAMPLE2), 436, "EXAMPLE2");
        assert_eq!(part_2(EXAMPLE4), 236, "EXAMPLE4");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&read_input()), 818286);
    }
}
