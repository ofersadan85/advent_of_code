use crate::default_input_path;
use advent_of_code_common::grid::{Coords, Grid, GridCell, Point};
use advent_of_code_common::Solver;
use advent_of_code_macros::char_enum;
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[char_enum(display)]
#[derive(Hash)]
enum Entity {
    Sheep = 'S',
    Dragon = 'D',
    Shield = '#',
}

impl Entity {
    fn next_moves(&self) -> Vec<(isize, isize)> {
        match self {
            Self::Sheep => vec![(0, 1)],
            Self::Dragon => vec![
                (2, 1),
                (1, 2),
                (-1, 2),
                (-2, 1),
                (-2, -1),
                (-1, -2),
                (1, -2),
                (2, -1),
            ],
            Self::Shield => vec![(0, 0)],
        }
    }
}

type MultiGrid = Grid<HashSet<Entity>>;

fn parse_grid(s: &str) -> MultiGrid {
    let mut grid = MultiGrid {
        x_range: 0..0,
        y_range: 0..0,
        cells: BTreeMap::new(),
    };
    let mut height = 0;
    let mut width = 0;
    for (y, line) in s.lines().enumerate() {
        height += 1;
        let mut line_width = 0;
        for (x, c) in line.chars().enumerate() {
            line_width += 1;
            let x = isize::try_from(x).expect("x in isize range");
            let y = isize::try_from(y).expect("y in isize range");
            let point = (x, y).as_point();
            match Entity::try_from(c) {
                Ok(ty) => {
                    let cell = grid
                        .cells
                        .entry(point)
                        .or_insert_with(|| GridCell::new(&point, HashSet::new()));
                    cell.data.insert(ty);
                }
                Err(_) => {}
            }
        }
        width = width.max(line_width);
    }
    grid.x_range = 0..width;
    grid.y_range = 0..height;
    grid
}

fn step_ty(grid: &mut MultiGrid, ty: Entity, replace: bool) {
    let mut new_points = HashSet::new();
    for (p, cell) in grid.iter_mut() {
        let had_ty = if replace {
            cell.data.remove(&ty)
        } else {
            cell.data.contains(&ty)
        };
        if had_ty {
            new_points.extend(ty.next_moves().iter().map(|mv| Point {
                x: p.x + mv.0,
                y: p.y + mv.1,
            }));
        }
    }
    new_points.retain(|p| grid.is_in_range(p));
    for p in new_points {
        let cell = grid
            .entry(p)
            .or_insert_with(|| GridCell::new(&p, HashSet::new()));
        cell.data.insert(ty);
    }
}

fn remove_eaten(grid: &mut MultiGrid) -> usize {
    grid.iter_mut()
        .map(|(_, cell)| {
            if cell.data.contains(&Entity::Dragon) && !cell.data.contains(&Entity::Shield) {
                if cell.data.remove(&Entity::Sheep) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

struct Part1(usize);
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut grid = parse_grid(input);
        for _ in 0..self.0 {
            step_ty(&mut grid, Entity::Dragon, false);
        }
        remove_eaten(&mut grid)
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part2(usize);
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut grid = parse_grid(input);
        let mut removed_sheep = 0;
        for _ in 0..self.0 {
            step_ty(&mut grid, Entity::Dragon, true);
            removed_sheep += remove_eaten(&mut grid);
            step_ty(&mut grid, Entity::Sheep, true);
            removed_sheep += remove_eaten(&mut grid);
        }
        removed_sheep
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Move {
    ty: Entity,
    from: Point,
    to: Point,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = usize::try_from(self.to.x).unwrap_or(usize::MAX);
        let x = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(x).unwrap_or('?');
        let y = self.to.y + 1;
        use colored::Colorize;
        let ty = match self.ty {
            Entity::Sheep => self.ty.to_string().green(),
            Entity::Dragon => self.ty.to_string().red(),
            Entity::Shield => self.ty.to_string().yellow(),
        };
        write!(f, "{ty}>{x}{y}")
    }
}

fn legal_moves(grid: &MultiGrid, ty: Entity) -> Vec<Move> {
    let mut moves = Vec::new();
    for (p, cell) in grid.iter() {
        if cell.data.contains(&ty) {
            for mv in ty.next_moves() {
                let new_point = Point {
                    x: p.x + mv.0,
                    y: p.y + mv.1,
                };
                if ty == Entity::Dragon && !grid.is_in_range(&new_point) {
                    continue;
                }
                if grid.get(&new_point).is_none_or(|next_cell| {
                    next_cell.data.contains(&Entity::Shield)
                        || !next_cell.data.contains(&Entity::Dragon)
                }) {
                    moves.push(Move {
                        ty,
                        from: *p,
                        to: new_point,
                    });
                }
            }
        }
    }
    println!(
        "Legal moves for {ty}: {}",
        moves.iter().map(|mv| mv.to_string()).join(" ")
    );
    moves
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GameState {
    #[default]
    SheepTurn,
    DragonTurn,
    SheepWon,
    DragonWon,
}

#[derive(Debug, Clone)]
struct GridGame {
    grid: MultiGrid,
    moves: Vec<Move>,
    state: GameState,
}

impl GridGame {
    fn apply_move(&mut self, mv: &Move) -> GameState {
        let from_cell = self.grid.get_mut(&mv.from).expect("from cell exists");
        from_cell.data.remove(&mv.ty);
        let to_cell = self
            .grid
            .entry(mv.to)
            .or_insert_with(|| GridCell::new(&mv.to, HashSet::new()));
        to_cell.data.insert(mv.ty);
        self.moves.push(mv.clone());
        let mut sheep_count = 0;
        for (p, _) in self
            .grid
            .iter()
            .filter(|(_, cell)| cell.data.contains(&Entity::Sheep))
        {
            sheep_count += 1;
            if !self.grid.is_in_range(p) {
                self.state = GameState::SheepWon;
                return self.state;
            }
        }
        self.state = if sheep_count == 0 {
            GameState::DragonWon
        } else {
            match self.state {
                GameState::SheepTurn => GameState::DragonTurn,
                GameState::DragonTurn => GameState::SheepTurn,
                _ => unreachable!("Game already over"),
            }
        };
        self.state
    }

    fn apply_legal_moves(&self) -> Vec<Self> {
        if !matches!(self.state, GameState::SheepTurn | GameState::DragonTurn) {
            return Vec::new();
        }
        let mut new_games = Vec::new();
        let ty = match self.state {
            GameState::SheepTurn => Entity::Sheep,
            GameState::DragonTurn => Entity::Dragon,
            _ => unreachable!("Game already over"),
        };
        for mv in legal_moves(&self.grid, ty) {
            let mut new_game = self.clone();
            new_game.apply_move(&mv);
            new_games.push(new_game);
        }
        new_games
    }

    fn fmt_history(&self) -> String {
        self.moves.iter().map(|mv| mv.to_string()).join(" ")
    }
}

struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let grid = parse_grid(input);
        let game = GridGame {
            grid,
            moves: Vec::new(),
            state: GameState::default(),
        };
        let mut ongoing_games = vec![game];
        let mut dragon_won_games = HashSet::new();
        let mut limit = 30;
        while let Some(mut game) = ongoing_games.pop() {
            limit -= 1;
            if limit == 0 {
                println!("Reached limit of explored games, stopping early");
                break;
            }
            println!("Exploring game with moves: {}", game.fmt_history());
            let next_games = game.apply_legal_moves();
            if next_games.is_empty() {
                println!("No legal moves for {:?}", game.state);
                if game
                    .grid
                    .iter()
                    .any(|(_, cell)| cell.data.contains(&Entity::Sheep))
                {
                    assert_eq!(game.state, GameState::SheepTurn);
                    game.state = GameState::DragonTurn;
                    println!("Sheep still alive, passing turn to Dragon");
                    ongoing_games.push(game);
                } else {
                    println!("Dragon has won!");
                    dragon_won_games.insert(game.moves);
                }
            }
            for game in next_games {
                match game.state {
                    GameState::SheepTurn | GameState::DragonTurn => {
                        ongoing_games.push(game);
                    }
                    GameState::DragonWon => {
                        println!("Dragon won with moves: {}", game.fmt_history());
                        dragon_won_games.insert(game.moves);
                    }
                    GameState::SheepWon => {
                        println!("Sheep won with moves: {}", game.fmt_history());
                    }
                }
            }
        }
        dragon_won_games.len()
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1(3), 0, 27);
        expect_solution!(Part1(4), 1, 164);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2(3), 2, 27);
        expect_solution!(Part2(20), 3, 1734);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 4, 15);
        // expect_solution!(Part3, 5, 36);
        // expect_solution!(Part3, 6, 40528);
    }
}
