use std::collections::HashMap;

use itertools::{iproduct, Itertools};

type CharPair = (char, char);
type PairCounter = HashMap<CharPair, i64>;
type ChainRules = HashMap<CharPair, (CharPair, CharPair)>;

fn char_pair_counter() -> PairCounter {
    iproduct!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
    )
    .map(|(a, b)| ((a, b), 0))
    .collect()
}

fn apply_rules(pairs: &mut PairCounter, rules: &ChainRules) {
    let mut operations: HashMap<(char, char), i64> = HashMap::new();
    for (rule, (pair1, pair2)) in rules {
        let value = pairs.get(rule).unwrap().to_owned();
        operations
            .entry(*rule)
            .and_modify(|v| *v -= value)
            .or_insert(-value);
        operations
            .entry(*pair1)
            .and_modify(|v| *v += value)
            .or_insert(value);
        operations
            .entry(*pair2)
            .and_modify(|v| *v += value)
            .or_insert(value);
    }
    for (key, value) in operations {
        *pairs.get_mut(&key).unwrap() += value;
    }
}

fn count_chars(pairs: &PairCounter, edges: CharPair) -> i64 {
    let mut result: HashMap<char, i64> = HashMap::new();
    for ((char1, char2), count) in pairs.iter().filter(|(_, &v)| v > 0) {
        result
            .entry(*char1)
            .and_modify(|v| *v += count)
            .or_insert(*count);
        result
            .entry(*char2)
            .and_modify(|v| *v += count)
            .or_insert(*count);
    }
    result.entry(edges.0).and_modify(|v| *v += 1);
    result.entry(edges.1).and_modify(|v| *v += 1);

    let (min, max) = result
        .iter()
        .minmax_by_key(|(_, &v)| v)
        .into_option()
        .unwrap();
    (max.1 - min.1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::{get_data, split_lines};
    const PATH: &str = "inputs/day14.txt";
    const EXAMPLE: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    fn setup_data(data: &[String]) -> (PairCounter, ChainRules, CharPair) {
        let mut counter = char_pair_counter();
        let v: Vec<char> = data[0].chars().collect();
        for window in v.windows(2) {
            *counter.get_mut(&(window[0], window[1])).unwrap() += 1;
        }

        let arrow = " -> ";
        let rules = data
            .iter()
            .filter(|s| s.contains(arrow))
            .map(|s| {
                let mut split = s.split(arrow);
                let mut left = split.next().unwrap().chars();
                let a = left.next().unwrap();
                let b = left.next().unwrap();
                let c = split.next().unwrap().chars().next().unwrap();
                ((a, b), ((a, c), (c, b)))
            })
            .collect();

        let edges = (v.first().unwrap().to_owned(), v.last().unwrap().to_owned());

        (counter, rules, edges)
    }

    #[test]
    fn example_1() {
        let mut data = setup_data(&split_lines(EXAMPLE));
        for _ in 0..10 {
            apply_rules(&mut data.0, &data.1);
        }
        let result = count_chars(&data.0, data.2);
        assert_eq!(result, 1588);
    }

    #[test]
    fn example_2() {
        let mut data = setup_data(&split_lines(EXAMPLE));
        for _ in 0..40 {
            apply_rules(&mut data.0, &data.1);
        }
        let result = count_chars(&data.0, data.2);
        assert_eq!(result, 2_188_189_693_529);
    }

    #[test]
    fn task_1() {
        let mut data = setup_data(&get_data(PATH).unwrap());
        for _ in 0..10 {
            apply_rules(&mut data.0, &data.1);
        }
        let result = count_chars(&data.0, data.2);
        assert_eq!(result, 2587);
    }

    #[test]
    fn task_2() {
        let mut data = setup_data(&get_data(PATH).unwrap());
        for _ in 0..40 {
            apply_rules(&mut data.0, &data.1);
        }
        let result = count_chars(&data.0, data.2);
        assert_eq!(result, 3_318_837_563_123);
    }
}
