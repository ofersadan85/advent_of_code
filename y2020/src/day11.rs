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
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err("Invalid character in input"),
        }
    }
}

fn step_rule_1(seats: &Grid<SeatState>) -> Grid<SeatState> {
    let mut new_seats = seats.clone();
    for (old, new) in seats.cells.iter().zip(new_seats.cells.iter_mut()) {
        let occupied = seats.count_neighbors(old.x, old.y, SeatState::Occupied);
        new.state = match old.state {
            SeatState::Empty if occupied == 0 => SeatState::Occupied,
            SeatState::Occupied if occupied >= 4 => SeatState::Empty,
            _ => old.state,
        }
    }
    new_seats
}

fn step_rule_2(seats: &Grid<SeatState>) -> Grid<SeatState> {
    let mut new_seats = seats.clone();
    for (old, new) in seats.cells.iter().zip(new_seats.cells.iter_mut()) {
        let occupied = seats
            .sight_lines_edges(old.x, old.y, &[SeatState::Occupied, SeatState::Empty])
            .iter()
            .filter(|&&s| s == Some(SeatState::Occupied))
            .count();
        new.state = match old.state {
            SeatState::Empty if occupied == 0 => SeatState::Occupied,
            SeatState::Occupied if occupied >= 5 => SeatState::Empty,
            _ => old.state,
        };
    }
    new_seats
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
        let mut seats: Grid<SeatState> = EXAMPLE.parse().unwrap();
        seats.apply_steps_until(step_rule_1, None);
        assert_eq!(seats.count_state(SeatState::Occupied), 37);
    }

    #[test]
    fn test_part_1() {
        let mut seats: Grid<SeatState> = read_to_string("../inputs/2020/day11.txt")
            .unwrap()
            .parse()
            .unwrap();
        seats.apply_steps_until(step_rule_1, None);
        assert_eq!(seats.count_state(SeatState::Occupied), 2441);
    }

    #[test]
    fn test_example_2() {
        let mut seats: Grid<SeatState> = EXAMPLE.parse().unwrap();
        seats.apply_steps_until(step_rule_2, None);
        assert_eq!(seats.count_state(SeatState::Occupied), 26);
    }

    #[test]
    fn test_part_2() {
        let mut seats: Grid<SeatState> = read_to_string("../inputs/2020/day11.txt")
            .unwrap()
            .parse()
            .unwrap();
        seats.apply_steps_until(step_rule_2, None);
        assert_eq!(seats.count_state(SeatState::Occupied), 2190);
    }
}
