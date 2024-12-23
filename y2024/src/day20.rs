use advent_of_code_common::grid::{Coords, Grid, Point};
use advent_of_code_macros::aoc_tests;
use std::collections::{HashMap, HashSet};

type Maze = Grid<CellData>;
type Tunnel = (Point, Point);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct CellData {
    state: char,
    cheat: HashSet<Point>,
    distance: isize,
}

impl From<char> for CellData {
    fn from(value: char) -> Self {
        Self {
            state: value,
            cheat: HashSet::new(),
            distance: isize::MAX,
        }
    }
}

fn mark_distances(maze: &mut Maze, start: Point) {
    let mut distance = 0;
    let mut to_visit = vec![start];
    let mut visited = HashSet::new();
    while let Some(p) = to_visit.pop() {
        if let Some(cell) = maze.get_mut(&p) {
            if !visited.insert(p) || cell.data.state == '#' {
                continue;
            }
            cell.data.distance = distance;
            distance += 1;
            to_visit.extend(
                maze.neighbors_orthogonal(&p)
                    .into_iter()
                    .flatten()
                    .filter(|n| n.data.state != '#')
                    .map(Coords::as_point),
            );
        }
    }
}

fn find_tunnels(maze: &Maze, min_saved: isize, max_cheat: isize) -> HashMap<Tunnel, isize> {
    let mut tunnels = HashMap::new();
    for cell in maze.values() {
        if cell.data.state == '#' {
            continue;
        }
        for neighbor in maze
            .neighbors_box_n(&cell, max_cheat)
            .into_iter()
            .flatten()
            .filter(|n| n.data.state != '#')
        {
            let path_distance = (cell.data.distance - neighbor.data.distance).abs();
            let cheat_distance = cell.manhattan_distance(neighbor);
            let saved = (path_distance - cheat_distance).abs();
            if cheat_distance < path_distance && saved >= min_saved && cheat_distance <= max_cheat {
                tunnels
                    .entry((cell.as_point(), neighbor.as_point()))
                    .or_insert(saved);
            }
        }
    }
    tunnels
}

fn best_tunnels(mut maze: Maze, min_saved: isize, max_cheat: isize) -> usize {
    let p = maze
        .values()
        .find(|c| c.data.state == 'S')
        .map(Coords::as_point)
        .unwrap_or_default();
    mark_distances(&mut maze, p);
    find_tunnels(&maze, min_saved, max_cheat).len() / 2
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "###############
                            #...#...#.....#
                            #.#.#.#.#.###.#
                            #S#...#.#.#...#
                            #######.#.#.###
                            #######.#.#...#
                            #######.#.###.#
                            ###..E#...#...#
                            ###.#######.###
                            #...###...#...#
                            #.#####.#.###.#
                            #.#...#.#.#...#
                            #.#.#.#.#.#.###
                            #...#...#...###
                            ###############";

    #[test]
    fn example_1() {
        let maze: Maze = EXAMPLE1.parse().unwrap();
        assert_eq!(best_tunnels(maze, 2, 2), 44);
    }

    #[test]
    fn part_1() {
        let maze: Maze = read_input().parse().unwrap();
        assert_eq!(best_tunnels(maze, 100, 2), 1450);
    }

    #[test]
    fn example_2() {
        let maze: Maze = EXAMPLE1.parse().unwrap();
        assert_eq!(best_tunnels(maze, 50, 20), 285);
    }

    #[test]
    fn part_2() {
        let maze: Maze = read_input().parse().unwrap();
        assert_eq!(best_tunnels(maze, 100, 20), 1015247);
    }
}
