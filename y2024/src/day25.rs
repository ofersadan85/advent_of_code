use advent_of_code_macros::aoc_tests;
use itertools::{iproduct, Itertools};

fn parse_input(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for mut chunk in &input.lines().chunks(8) {
        let is_lock = chunk.next().is_some_and(|line| line == "#####");
        let mut object = [0; 5];
        for line in chunk.take(5) {
            for (j, c) in line.chars().enumerate() {
                object[j] += u8::from(c == '#');
            }
        }
        if is_lock {
            locks.push(object);
        } else {
            keys.push(object);
        }
    }
    (locks, keys)
}

fn count_fits(locks: &[[u8; 5]], keys: &[[u8; 5]]) -> usize {
    iproduct!(locks.iter(), keys.iter())
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 5))
        .count()
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn example_1() {
        let (locks, keys) = parse_input(EXAMPLE1);
        assert_eq!(count_fits(&locks, &keys), 3);
    }

    #[test]
    fn part_1() {
        let input = read_input();
        let (locks, keys) = parse_input(&input);
        assert_eq!(locks.last().unwrap(), &[3, 2, 3, 2, 4]);
        assert_eq!(keys.last().unwrap(), &[1, 4, 0, 3, 0]);
        assert_eq!(count_fits(&locks, &keys), 2770);
    }
}
