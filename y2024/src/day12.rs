use advent_of_code_common::grid::{Direction, Grid};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};
use tracing::instrument;

#[derive(Debug)]
struct Fence {
    x: isize,
    y: isize,
    direction: Direction,
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
                    .entry((fence.direction, fence.y))
                    .or_default()
                    .push(fence);
            });
        for v in h_sides_map.values_mut() {
            v.sort_unstable_by_key(|f| f.x);
            total_sides += v
                .windows(2)
                .filter(|w| w[0].x.abs_diff(w[1].x) != 1)
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
                    .entry((fence.direction, fence.x))
                    .or_default()
                    .push(fence);
            });
        for v in v_sides_map.values_mut() {
            v.sort_unstable_by_key(|f| f.y);
            total_sides += v
                .windows(2)
                .filter(|w| w[0].y.abs_diff(w[1].y) != 1)
                .count()
                + 1;
        }
        self.area * total_sides
    }
}

fn next_by_region_id(grid: &Grid<char, Cell<usize>>, region_id: usize) -> Option<(isize, isize)> {
    grid.cells.iter().find_map(|cell| {
        if cell.data.get() == region_id {
            Some((cell.x, cell.y))
        } else {
            None
        }
    })
}

fn mark_regions(grid: &Grid<char, Cell<usize>>) {
    let mut next_region_id = 0;
    while let Some((x, y)) = next_by_region_id(grid, 0) {
        next_region_id += 1;
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut to_visit: Vec<(isize, isize)> = vec![(x, y)];
        while let Some((x, y)) = to_visit.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            if let Some(cell) = grid.get_cell(x, y) {
                cell.data.set(next_region_id);
                for n in grid.neighbors_orthogonal_cells(x, y).into_iter().flatten() {
                    if n.state == cell.state {
                        to_visit.push((n.x, n.y));
                    }
                }
            }
        }
    }
}

fn map_regions(grid: &Grid<char, Cell<usize>>) -> HashMap<usize, Region> {
    mark_regions(grid);
    let mut result = HashMap::with_capacity(grid.cells.len());
    grid.cells.iter().for_each(|cell| {
        let region_id = cell.data.get();
        let region = result.entry(region_id).or_insert(Region {
            state: cell.state,
            area: 0,
            perimeter: Vec::new(),
        });
        region.area += 1;
        for (direction, neighbor) in grid
            .neighbors_orthogonal_cells(cell.x, cell.y)
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let state = n.map(|n| n.state).unwrap_or_default();
                match i {
                    0 => (Direction::North, state),
                    1 => (Direction::East, state),
                    2 => (Direction::South, state),
                    3 => (Direction::West, state),
                    _ => unreachable!("Invalid direction"),
                }
            })
        {
            if neighbor != cell.state {
                region.perimeter.push(Fence {
                    x: cell.x,
                    y: cell.y,
                    direction,
                });
            }
        }
    });
    result
}

#[instrument(skip(input), level = "info")]
fn part_1(input: &str) -> usize {
    let grid: Grid<char, Cell<usize>> = input.parse().unwrap();
    map_regions(&grid).values().map(Region::cost).sum()
}

#[instrument(skip(input), level = "info")]
fn part_2(input: &str) -> usize {
    let grid: Grid<char, Cell<usize>> = input.parse().unwrap();
    map_regions(&grid).values().map(Region::cost_discount).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use test_log::test;

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
        let input = read_to_string("../inputs/2024/day12.txt").unwrap();
        assert_eq!(part_1(&input), 1370100);
    }

    #[test]
    fn test_costs_discounted() {
        assert_eq!(part_2(EXAMPLE1), 80, "EXAMPLE1");
        assert_eq!(part_2(EXAMPLE2), 436, "EXAMPLE2");
        assert_eq!(part_2(EXAMPLE4), 236, "EXAMPLE4");
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day12.txt").unwrap();
        assert_eq!(part_2(&input), 818286);
    }
}
