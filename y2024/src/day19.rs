use advent_of_code_macros::aoc_tests;
use std::collections::{BTreeMap, BTreeSet};
use tracing::{debug_span, field::Empty};

fn count_possible<'a>(
    options: &[&'a str],
    line: &'a str,
    cache: &mut BTreeMap<&'a str, usize>,
) -> usize {
    if let Some(count) = cache.get(line) {
        if !options.contains(&line) {
            return *count;
        }
    }
    if line.is_empty() {
        return 1;
    }
    let local_options: BTreeSet<&str> = options
        .iter()
        .copied()
        .filter(|o| line.starts_with(o))
        .collect();
    let mut total = 0;
    for option in local_options {
        let (_, after) = line.split_at(option.len());
        let after_count = count_possible(options, after, cache);
        cache.insert(after, after_count);
        total += after_count;
    }
    cache.insert(line, total);
    total
}

fn count_possible_total(input: &str, count: bool) -> usize {
    let mut lines = input.lines();
    let options: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let mut cache = options.iter().map(|&o| (o, 1)).collect();
    let mut total = 0;
    let mut sum = 0;
    for line in lines {
        let span = debug_span!("line", line = Empty, c = Empty);
        let _enter = span.enter();
        if line.is_empty() {
            continue;
        }
        let c = count_possible(&options, line.trim(), &mut cache);
        span.record("c", &c);
        if c > 0 {
            total += 1;
        }
        sum += c;
    }
    if count {
        total
    } else {
        sum
    }
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "r, wr, b, g, bwu, rb, gb, br

                            brwrr
                            bggr
                            gbbr
                            rrbgbr
                            ubwu
                            bwurrg
                            brgr
                            bbrgwb";

    #[test]
    fn example_1() {
        assert_eq!(count_possible_total(EXAMPLE1, true), 6);
    }

    #[test]
    fn part_1() {
        assert_eq!(count_possible_total(&read_input(), true), 260);
    }
    #[test]
    fn example_2() {
        assert_eq!(count_possible_total(EXAMPLE1, false), 16);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_possible_total(&read_input(), false), 639963796864990);
    }
}
