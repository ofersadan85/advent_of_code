use advent_of_code_common::grid::Grid;

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

fn find_diagonal_sequences(grid: &Grid<char>) -> usize {
    grid.cells
        .iter()
        .filter(|c| c.state == 'A')
        .map(|cell| match grid.neighbors_diagonal(cell.x, cell.y) {
            [Some('M'), Some('M'), Some('S'), Some('S')]
            | [Some('M'), Some('S'), Some('S'), Some('M')]
            | [Some('S'), Some('S'), Some('M'), Some('M')]
            | [Some('S'), Some('M'), Some('M'), Some('S')] => 1,
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
