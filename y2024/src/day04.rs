use advent_of_code_common::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Letter {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            _ => Err(()),
        }
    }
}

fn find_sequences(grid: &Grid<Letter>) -> usize {
    let search = [Letter::M, Letter::A, Letter::S]; // Doesn't include the starting X
    grid.cells
        .iter()
        .filter(|c| c.state == Letter::X)
        .map(|cell| {
            grid.sight_lines_all(cell.x, cell.y, &[Letter::S])
                .iter()
                .filter(|seq| seq.as_slice() == search)
                .count()
        })
        .sum()
}

fn find_diagonal_sequences(grid: &Grid<Letter>) -> usize {
    grid.cells
        .iter()
        .filter(|c| c.state == Letter::A)
        .map(|cell| match grid.neighbors_diagonal(cell.x, cell.y) {
            [Some(Letter::M), Some(Letter::M), Some(Letter::S), Some(Letter::S)]
            | [Some(Letter::M), Some(Letter::S), Some(Letter::S), Some(Letter::M)]
            | [Some(Letter::S), Some(Letter::S), Some(Letter::M), Some(Letter::M)]
            | [Some(Letter::S), Some(Letter::M), Some(Letter::M), Some(Letter::S)] => 1,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_example_1() {
        let input = read_to_string("../inputs/2024/day04_example.txt").unwrap();
        let grid: Grid<Letter> = input.parse().unwrap();
        assert_eq!(find_sequences(&grid), 18);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2024/day04.txt").unwrap();
        let grid: Grid<Letter> = input.parse().unwrap();
        assert_eq!(find_sequences(&grid), 2517);
    }

    #[test]
    fn test_example_2() {
        let input = read_to_string("../inputs/2024/day04_example.txt").unwrap();
        let grid: Grid<Letter> = input.parse().unwrap();
        assert_eq!(find_diagonal_sequences(&grid), 9);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day04.txt").unwrap();
        let grid: Grid<Letter> = input.parse().unwrap();
        assert_eq!(find_diagonal_sequences(&grid), 1960);
    }
}
