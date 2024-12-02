use std::collections::HashSet;

fn count_diffs(adapters: &[u32]) -> (u32, u32) {
    let mut ones = 0;
    let mut threes = 0;
    let mut prev = 0;
    for &adapter in adapters {
        match adapter - prev {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
        prev = adapter;
    }
    (ones, threes + 1)
}

fn is_valid(adapters: &[u32]) -> bool {
    let mut prev = adapters.first().unwrap_or(&0);
    for adapter in &adapters[1..] {
        if adapter - prev > 3 {
            return false;
        }
        prev = adapter;
    }
    true
}

fn split_by_diffs(adapters: &[u32]) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut prev = 0;
    let mut start: usize = 0;
    for (i, &adapter) in adapters.iter().enumerate() {
        if adapter - prev == 3 {
            result.push(adapters[start.saturating_sub(1)..=i].to_vec());
            start = i + 1;
        }
        prev = adapter;
    }
    result
}

fn split_count(adapters: &[u32]) -> usize {
    let extended = std::iter::once(&0)
        .chain(adapters.iter())
        .chain(adapters.last().map(|&x| x + 3).iter())
        .copied()
        .collect::<Vec<_>>();
    let split_groups = split_by_diffs(&extended);
    let mut arrangements = HashSet::new();
    let mut result = 1;
    for group in &split_groups {
        count_arrangements(group, &mut arrangements);
        result *= dbg!(arrangements.len());
        arrangements.clear();
    }
    result
}

fn count_arrangements(adapters: &[u32], arrangements: &mut HashSet<Vec<u32>>) {
    let adapters = adapters.to_vec();
    if is_valid(&adapters) {
        arrangements.insert(adapters.clone());
        for i in 1..adapters.len() - 1 {
            let mut new_adapters = adapters.clone();
            new_adapters.remove(i);
            count_arrangements(&new_adapters, arrangements);
        }
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut adapters: Vec<u32> = input.lines().filter_map(|line| line.parse().ok()).collect();
    adapters.sort_unstable();
    adapters
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_count_diffs() {
        let input = read_to_string("../inputs/2020/day10_example1.txt").unwrap();
        let adapters = parse_input(&input);
        let (ones, threes) = count_diffs(&adapters);
        assert_eq!(ones, 7);
        assert_eq!(threes, 5);
        let input = read_to_string("../inputs/2020/day10_example2.txt").unwrap();
        let adapters = parse_input(&input);
        let (ones, threes) = count_diffs(&adapters);
        assert_eq!(ones, 22);
        assert_eq!(threes, 10);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2020/day10.txt").unwrap();
        let adapters = parse_input(&input);
        let (ones, threes) = count_diffs(&adapters);
        assert_eq!(ones * threes, 1856);
    }

    #[test]
    fn test_count_arrangements() {
        let input = read_to_string("../inputs/2020/day10_example1.txt").unwrap();
        let adapters = parse_input(&input);
        assert_eq!(split_count(&adapters), 8);
        let input = read_to_string("../inputs/2020/day10_example2.txt").unwrap();
        let adapters = parse_input(&input);
        assert_eq!(split_count(&adapters), 19208);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2020/day10.txt").unwrap();
        let adapters = parse_input(&input);
        assert_eq!(split_count(&adapters), 2314037239808);
    }
}
