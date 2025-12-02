use advent_of_code_common::Solver;
use std::{borrow::Cow, collections::HashSet};

fn invalids_in_range(start: &str, end: &str, repeats: usize) -> impl Iterator<Item = usize> {
    let start: Cow<str> = if start.len().is_multiple_of(repeats) {
        Cow::Borrowed(start)
    } else {
        let len = u32::try_from(start.len()).expect("valid length");
        let x = format!("{}", 10_usize.pow(len));
        Cow::Owned(x)
    };
    let real_start: usize = start.parse().expect("start number");
    let new_start: usize = start
        .chars()
        .take(start.len() / repeats)
        .collect::<String>()
        .parse()
        .expect("known number");
    let end: usize = end.parse().expect("end number");
    println!("start: {new_start}, end: {end}");
    (new_start..)
        .map(move |n| n.to_string().repeat(repeats).parse().expect("known number"))
        .take_while(move |&n| n <= end)
        .filter(move |&n| n >= real_start)
}

fn all_invalids(start: &str, end: &str) -> HashSet<usize> {
    (2..=end.len())
        .flat_map(move |repeats| invalids_in_range(start, end, repeats))
        .collect()
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input
        .split(',')
        .map(|range| range.trim().split_once('-').expect("valid range"))
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_input(input)
            .flat_map(|(start, end)| invalids_in_range(start, end, 2))
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_input(input)
            .map(|(start, end)| all_invalids(start, end).iter().sum::<usize>())
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    fn test_invalids(start: usize, end: usize, repeats: usize) -> Vec<usize> {
        invalids_in_range(&start.to_string(), &end.to_string(), repeats).collect()
    }

    #[test]
    fn test_invalids_2() {
        assert_eq!(test_invalids(11, 22, 2), vec![11, 22]);
        assert_eq!(test_invalids(95, 115, 2), vec![99]);
        assert_eq!(test_invalids(998, 1012, 2), vec![1010]);
        assert_eq!(test_invalids(1188511880, 1188511890, 2), vec![1188511885]);
        assert_eq!(test_invalids(222220, 222224, 2), vec![222222]);
        assert_eq!(test_invalids(1698522, 1698528, 2), vec![]);
        assert_eq!(test_invalids(446443, 446449, 2), vec![446446]);
        assert_eq!(test_invalids(38593856, 38593862, 2), vec![38593859]);
        assert_eq!(test_invalids(565653, 565659, 2), vec![]);
        assert_eq!(test_invalids(824824821, 824824827, 2), vec![]);
        assert_eq!(test_invalids(2121212118, 2121212124, 2), vec![]);
    }

    #[test]
    fn test_invalids_3() {
        assert_eq!(test_invalids(11, 22, 3), vec![]);
        assert_eq!(test_invalids(95, 115, 3), vec![111]);
        assert_eq!(test_invalids(998, 1012, 3), vec![999]);
        assert_eq!(test_invalids(1188511880, 1188511890, 3), vec![]);
        assert_eq!(test_invalids(222220, 222224, 3), vec![222222]);
        assert_eq!(test_invalids(1698522, 1698528, 3), vec![]);
        assert_eq!(test_invalids(446443, 446449, 3), vec![]);
        assert_eq!(test_invalids(38593856, 38593862, 3), vec![]);
        assert_eq!(test_invalids(565653, 565659, 3), vec![565656]);
        assert_eq!(test_invalids(824824821, 824824827, 3), vec![824824824]);
        assert_eq!(test_invalids(2121212118, 2121212124, 3), vec![]);
    }

    #[test]
    fn test_all_invalids() {
        assert_eq!(test_invalids(222220, 222224, 2), vec![222222]);
        assert_eq!(test_invalids(222220, 222224, 3), vec![222222]);
        assert_eq!(test_invalids(222220, 222224, 6), vec![222222]);
        assert_eq!(
            all_invalids("222220", "222224"),
            vec![222222].into_iter().collect(),
            "222222 Should be invalid as repeats of 2, 3 and 6, but should only appear once"
        );
        assert_eq!(
            all_invalids("2121212118", "2121212124"),
            vec![2121212121].into_iter().collect(),
            "2121212121 Should be invalid as repeats of 5"
        );
    }

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 1227775554);
        expect_solution!(Part1, 1, 40398804950);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 4174379265);
        expect_solution!(Part2, 1, 65794984339);
    }
}
