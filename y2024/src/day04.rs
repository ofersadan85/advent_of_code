use advent_of_code_common::grid::{Grid, PositionedCell};

fn find_sequences(grid: &Grid<char>) -> usize {
    let search: Vec<char> = "MAS".chars().collect(); // Doesn't include the starting X
    grid.cells
        .iter()
        .filter(|c| c.state == 'X')
        .map(|cell| {
            grid.sight_lines_all(cell.x, cell.y, &['S'])
                .iter()
                .filter(|seq| seq.as_slice() == search)
                .count()
        })
        .sum()
}

fn slice_to_string(slice: &[Option<char>]) -> String {
    slice.iter().map(|c| c.unwrap_or(' ')).collect()
}

fn find_diagonal_sequences(grid: &Grid<char>) -> usize {
    // Cspell:disable-next-line
    let correct = ["MMSS", "MSSM", "SSMM", "SMMS"]; // The results of diagonal slices that are correct
    let diag_str = |cell: &PositionedCell<char>| {
        grid.neighbors_diagonal(cell.x, cell.y)
            .iter()
            .map(|c| c.unwrap_or_default())
            .collect::<String>()
    };
    grid.cells
        .iter()
        .filter(|c| c.state == 'A' && correct.contains(&diag_str(c).as_str()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_example_1() {
        let input = read_to_string("../inputs/2024/day04_example.txt").unwrap();
        let grid: Grid<char> = input.parse().unwrap();
        assert_eq!(find_sequences(&grid), 18);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2024/day04.txt").unwrap();
        let grid: Grid<char> = input.parse().unwrap();
        assert_eq!(find_sequences(&grid), 2517);
    }

    #[test]
    fn test_example_2() {
        let input = read_to_string("../inputs/2024/day04_example.txt").unwrap();
        let grid: Grid<char> = input.parse().unwrap();
        assert_eq!(find_diagonal_sequences(&grid), 9);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day04.txt").unwrap();
        let grid: Grid<char> = input.parse().unwrap();
        assert_eq!(find_diagonal_sequences(&grid), 1960);
    }
}
