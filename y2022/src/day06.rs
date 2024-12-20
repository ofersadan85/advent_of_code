use advent_of_code_macros::aoc_tests;
use std::collections::HashSet;

fn detect_non_repeats(input: &str, window_size: usize) -> usize {
    let data: Vec<char> = input.chars().collect();
    let mut index = window_size;
    for window in data.windows(window_size) {
        let set: HashSet<char> = window.iter().copied().collect();
        if set.len() == window_size {
            break;
        }
        index += 1;
    }
    index
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn example_1() {
        assert_eq!(detect_non_repeats(EXAMPLE, 4), 7);
    }

    #[test]
    fn solution_1() {
        assert_eq!(detect_non_repeats(&read_input(), 4), 1155);
    }

    #[test]
    fn example_2() {
        assert_eq!(detect_non_repeats(EXAMPLE, 14), 19);
    }

    #[test]
    fn solution_2() {
        assert_eq!(detect_non_repeats(&read_input(), 14), 2789);
    }
}
