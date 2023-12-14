// use anyhow::{anyhow, Result, Context};

use std::fmt::Debug;

use itertools::Itertools;
use tracing::{debug, info, instrument};

pub const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

pub const EXAMPLE_AFTER_NORTH: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

pub const EXAMPLE_AFTER_CYCLE1: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

pub const EXAMPLE_AFTER_CYCLE2: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

pub const EXAMPLE_AFTER_CYCLE3: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

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

#[instrument(skip_all, level = "debug")]
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
    match direction {
        North | West => {
            no_move.sort_unstable_by(|a: &Rock, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
            to_move.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        }
        South | East => {
            no_move.sort_unstable_by(|b: &Rock, a| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
            to_move.sort_unstable_by(|b, a| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        }
    }

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
            (NoElements, South) => max - 1,
            (OneElement(r), North) | (MinMax(_, r), North) => r.y + 1,
            (OneElement(r), South) | (MinMax(r, _), South) => r.y - 1,
        };
        let target_x = match (minmax, direction) {
            (NoElements, East) => max - 1,
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

fn cycle(rocks: &mut Vec<Rock>, width: usize, height: usize) {
    *rocks = push_rocks(rocks, Direction::North, height);
    *rocks = push_rocks(&rocks, Direction::West, width);
    *rocks = push_rocks(&rocks, Direction::South, height);
    *rocks = push_rocks(&rocks, Direction::East, width);
}

#[instrument(skip_all, level = "info")]
pub fn cycle_detect_repeats(input: &str, target: usize) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut rocks = parse_input(input);
    let mut previous_results = vec![];
    let mut repeat_len = usize::MAX;
    let mut repeat1 = 0;
    let mut repeat2 = 0;
    for i in 0.. {
        cycle(&mut rocks, width, height);
        rocks.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        match previous_results.iter().position(|r| *r == rocks) {
            Some(last_repeat) => {
                info!("Found repeat at {i}, last repeat at {last_repeat}");
                repeat_len = i - last_repeat;
                repeat1 = i - repeat_len;
                repeat2 = i;
                break;
            }
            None => {}
        }
        previous_results.push(rocks.clone());
    }
    let slice = &previous_results[repeat1..repeat2];
    info!("slice len: {}", slice.len());
    for (i, r) in slice.iter().enumerate() {
        info!(
            "{i} {}",
            r.iter()
                .filter(|r| r.shape == RockShape::Round)
                .map(|r| height - r.y)
                .sum::<usize>()
        );
    }
    slice[(target - 1) % repeat_len]
        .iter()
        .filter(|r| r.shape == RockShape::Round)
        .map(|r| height - r.y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::init_tracing;

    fn fmt_output(rocks: &[Rock], width: usize, height: usize) -> String {
        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                let c = rocks
                    .iter()
                    .find(|r| r.x == x && r.y == y)
                    .map_or('.', |r| match r.shape {
                        RockShape::Round => 'O',
                        RockShape::Square => '#',
                    });
                output.push(c);
            }
            output.push('\n');
        }
        output
    }

    #[test]
    fn example() {
        let input = parse_input(EXAMPLE);
        let mut result = push_rocks(&input, Direction::North, 100);
        result.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        let mut expected = parse_input(EXAMPLE_AFTER_NORTH);
        expected.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        assert_eq!(result, expected);
        assert_eq!(push_rocks_north(EXAMPLE), 136);
    }

    #[test]
    fn part1() {
        let input = include_str!("day14.txt");
        assert_eq!(push_rocks_north(input), 108144);
    }

    #[test]
    fn cycles() {
        let mut rocks = parse_input(EXAMPLE);
        let width = EXAMPLE.lines().next().unwrap().len();
        let height = EXAMPLE.lines().count();
        cycle(&mut rocks, width, height);
        let output_str = fmt_output(&rocks, width, height);
        assert_eq!(output_str.trim(), EXAMPLE_AFTER_CYCLE1.trim());
        rocks.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        let mut expected = parse_input(EXAMPLE_AFTER_CYCLE1);
        expected.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        assert_eq!(rocks, expected);
        cycle(&mut rocks, width, height);
        let output_str = fmt_output(&rocks, width, height);
        assert_eq!(output_str.trim(), EXAMPLE_AFTER_CYCLE2.trim());
        rocks.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        let mut expected = parse_input(EXAMPLE_AFTER_CYCLE2);
        expected.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        assert_eq!(rocks, expected);
        cycle(&mut rocks, width, height);
        let output_str = fmt_output(&rocks, width, height);
        assert_eq!(output_str.trim(), EXAMPLE_AFTER_CYCLE3.trim());
        rocks.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        let mut expected = parse_input(EXAMPLE_AFTER_CYCLE3);
        expected.sort_unstable_by(|a, b| b.y.cmp(&a.y).then(b.x.cmp(&a.x)));
        assert_eq!(rocks, expected);
    }

    #[test]
    fn part2_example() {
        init_tracing();
        assert_eq!(cycle_detect_repeats(EXAMPLE, 1_000_000_000), 64);
    }

    #[test]
    #[ignore]
    fn part2() {
        let input = include_str!("day14.txt");
        assert_ne!(cycle_detect_repeats(input, 1_000_000_000), 101146);
    }
}
