use advent_of_code_common::grid::Grid;
use advent_of_code_macros::aoc_tests;
use std::collections::{HashMap, HashSet};

type Maze = Grid<char, Data>;
type Point = (isize, isize);
type Tunnel = (Point, Point);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Data {
    cheat: HashSet<Point>,
    distance: isize,
}

fn mark_distances(maze: &mut Maze, start: Point) {
    let mut distance = 0;
    let mut to_visit = vec![start];
    let mut visited = HashSet::new();
    while let Some((x, y)) = to_visit.pop() {
        if let Some(cell) = maze.get_cell_mut(x, y) {
            if !visited.insert((x, y)) || cell.state == '#' {
                continue;
            }
            cell.data.distance = distance;
            distance += 1;
            to_visit.extend(
                maze.neighbors_orthogonal_cells(x, y)
                    .into_iter()
                    .flatten()
                    .filter(|n| n.state != '#')
                    .map(|n| (n.x, n.y)),
            );
        }
    }
}

fn find_tunnels(maze: &Maze, min_saved: isize, max_cheat: isize) -> HashMap<Tunnel, isize> {
    let mut tunnels = HashMap::new();
    for cell in &maze.cells {
        if cell.state == '#' {
            continue;
        }
        for neighbor in maze
            .neighbors_box_cells_n(cell.x, cell.y, max_cheat)
            .into_iter()
            .flatten()
            .filter(|n| n.state != '#')
        {
            let path_distance = (cell.data.distance - neighbor.data.distance).abs();
            let cheat_distance = (cell.x - neighbor.x).abs() + (cell.y - neighbor.y).abs();
            let saved = (path_distance - cheat_distance).abs();
            if cheat_distance < path_distance && saved >= min_saved && cheat_distance <= max_cheat {
                tunnels
                    .entry(((cell.x, cell.y), (neighbor.x, neighbor.y)))
                    .or_insert(saved);
            }
        }
    }
    tunnels
}

fn best_tunnels(mut maze: Maze, min_saved: isize, max_cheat: isize) -> usize {
    let (x, y) = maze
        .cells
        .iter()
        .find(|c| c.state == 'S')
        .map(|c| (c.x, c.y))
        .unwrap_or_default();
    mark_distances(&mut maze, (x, y));
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
