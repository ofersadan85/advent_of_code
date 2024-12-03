use std::collections::HashMap;

use advent_of_code_common::grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for SeatState {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(SeatState::Floor),
            'L' => Ok(SeatState::Empty),
            '#' => Ok(SeatState::Occupied),
            _ => Err("Invalid character in input"),
        }
    }
}

impl std::fmt::Display for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SeatState::Floor => '.',
            SeatState::Empty => 'L',
            SeatState::Occupied => '#',
        };
        write!(f, "{c}")
    }
}

fn part_1(mut seats: Grid<SeatState>) -> usize {
    let mut neighbor_map: HashMap<(isize, isize), usize> =
        HashMap::with_capacity(seats.cells.len());
    neighbor_map.clear();
    let mut changed = true;
    while changed {
        neighbor_map = seats
            .cells
            .iter()
            .map(|cell| {
                let occupied = seats.count_neighbors(cell.x, cell.y, SeatState::Occupied);
                ((cell.x, cell.y), occupied)
            })
            .collect();
        changed = seats.apply_step(|cell| {
            let occupied = neighbor_map[&(cell.x, cell.y)];
            match cell.state {
                SeatState::Empty if occupied == 0 => SeatState::Occupied,
                SeatState::Occupied if occupied >= 4 => SeatState::Empty,
                _ => cell.state,
            }
        });
        dbg!(changed);
        println!("{seats}");
    }
    seats.count_state(SeatState::Occupied)
}

fn part_2(mut seats: Grid<SeatState>) -> usize {
    let mut neighbor_map: HashMap<(isize, isize), usize> =
        HashMap::with_capacity(seats.cells.len());
    neighbor_map.clear();
    let mut changed = true;
    while changed {
        neighbor_map = seats
            .cells
            .iter()
            .map(|cell| {
                let occupied = seats
                    .sight_lines_edges(cell.x, cell.y, &[SeatState::Occupied, SeatState::Empty])
                    .iter()
                    .filter(|&&s| s == Some(SeatState::Occupied))
                    .count();
                ((cell.x, cell.y), occupied)
            })
            .collect();
        changed = seats.apply_step(|cell| {
            let occupied = neighbor_map[&(cell.x, cell.y)];
            match cell.state {
                SeatState::Empty if occupied == 0 => SeatState::Occupied,
                SeatState::Occupied if occupied >= 5 => SeatState::Empty,
                _ => cell.state,
            }
        });
    }
    seats.count_state(SeatState::Occupied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: &str = "L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL";

    #[test]
    fn test_example_1() {
        let seats: Grid<SeatState> = EXAMPLE.parse().unwrap();
        assert_eq!(part_1(seats), 37);
    }

    #[test]
    fn test_part_1() {
        let seats: Grid<SeatState> = read_to_string("../inputs/2020/day11.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(part_1(seats), 2441);
    }
    #[test]
    fn test_example_2() {
        let seats: Grid<SeatState> = EXAMPLE.parse().unwrap();
        assert_eq!(part_2(seats), 26);
    }

    #[test]
    fn test_part_2() {
        let seats: Grid<SeatState> = read_to_string("../inputs/2020/day11.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(part_2(seats), 2190);
    }
}
