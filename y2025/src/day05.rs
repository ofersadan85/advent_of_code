use advent_of_code_common::Solver;
use itertools::Itertools;
use std::ops::RangeInclusive;

fn ranges_cross<T: Ord>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

fn unify_ranges<T: Ord + Copy>(ranges: &mut Vec<RangeInclusive<T>>) {
    let mut i = 0;
    while i < ranges.len() {
        let mut changed = false;
        let mut j = i + 1;
        while j < ranges.len() {
            if ranges_cross(&ranges[i], &ranges[j]) {
                let start = *ranges[i].start().min(ranges[j].start());
                let end = *ranges[i].end().max(ranges[j].end());
                ranges[i] = start..=end;
                ranges.remove(j);
                changed = true;
            } else {
                j += 1;
            }
        }
        if !changed {
            i += 1;
        }
    }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut lines = input.lines();
    let mut ranges = lines
        .take_while_ref(|s| !s.is_empty())
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap_or_default();
            let start = start.parse().expect("valid start");
            let end = end.parse().expect("valid end");
            start..=end
        })
        .collect();
    unify_ranges(&mut ranges);
    let numbers = lines.filter_map(|s| s.parse().ok()).collect();
    (ranges, numbers)
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut count = 0;
        let (ranges, numbers) = parse_input(input);
        for n in numbers {
            for r in &ranges {
                if r.contains(&n) {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}
struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let (ranges, _) = parse_input(input);
        ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 3);
        expect_solution!(Part1, 1, 848);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 14);
        expect_solution!(Part2, 1, 334714395325710);
    }
}
