use advent_of_code_macros::aoc_tests;
use petgraph::graphmap::UnGraphMap;
use std::collections::BTreeSet;
use tracing::{debug, instrument};

fn parse_input(input: &str) -> UnGraphMap<&str, ()> {
    UnGraphMap::from_edges(input.lines().filter_map(|line| line.split_once('-')))
}

fn common_neighbors<'a>(
    graph: &UnGraphMap<&'a str, ()>,
    group: &BTreeSet<&'a str>,
) -> BTreeSet<BTreeSet<&'a str>> {
    let mut group_copy: Vec<&str> = group.iter().copied().collect();
    let Some(first) = group_copy.pop() else {
        return BTreeSet::new();
    };
    let mut common: BTreeSet<&str> = graph.neighbors(first).collect();
    // common.extend(graph.neighbors(first));  //.filter(|node| !group_copy.contains(node))
    while let Some(node) = group_copy.pop() {
        let neighbors: BTreeSet<&str> = graph.neighbors(node).collect();
        common = common.intersection(&neighbors).copied().collect();
    }
    common
        .into_iter()
        .map(|node| group.iter().copied().chain(std::iter::once(node)).collect())
        .filter(|group: &BTreeSet<&str>| !group.is_empty())
        .collect()
}

fn get_triplets<'a>(graph: &UnGraphMap<&'a str, ()>) -> BTreeSet<BTreeSet<&'a str>> {
    graph
        .all_edges()
        .flat_map(|(a, b, ())| common_neighbors(graph, &BTreeSet::from([a, b])))
        .filter(|group| group.iter().any(|node| node.starts_with('t')))
        .collect()
}

#[instrument(skip_all)]
fn largest_group<'a>(graph: &UnGraphMap<&'a str, ()>) -> BTreeSet<&'a str> {
    let mut largest_groups: BTreeSet<BTreeSet<&str>> = graph
        .all_edges()
        .flat_map(|(a, b, ())| common_neighbors(graph, &BTreeSet::from([a, b])))
        .collect();
    while largest_groups.len() > 1 {
        largest_groups = largest_groups
            .iter()
            .flat_map(|group| common_neighbors(graph, group))
            .collect();
        debug!(
            "{} group(s) of length {}",
            largest_groups.len(),
            largest_groups.first().map_or(0, BTreeSet::len)
        );
    }
    largest_groups.pop_first().unwrap_or_default()
}

fn group_password(group: &BTreeSet<&str>) -> String {
    group.iter().copied().collect::<Vec<&str>>().join(",")
}

#[aoc_tests]
mod tests {
    #[test]
    fn example_1() {
        let input = std::fs::read_to_string("../inputs/2024/day23_example.txt").unwrap();
        let graph = parse_input(&input);
        assert_eq!(get_triplets(&graph).len(), 7);
    }

    #[test]
    fn part_1() {
        let input = read_input();
        let graph = parse_input(&input);
        assert_eq!(get_triplets(&graph).len(), 1200);
    }

    #[test]
    fn example_2() {
        let input = std::fs::read_to_string("../inputs/2024/day23_example.txt").unwrap();
        let graph = parse_input(&input);
        let largest_group = largest_group(&graph);
        assert_eq!(group_password(&largest_group), "co,de,ka,ta");
    }

    #[test]
    fn part_2() {
        let input = read_input();
        let graph = parse_input(&input);
        let largest_group = largest_group(&graph);
        assert_eq!(
            group_password(&largest_group),
            "ag,gh,hh,iv,jx,nq,oc,qm,rb,sm,vm,wu,zr"
        );
    }
}
