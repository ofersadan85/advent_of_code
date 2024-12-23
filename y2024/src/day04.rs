use advent_of_code_common::grid::{Grid, GridCell};
use advent_of_code_macros::aoc_tests;

fn find_sequences(grid: &Grid) -> usize {
    let search = "MAS"; // Doesn't include the starting X
    grid.cells
        .values()
        .filter(|c| c.data == 'X')
        .map(|cell| {
            grid.sight_lines_all(cell, &['S'])
                .into_iter()
                .filter(|seq| seq.iter().map(|c| c.data).collect::<String>() == search)
                .count()
        })
        .sum()
}

fn slice_to_string(slice: &[Option<char>]) -> String {
    slice.iter().map(|c| c.unwrap_or(' ')).collect()
}

fn find_diagonal_sequences(grid: &Grid) -> usize {
    // Cspell:disable-next-line
    let correct = ["MMSS", "MSSM", "SSMM", "SMMS"]; // The results of diagonal slices that are correct
    let diag_str = |cell: &GridCell| {
        correct.contains(
            &grid
                .neighbors_diagonal(cell)
                .iter()
                .map(|c| c.map(|c| c.data).unwrap_or_default())
                .collect::<String>()
                .as_str(),
        )
    };
    grid.cells
        .values()
        .filter(|c| c.data == 'A' && diag_str(c))
        .count()
}

#[aoc_tests]
mod tests {
    #[test]
    fn example_1() {
        let grid: Grid = std::fs::read_to_string("../inputs/2024/day04_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(find_sequences(&grid), 18);
    }

    #[test]
    fn part_1() {
        let grid: Grid = read_input().parse().unwrap();
        assert_eq!(find_sequences(&grid), 2517);
    }

    #[test]
    fn example_2() {
        let grid: Grid = std::fs::read_to_string("../inputs/2024/day04_example.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(find_diagonal_sequences(&grid), 9);
    }

    #[test]
    fn part_2() {
        let grid: Grid = read_input().parse().unwrap();
        assert_eq!(find_diagonal_sequences(&grid), 1960);
    }
}
