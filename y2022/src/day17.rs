// Building A Tetris game in Rust
// https://adventofcode.com/2022/day/17

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
            Pixel::Empty => write!(f, "."),
            Pixel::Moving => write!(f, "M"),
            Pixel::Full => write!(f, "#"),
        }
    }
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
    score: usize,
    shape_index: usize,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .board
            .iter()
            .map(|row| row.iter().map(Pixel::to_string).collect::<String>())
            .join("\n");
        writeln!(f, "{result}")?;
        writeln!(f, "Score: {}", self.score)
    }
}

impl Game {
    fn new(width: usize) -> Self {
        Self {
            board: VecDeque::new(),
            width,
            score: 0,
            shape_index: 0,
        }
    }

    fn add_shape(&mut self) {
        let mut shape = Shape::from_int(self.shape_index % 5).to_vec(self.width);
        self.shape_index += 1;
        self.board.push_front(vec![Pixel::Empty; self.width]);
        while !shape.is_empty() {
            let row = shape.pop().unwrap();
            self.board.push_front(row);
        }
        println!("{self}");
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

    fn prune_bottom(&mut self) {
        let full_row = self
            .board
            .iter()
            .find_position(|row| row.iter().all(|p| *p == Pixel::Full));
        if let Some((at, _)) = full_row {
            // Split and drop the bottom rows, and add the number of rows dropped to the score
            self.score += self.board.split_off(at).len();
        }
    }
}

#[test]
fn test_game() {
    let mut game = Game::new(7);
    game.add_shape();
    game.add_shape();
    game.add_shape();
    game.add_shape();
    game.add_shape();
    game.add_shape();
    game.add_shape();
    todo!()
}
