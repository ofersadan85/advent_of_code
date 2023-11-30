use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn brute_force_path(s: &str, min: bool) -> usize {
    let regex = Regex::new(r"^(?P<from>\w+) to (?P<to>\w+) = (?P<dist>\d+)$").expect("regex");
    let mut graph: HashMap<(&str, &str), usize> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();
    for line in s.lines() {
        let line = line.trim();
        let caps = regex.captures(line).expect("regex");
        let from = caps.name("from").expect("from").as_str();
        let to = caps.name("to").expect("to").as_str();
        let dist = caps.name("dist").expect("dist").as_str();
        graph.insert((from, to), dist.parse().expect("parse"));
        graph.insert((to, from), dist.parse().expect("parse"));
        cities.insert(from);
        cities.insert(to);
    }
    // let cities = cities.into_iter().collect_vec();
    let distances = cities.iter().permutations(cities.len()).map(|path| {
        path.iter()
            .tuple_windows()
            .map(|(&&from, &&to)| graph[&(from, to)])
            .sum::<usize>()
    });
    if min {
        distances.min().expect("min")
    } else {
        distances.max().expect("max")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_min() {
        let input = "London to Dublin = 464
                            London to Belfast = 518
                            Dublin to Belfast = 141";
        let result = brute_force_path(input, true);
        assert_eq!(result, 605);
    }

    #[test]
    fn test_example_max() {
        let input = "London to Dublin = 464
                            London to Belfast = 518
                            Dublin to Belfast = 141";
        let result = brute_force_path(input, false);
        assert_eq!(result, 982);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("day09.txt");
        let result = brute_force_path(input, true);
        assert_eq!(result, 141);
    }
    #[test]
    fn test_part_2() {
        let input = include_str!("day09.txt");
        let result = brute_force_path(input, false);
        assert_eq!(result, 736);
    }
}
