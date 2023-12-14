// use anyhow::{anyhow, Result, Context};

use std::fmt::Debug;

use itertools::Itertools;
use tracing::{debug, info, instrument};

pub const EXAMPLE1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

pub const EXAMPLE2: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockShape {
    Round,
    Square,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rock {
    shape: RockShape,
    x: usize,
    y: usize,
}

impl Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.shape {
            RockShape::Round => write!(f, "O ({}, {})", self.x, self.y),
            RockShape::Square => write!(f, "# ({}, {})", self.x, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for RockShape {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Round),
            '#' => Ok(Self::Square),
            _ => Err("Invalid rock"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Rock> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                Some(Rock {
                    shape: RockShape::try_from(c).ok()?,
                    x,
                    y,
                })
            })
        })
        .collect()
}

#[instrument(skip_all, level = "info")]
fn push_rocks(rocks: &[Rock], direction: Direction, max: usize) -> Vec<Rock> {
    use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
    use Direction::{East, North, South, West};
    use RockShape::{Round, Square};
    let mut no_move = vec![];
    let mut to_move = vec![];
    for r in rocks {
        match r.shape {
            Round => to_move.push(*r),
            Square => no_move.push(*r),
        }
    }
    no_move.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
    to_move.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));

    while let Some(current_rock) = to_move.pop() {
        let minmax = no_move
            .iter()
            .chain(to_move.iter())
            .filter(|rock| match direction {
                North => rock.y < current_rock.y && rock.x == current_rock.x,
                East => rock.x > current_rock.x && rock.y == current_rock.y,
                South => rock.y > current_rock.y && rock.x == current_rock.x,
                West => rock.x < current_rock.x && rock.y == current_rock.y,
            })
            .minmax_by_key(|r| match direction {
                North | South => r.y,
                East | West => r.x,
            });
        let target_y = match (minmax, direction) {
            (NoElements, North) => 0,
            (_, East) | (_, West) => current_rock.y,
            (NoElements, South) => max,
            (OneElement(r), North) | (MinMax(_, r), North) => r.y + 1,
            (OneElement(r), South) | (MinMax(r, _), South) => r.y - 1,
        };
        let target_x = match (minmax, direction) {
            (NoElements, East) => max,
            (_, North) | (_, South) => current_rock.x,
            (NoElements, West) => 0,
            (OneElement(r), East) | (MinMax(r, _), East) => r.x - 1,
            (OneElement(r), West) | (MinMax(_, r), West) => r.x + 1,
        };
        let target = Rock {
            shape: current_rock.shape,
            x: target_x,
            y: target_y,
        };
        if current_rock == target {
            debug!("{:?} OK", current_rock);
        } else {
            debug!("{:?} => {:?}", current_rock, target);
        }
        no_move.push(target)
    }
    no_move
}

pub fn push_rocks_north(input: &str) -> usize {
    let len = input.lines().count();
    let rocks = parse_input(input);
    let pushed = push_rocks(&rocks, Direction::North, len);
    pushed
        .iter()
        .filter(|r| r.shape == RockShape::Round)
        .map(|r| len - r.y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::init_tracing;

    #[test]
    fn example1() {
        let input = parse_input(EXAMPLE1);
        let mut result = push_rocks(&input, Direction::North, 100);
        result.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        let mut expected = parse_input(EXAMPLE2);
        expected.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        assert_eq!(result, expected, "{:?} != {:?}", result, expected);
        assert_eq!(push_rocks_north(EXAMPLE1), 136);
    }

    #[test]
    fn part1() {
        init_tracing();
        let input = include_str!("day14.txt");
        assert_eq!(push_rocks_north(input), 0);
    }
}
