// use std::collections::HashMap;

// use itertools::iproduct;

// type CharPair = (char, char);
// type PairCounter = HashMap<CharPair, usize>;
// type ChainRules = HashMap<CharPair, (CharPair, CharPair)>;

// fn char_pair_counter() -> PairCounter {
//     iproduct!(
//         "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars(),
//         "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
//     )
//     .map(|(a, b)| ((a, b), 0))
//     .collect()
// }

// fn apply_rules(pairs: &mut PairCounter, rules: ChainRules) {
//     let mut new_pairs = pairs.clone();
//     for (rule, (pair1, pair2)) in rules {
//         let value = pairs.get(&rule).unwrap().to_owned();
//         *new_pairs.get_mut(&rule).unwrap() -= value;
//         *new_pairs.get_mut(&pair1).unwrap() += value;
//         *new_pairs.get_mut(&pair2).unwrap() += value;
//     }
//     pairs = &mut new_pairs;
// }

// #[cfg(test)]
// mod tests {
//     use itertools::Itertools;

//     use super::*;
//     use crate::common::*;
//     const PATH: &str = "inputs/aoc_2021_14.txt";
//     const EXAMPLE: &str = "NNCB

//     CH -> B
//     HH -> N
//     CB -> H
//     NH -> C
//     HB -> C
//     HC -> B
//     HN -> C
//     NN -> C
//     BH -> H
//     NC -> B
//     NB -> B
//     BN -> B
//     BB -> N
//     BC -> B
//     CC -> N
//     CN -> C";

//     fn setup_data(data: Vec<String>) -> (PairCounter, ChainRules, CharPair) {
//         let mut counter = char_pair_counter();
//         let v: Vec<char> = data[0].chars().collect();
//         for window in v.windows(2) {
//             *counter.get_mut(&(window[0], window[1])).unwrap() += 1;
//         }

//         let arrow = " -> ";
//         let rules = data
//             .iter()
//             .filter(|s| s.contains(arrow))
//             .map(|s| {
//                 let mut split = s.split(arrow);
//                 let mut left = split.next().unwrap().chars();
//                 let a = left.next().unwrap();
//                 let b = left.next().unwrap();
//                 let c = split.next().unwrap().chars().next().unwrap();
//                 ((a, b), ((a, c), (c, b)))
//             })
//             .collect();

//         let edges = (v.first().unwrap().to_owned(), v.last().unwrap().to_owned());

//         (counter, rules, edges)
//     }

//     #[test]
//     fn example_1() {
//         let (mut pairs, rules) = setup_data(split_lines(EXAMPLE));
//         // apply_rules(&mut pairs, rules);
//         let result = pairs.iter().filter(|(_, &v)| v > 0).collect_vec();
//         println!("{:?}", result);
//         assert_eq!(323, 123);
//     }

//     // #[test]
//     // fn example_2() {
//     //     let data = setup_data(split_lines(EXAMPLE));
//     //     let result: usize = calc_fuel_increasing(data);
//     //     assert_eq!(result, 168);
//     // }

//     // #[test]
//     // fn task_1() {
//     //     let data = setup_data(get_data(PATH).unwrap());
//     //     let result: usize = calc_fuel(data);
//     //     assert_eq!(result, 356958);
//     // }

//     // #[test]
//     // fn task_2() {
//     //     let data = setup_data(get_data(PATH).unwrap());
//     //     let result: usize = calc_fuel_increasing(data);
//     //     assert_eq!(result, 105461913);
//     // }
// }
