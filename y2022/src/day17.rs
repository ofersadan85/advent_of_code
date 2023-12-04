use anyhow::{Context, Result};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
const PATH: &str = "inputs/day17.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Empty,
    Moving,
    Full,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Moving => write!(f, "@"),
            Self::Full => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
}

enum Shape {
    Row,
    Plus,
    LRev,
    Col,
    Square,
}

impl Shape {
    fn from_int(i: usize) -> Self {
        match i {
            0 => Self::Row,
            1 => Self::Plus,
            2 => Self::LRev,
            3 => Self::Col,
            4 => Self::Square,
            _ => panic!("Invalid shape"),
        }
    }

    fn to_vec(&self, width: usize) -> Vec<Vec<Pixel>> {
        match self {
            Self::Row => {
                let mut p = vec![vec![Pixel::Empty; width]];
                (2..6).for_each(|i| p[0][i] = Pixel::Moving);
                p
            }
            Self::Plus => {
                let mut p = vec![vec![Pixel::Empty; width]; 3];
                (2..5).for_each(|i| p[1][i] = Pixel::Moving);
                (0..3).for_each(|i| p[i][3] = Pixel::Moving);
                p
            }
            Self::LRev => {
                let mut p = vec![vec![Pixel::Empty; width]; 3];
                (2..5).for_each(|i| p[2][i] = Pixel::Moving);
                (0..3).for_each(|i| p[i][4] = Pixel::Moving);
                p
            }
            Self::Col => {
                let mut p = vec![vec![Pixel::Empty; width]; 4];
                (0..4).for_each(|i| p[i][2] = Pixel::Moving);
                p
            }
            Self::Square => {
                let mut p = vec![vec![Pixel::Empty; width]; 2];
                (2..4).for_each(|i| p[0][i] = Pixel::Moving);
                (2..4).for_each(|i| p[1][i] = Pixel::Moving);
                p
            }
        }
    }
}

struct Game {
    board: VecDeque<Vec<Pixel>>,
    width: usize,
    shape_index: usize,
    top_of_mover: usize,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .board
            .iter()
            .map(|row| row.iter().map(Pixel::to_string).collect::<String>())
            .join("\n");
        writeln!(f, "{result}")
    }
}

impl Game {
    const fn new(width: usize) -> Self {
        Self {
            board: VecDeque::new(),
            width,
            shape_index: 0,
            top_of_mover: 0,
        }
    }

    fn add_shape(&mut self) {
        let mut shape = Shape::from_int(self.shape_index % 5).to_vec(self.width);
        self.shape_index += 1;
        for _ in 0..3 {
            self.board.push_front(vec![Pixel::Empty; self.width]);
        }
        while let Some(row) = shape.pop() {
            self.board.push_front(row);
        }
        self.top_of_mover = 0;
    }

    fn prune_top(&mut self) {
        loop {
            let top_empty = self.board[0].iter().all(|p| *p == Pixel::Empty);
            if top_empty {
                self.board.pop_front();
            } else {
                break;
            }
        }
    }

    fn get_mover_coordinates(&self) -> Vec<(usize, usize)> {
        let lookup_range = self.top_of_mover..(self.top_of_mover + 4).min(self.board.len());
        self.board
            .range(lookup_range)
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, p)| {
                    if *p == Pixel::Moving {
                        Some((x, y + self.top_of_mover))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn move_step(&mut self, dir: Direction) -> bool {
        use Direction::{Down, Left, Right};
        use Pixel::{Empty, Full, Moving};
        let mover_coords = self.get_mover_coordinates();
        if mover_coords.is_empty() {
            return false;
        }
        let (min_x, max_x, max_y) = mover_coords
            .iter()
            .fold((self.width, 0, 0), |(min_x, max_x, max_y), (x, y)| {
                (min_x.min(*x), max_x.max(*x), max_y.max(*y))
            });

        let can_move = match dir {
            Left => {
                min_x > 0
                    && mover_coords
                        .iter()
                        .all(|(x, y)| self.board[*y][*x - 1] != Full)
            }
            Right => {
                max_x + 1 < self.width
                    && mover_coords
                        .iter()
                        .all(|(x, y)| self.board[*y][*x + 1] != Full)
            }
            Down => {
                max_y + 1 < self.board.len()
                    && mover_coords
                        .iter()
                        .all(|(x, y)| self.board[*y + 1][*x] != Full)
            }
        };
        if can_move {
            mover_coords
                .iter()
                .for_each(|&(x, y)| self.board[y][x] = Empty);
            mover_coords.iter().for_each(|&(x, y)| match dir {
                Left => self.board[y][x - 1] = Moving,
                Right => self.board[y][x + 1] = Moving,
                Down => self.board[y + 1][x] = Moving,
            });
            if dir == Down {
                self.top_of_mover += 1;
            }
        } else if dir == Down {
            mover_coords
                .iter()
                .for_each(|&(x, y)| self.board[y][x] = Full);
            self.top_of_mover = 0;
            self.prune_top();
        }
        can_move
    }

    fn game_loop(&mut self, directions: &str, shapes: usize) {
        let mut chars = directions.chars().cycle();
        for _ in 0..shapes {
            self.add_shape();
            loop {
                match chars.next().unwrap_or(' ') {
                    '<' => self.move_step(Direction::Left),
                    '>' => self.move_step(Direction::Right),
                    _ => false,
                };
                let moved = self.move_step(Direction::Down);
                if !moved {
                    break;
                }
            }
        }
    }
}

fn input(example: bool) -> Result<String> {
    if example {
        Ok(EXAMPLE.to_string())
    } else {
        std::fs::read_to_string(PATH).context("Error reading input file")
    }
}

fn play(directions: &str, shapes: usize) -> usize {
    let mut game = Game::new(7);
    game.game_loop(directions, shapes);
    game.board.len()
}

fn part_2(directions: &str, r_cycle: usize, shapes: usize) -> usize {
    let r_start = r_cycle * 10;
    let start_len = play(directions, r_start);
    let r_len = play(directions, r_start + r_cycle) - start_len;
    let repeats = (shapes - r_start) / r_cycle;
    let diff = play(directions, shapes - (repeats * r_cycle));
    diff + repeats * r_len
}

#[test]
fn example_1() {
    assert_eq!(play(&input(true).unwrap(), 2022), 3068);
}

#[test]
fn task_1() {
    assert_eq!(play(&input(false).unwrap(), 2022), 3188);
}

#[test]
fn example_2() {
    let r_cycle = 35; // Measured the repeats by hand //todo: automate
    let shapes = 1_000_000_000_000;
    assert_eq!(part_2(&input(true).unwrap(), r_cycle, shapes), 1_514_285_714_288);
}

#[test]
fn task_2() {
    let r_cycle = 2778; // Measured the repeats by hand //todo: automate
    let shapes = 1_000_000_000_000;
    assert_eq!(part_2(&input(false).unwrap(), r_cycle, shapes), 0);
    todo!("This is not the right answer, but it's the right order of magnitude. I'm not sure what's wrong. :(")
}
