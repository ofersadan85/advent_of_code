use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BagInner<'a> {
    amount: usize,
    color: &'a str,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bag<'a> {
    contain: Vec<BagInner<'a>>,
}

fn parse_input(s: &str) -> HashMap<&str, Bag<'_>> {
    s.lines()
        .map(|s| {
            let (color, contain) = s.split_once(" bags contain ").unwrap_or_default();
            let mut contain: Vec<BagInner> = contain
                .split(", ")
                .map(|c| {
                    let (amount, color) = c
                        .trim_end_matches('.')
                        .trim_end_matches(',')
                        .trim_end_matches("bags")
                        .trim_end_matches("bag")
                        .trim()
                        .split_once(' ')
                        .unwrap_or_default();
                    let amount = amount.parse().unwrap_or(0);
                    BagInner { amount, color }
                })
                .collect();
            if contain.iter().map(|b| b.amount).sum::<usize>() == 0 {
                contain = vec![];
            }
            (color, Bag { contain })
        })
        .collect()
}

fn count_gold_options(s: &str) -> usize {
    let mut unchecked: HashMap<&str, Bag> = parse_input(s)
        .into_iter()
        .filter(|(_, b)| b.contain.len() > 0)
        .collect();
    let mut includes_gold = HashSet::new();
    let mut last_result = usize::MAX;
    while unchecked.len() > 0 {
        let mut checked = HashMap::new();
        for (color, bag) in unchecked {
            if includes_gold.contains(color)
                || bag
                    .contain
                    .iter()
                    .find(|b| b.color == "shiny gold" || includes_gold.contains(b.color))
                    .is_some()
            {
                includes_gold.insert(color);
            } else {
                checked.insert(color, bag);
            }
        }
        unchecked = checked;
        if last_result == includes_gold.len() {
            break;
        } else {
            last_result = includes_gold.len();
        }
    }
    includes_gold.len()
}

fn count_total_bags(map: &HashMap<&str, Bag>, color: &str) -> usize {
    let mut total = 0;
    if let Some(bag) = map.get(color) {
        for inner_bag in &bag.contain {
            total += inner_bag.amount * (count_total_bags(map, inner_bag.color) + 1);
        }
    }
    total
}

#[advent_of_code_macros::aoc_tests]
mod tests {
    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    const EXAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn parse_example() {
        let parsed = parse_input(EXAMPLE);
        assert_eq!(parsed.len(), 9);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(count_gold_options(EXAMPLE), 4);
    }

    #[test]
    fn part_1() {
        assert_eq!(count_gold_options(&read_input()), 124);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(count_total_bags(&parse_input(EXAMPLE2), "shiny gold"), 126);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_total_bags(&parse_input(&read_input()), "shiny gold"), 34862);
    }
}
