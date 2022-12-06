use std::collections::HashSet;

const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const PATH: &str = "inputs/day06.txt";

fn input(example: bool) -> Vec<char> {
    if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .chars()
    .collect()
}

fn detect_non_repeats(data: &[char], window_size: usize) -> usize {
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

#[test]
fn example_1() {
    assert_eq!(detect_non_repeats(&input(true), 4), 7);
}

#[test]
fn solution_1() {
    assert_eq!(detect_non_repeats(&input(false), 4), 1155);
}

#[test]
fn example_2() {
    assert_eq!(detect_non_repeats(&input(true), 14), 19);
}

#[test]
fn solution_2() {
    assert_eq!(detect_non_repeats(&input(false), 14), 2789);
}
