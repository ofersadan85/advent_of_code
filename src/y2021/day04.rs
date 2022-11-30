use crate::common::transpose;
#[derive(Debug, Clone, Copy)]
struct BingoCell {
    value: usize,
    selected: bool,
}

#[derive(Debug)]
struct BingoBoard {
    cells: Vec<Vec<BingoCell>>,
    has_won: bool,
}

#[derive(Debug)]

struct BingoGame {
    guesses: Vec<usize>,
    boards: Vec<BingoBoard>,
    winners: Vec<(usize, usize)>,
}

impl BingoGame {
    fn play(&mut self) {
        for (index, guess) in self.guesses.clone().iter().enumerate() {
            for board in &mut self.boards {
                board.mark_guess(*guess);
                if board.is_winner() && !board.has_won {
                    self.winners.push((board.calc_value() * guess, index));
                    board.has_won = true;
                }
            }
        }
    }
}

impl BingoBoard {
    fn from_lines(lines: &[String]) -> Self {
        let cells = lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| BingoCell {
                        value: s.parse().unwrap(),
                        selected: false,
                    })
                    .collect()
            })
            .collect();
        BingoBoard {
            cells,
            has_won: false,
        }
    }

    fn is_winner(&self) -> bool {
        let rows = self
            .cells
            .iter()
            .any(|row| row.iter().all(|cell| cell.selected));
        let cols = transpose(self.cells.clone())
            .iter()
            .any(|row| row.iter().all(|cell| cell.selected));
        rows || cols
    }

    fn mark_guess(&mut self, n: usize) {
        for cell in self.cells.iter_mut().flatten() {
            if cell.value == n {
                cell.selected = true;
            }
        }
    }

    fn calc_value(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| !cell.selected)
            .map(|cell| cell.value)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{get_data, split_lines};
    const PATH: &str = "inputs/2021/day04.txt";
    const EXAMPLE: &str = "
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    fn setup_data(data: &[String]) -> BingoGame {
        let guesses: Vec<usize> = data
            .first()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let boards: Vec<BingoBoard> = data[2..]
            .split(|line| *line.trim() == *"")
            .map(BingoBoard::from_lines)
            .collect();
        BingoGame {
            guesses,
            boards,
            winners: vec![],
        }
    }

    #[test]
    fn example_1() {
        let mut data = setup_data(&split_lines(EXAMPLE));
        data.play();
        let result = data.winners.iter().min_by_key(|w| w.1).unwrap().0;
        assert_eq!(result, 4512);
    }

    #[test]
    fn example_2() {
        let mut data = setup_data(&split_lines(EXAMPLE));
        data.play();
        let result = data.winners.iter().max_by_key(|w| w.1).unwrap().0;
        println!("{:#?}", data.winners);
        assert_eq!(result, 1924);
    }

    #[test]
    fn task_1() {
        let mut data = setup_data(&get_data(PATH).unwrap());
        data.play();
        let result = data.winners.iter().min_by_key(|w| w.1).unwrap().0;
        assert_eq!(result, 29440);
    }

    #[test]
    fn task_2() {
        let mut data = setup_data(&get_data(PATH).unwrap());
        data.play();
        let result = data.winners.iter().max_by_key(|w| w.1).unwrap().0;
        assert_eq!(result, 13884);
    }
}
