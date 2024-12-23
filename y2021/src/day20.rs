use advent_of_code_common::grid::{Coords, Grid};
use advent_of_code_macros::{aoc_tests, char_enum};
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

#[char_enum(display)]
#[derive(Default, Hash)]
enum Tile {
    #[default]
    Empty = '.',
    Full = '#',
}

impl From<Tile> for u16 {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Empty => 0,
            Tile::Full => 1,
        }
    }
}

/// Supports up to 9 bits, otherwise use a dynamic Vec<u16> instead of a fixed array.
const EXP: [u16; 9] = [256, 128, 64, 32, 16, 8, 4, 2, 1];

fn build_index_map() -> HashMap<Vec<Tile>, u16> {
    let b = [Tile::Empty, Tile::Full];
    iproduct!(b, b, b, b, b, b, b, b, b)
        .map(|v| vec![v.0, v.1, v.2, v.3, v.4, v.5, v.6, v.7, v.8])
        .map(|v| {
            (
                v.clone(),
                v.iter()
                    .zip(EXP.iter())
                    .map(|(a, exp)| u16::from(*a) * exp)
                    .sum(),
            )
        })
        .collect()
}

type IndexMap = HashMap<Vec<Tile>, Tile>;

fn parse_input(input: &str) -> Result<(Grid<Tile>, IndexMap), Box<dyn std::error::Error>> {
    let mut lines = input.lines();
    let first: Vec<char> = lines.next().unwrap_or_default().chars().collect();
    let index_map = build_index_map();
    let next_step_map = index_map
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                first
                    .get(usize::from(*v))
                    .and_then(|c: &char| Tile::try_from(*c).ok())
                    .expect("valid tile index"),
            )
        })
        .collect();
    lines.next(); // Skip the next (empty) line.
    let grid: Grid<Tile> = lines.join("\n").parse()?;
    Ok((grid, next_step_map))
}

fn enhance(grid: &mut Grid<Tile>, next_step_map: &IndexMap, n: usize) {
    const EXPAND: isize = 3; // After each step, expand the grid by `EXPAND` cells in each direction.
    const TRIM: isize = 4; // After each second step, trim `TRIM` cells from each side.
                           // These are the lowest values that work for the given input, to maximize performance.
                           // For other inputs you may need to adjust these values slightly.
    for step in 0..n {
        let mut new_grid = grid.clone();
        for _ in 0..EXPAND {
            new_grid.expand();
        }
        let mut to_visit: Vec<_> = new_grid.keys().copied().collect();
        while let Some(point) = to_visit.pop() {
            let neighbors: Vec<Tile> = grid
                .neighbors_box(&point)
                .iter()
                .map(|t| t.map(|c| c.data).unwrap_or_default())
                .collect();
            new_grid.get_mut(&point).expect("existing cell").data =
                next_step_map.get(&neighbors).copied().unwrap_or_default();
        }
        *grid = new_grid;
        if step % 2 == 1 {
            let x_range = grid.x_range.start + TRIM..grid.x_range.end - TRIM;
            let y_range = grid.y_range.start + TRIM..grid.y_range.end - TRIM;
            grid.retain(|k, _| x_range.contains(&k.x()) && y_range.contains(&k.y()));
            grid.x_range = x_range;
            grid.y_range = y_range;
        }
        // eprintln!("{grid}\n{:?},{:?}", grid.x_range, grid.y_range);
    }
}

#[aoc_tests]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn example_1() {
        let input = read_to_string("../inputs/2021/day20_example.txt").unwrap();
        let (mut grid, next_step_map) = parse_input(&input).unwrap();
        enhance(&mut grid, &next_step_map, 2);
        let count = grid.values().filter(|c| c.data == Tile::Full).count();
        assert_eq!(count, 35);
    }

    #[test]
    fn part_1() {
        let input = read_input();
        let (mut grid, next_step_map) = parse_input(&input).unwrap();
        enhance(&mut grid, &next_step_map, 2);
        let count = grid.values().filter(|c| c.data == Tile::Full).count();
        assert_eq!(count, 5682);
    }

    #[test]
    fn example_2() {
        let input = read_to_string("../inputs/2021/day20_example.txt").unwrap();
        let (mut grid, next_step_map) = parse_input(&input).unwrap();
        enhance(&mut grid, &next_step_map, 50);
        let count = grid.values().filter(|c| c.data == Tile::Full).count();
        assert_eq!(count, 3351);
    }

    #[test]
    fn part_2() {
        let input = read_input();
        let (mut grid, next_step_map) = parse_input(&input).unwrap();
        enhance(&mut grid, &next_step_map, 50);
        let count = grid.values().filter(|c| c.data == Tile::Full).count();
        assert_eq!(count, 17628);
    }
}
