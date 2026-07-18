use crate::intcode::{IntcodeComputer, State};
use advent_of_code_common::coords::Point;
use advent_of_code_macros::aoc_solver;
use colored::Colorize;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl TryFrom<isize> for TileKind {
    type Error = ();
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(()),
        }
    }
}

struct Tile {
    x: isize,
    y: isize,
    kind: TileKind,
}

#[derive(Clone)]
struct Game {
    computer: IntcodeComputer,
    tiles: BTreeMap<Point, TileKind>,
    score: isize,
    ball: Option<Point>,
    paddle: Option<Point>,
    dimensions: Option<(isize, isize, isize, isize)>, // (min_x, max_x, min_y, max_y)
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.dimensions.unwrap_or_default();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 0 && y == 0 {
                    write!(f, "┏")?;
                } else if x == 0 && y == max_y {
                    write!(f, "┗")?;
                } else if x == max_x && y == 0 {
                    write!(f, "┓")?;
                } else if x == max_x && y == max_y {
                    write!(f, "┛")?;
                } else if x == 0 || x == max_x {
                    write!(f, "┃")?;
                } else if y == 0 || y == max_y {
                    write!(f, "━")?;
                } else {
                    let tile = self
                        .tiles
                        .get(&Point { x, y })
                        .copied()
                        .unwrap_or(TileKind::Empty);
                    let symbol = match tile {
                        TileKind::Empty => " ".normal(),
                        TileKind::Block => "▇".green(),
                        TileKind::Paddle => "▂".blue().bold(),
                        TileKind::Ball => "o".red().bold(),
                        TileKind::Wall => unreachable!("Handled by the border drawing code above"),
                    };
                    write!(f, "{symbol}")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Score: {} | DIFF: {:?}",
            self.score,
            self.get_diff_input()
        )
    }
}

impl Game {
    fn new(program: &str, with_quarters: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let mut computer: IntcodeComputer = program.parse()?;
        if with_quarters {
            computer.memory.insert(0, 2); // Set quarters
        }
        Ok(Self {
            computer,
            tiles: BTreeMap::new(),
            score: 0,
            ball: None,
            paddle: None,
            dimensions: None,
        })
    }

    fn blocks_remaining(&self) -> usize {
        self.tiles
            .values()
            .filter(|&&kind| kind == TileKind::Block)
            .count()
    }

    fn get_diff_input(&self) -> isize {
        self.ball.unwrap_or_default().x - self.paddle.unwrap_or_default().x
    }

    fn run(&mut self) {
        self.computer.run();
        let output_values = &self.computer.output;
        for (x, y, tile_value) in output_values.iter().copied().tuples() {
            if x == -1 && y == 0 {
                self.score = tile_value;
                continue;
            }
            let kind = TileKind::try_from(tile_value).expect("invalid tile value");
            let point = Point { x, y };
            match kind {
                TileKind::Ball => self.ball = Some(point),
                TileKind::Paddle => self.paddle = Some(point),
                _ => {}
            }
            self.tiles.insert(point, kind);
        }
        if self.dimensions.is_none() {
            // We can assume the dimensions only need to be set once
            let (min_x, max_x) = self
                .tiles
                .keys()
                .map(|k| k.x)
                .minmax()
                .into_option()
                .unwrap_or((isize::MIN, isize::MAX));
            let (min_y, max_y) = self
                .tiles
                .keys()
                .map(|k| k.y)
                .minmax()
                .into_option()
                .unwrap_or((isize::MIN, isize::MAX));
            self.dimensions = Some((min_x, max_x, min_y, max_y));
        }
    }
}

#[aoc_solver(file = "inputs/2019/day13.txt", expected = 268)]
fn part_1(input: &str) -> usize {
    let mut game = Game::new(input, false).expect("valid game");
    game.run();
    assert_eq!(game.computer.state, State::Halted);
    game.tiles
        .values()
        .filter(|&&kind| kind == TileKind::Block)
        .count()
}

#[aoc_solver(file = "inputs/2019/day13.txt", expected = 13989)]
fn part_2(input: &str) -> isize {
    let mut game = Game::new(input, true).expect("valid game");
    game.run();
    while game.blocks_remaining() > 0 {
        let diff = game.get_diff_input().signum();
        assert!(diff.abs() <= 1, "DIFF should be -1, 0, or 1");
        game.computer.queue_input(diff);
        game.run();
    }
    assert_eq!(game.computer.state, State::Halted);
    game.score
}
