use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap_or_default();
            Rule {
                before: before.parse().unwrap_or_default(),
                after: after.parse().unwrap_or_default(),
            }
        })
        .collect();
    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap_or_default())
                .collect()
        })
        .collect();
    (rules, updates)
}

fn is_ordered(update: &[usize], rules: &[Rule]) -> bool {
    let rules: Vec<_> = rules
        .iter()
        .filter(|rule| update.contains(&rule.before) && update.contains(&rule.after))
        .collect();
    let index_map: HashMap<usize, usize> =
        update.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    rules.iter().all(
        |rule| match (index_map.get(&rule.before), index_map.get(&rule.after)) {
            (Some(&before), Some(&after)) => before < after,
            _ => true,
        },
    )
}

fn part_1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update| is_ordered(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn fix_order(update: &[usize], rules: &[Rule]) -> Vec<usize> {
    let rules: Vec<_> = rules
        .iter()
        .filter(|rule| update.contains(&rule.before) && update.contains(&rule.after))
        .collect();
    let mut frequency_map: HashMap<usize, usize> = rules.iter().map(|r| r.after).counts();
    for n in update {
        if !frequency_map.contains_key(n) {
            frequency_map.insert(*n, 0);
        }
    }
    frequency_map
        .iter()
        .sorted_unstable_by_key(|(_, &v)| v)
        .map(|(k, _)| *k)
        .collect()
}

fn part_2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update| !is_ordered(update, &rules))
        .map(|update| fix_order(update, &rules)[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("../inputs/2024/day05_example.txt").unwrap();
        let (rules, updates) = parse_input(&input);
        assert_eq!(rules.len(), 21);
        assert_eq!(updates.len(), 6);
    }

    #[test]
    fn test_is_ordered() {
        let input = read_to_string("../inputs/2024/day05_example.txt").unwrap();
        let (rules, updates) = parse_input(&input);
        assert!(is_ordered(&updates[0], &rules));
        assert!(is_ordered(&updates[1], &rules));
        assert!(is_ordered(&updates[2], &rules));
        assert!(!is_ordered(&updates[3], &rules));
        assert!(!is_ordered(&updates[4], &rules));
        assert!(!is_ordered(&updates[5], &rules));
    }

    #[test]
    fn test_example_1() {
        let input = read_to_string("../inputs/2024/day05_example.txt").unwrap();
        assert_eq!(part_1(&input), 143);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2024/day05.txt").unwrap();
        assert_eq!(part_1(&input), 5374);
    }

    #[test]
    fn test_fix_order() {
        let input = read_to_string("../inputs/2024/day05_example.txt").unwrap();
        let (rules, updates) = parse_input(&input);
        assert_eq!(fix_order(&updates[3], &rules), vec![97, 75, 47, 61, 53]);
        assert_eq!(fix_order(&updates[4], &rules), vec![61, 29, 13]);
        assert_eq!(fix_order(&updates[5], &rules), vec![97, 75, 47, 29, 13]);
    }

    
    #[test]
    fn test_example_2() {
        let input = read_to_string("../inputs/2024/day05_example.txt").unwrap();
        assert_eq!(part_2(&input), 123);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day05.txt").unwrap();
        assert_eq!(part_2(&input), 4260);
    }

}
