use itertools::Itertools;
use petgraph::{algo::astar, graph::DiGraph};
use std::collections::HashMap;

fn parse_graph(input: &str) -> DiGraph<String, ()> {
    let mut count = 0;
    let mut graph = DiGraph::new();
    let bugs = ["ANT", "BUG"];
    for line in input.lines() {
        let (start, end) = line.split_once(':').expect("Invalid input");
        if bugs.contains(&start) {
            continue;
        }
        let start_node = graph
            .node_indices()
            .find(|i| graph[*i] == start)
            .unwrap_or_else(|| graph.add_node(start.to_string()));
        for node in end.split(',') {
            if bugs.contains(&node) {
                continue;
            }
            let node = if node == "@" {
                count += 1;
                format!("{node}{count}")
            } else {
                node.to_string()
            };
            let end_node = graph
                .node_indices()
                .find(|i| graph[*i] == node)
                .unwrap_or_else(|| graph.add_node(node));
            graph.add_edge(start_node, end_node, ());
        }
    }
    graph
}

fn get_paths(graph: &DiGraph<String, ()>, char_limit: usize) -> HashMap<usize, Vec<String>> {
    let start = graph
        .node_indices()
        .find(|i| graph[*i] == "RR")
        .expect("Start not found");
    let mut paths: HashMap<usize, Vec<String>> = HashMap::new();
    for node in graph.node_indices() {
        if graph[node].starts_with('@') {
            let (len, path) =
                astar(&graph, start, |n| n == node, |_| 1, |_| 1).expect("No path found");
            let path_str = path
                .iter()
                .map(|i| {
                    let node = &graph[*i];
                    if node.starts_with('@') {
                        "@".to_string()
                    } else {
                        node.chars().take(char_limit).collect()
                    }
                })
                .join("");
            paths.entry(len).or_default().push(path_str);
        }
    }
    paths
}

fn get_unique_path_length(input: &str, char_limit: usize) -> String {
    let graph = parse_graph(input);
    let paths = get_paths(&graph, char_limit);
    let path = paths
        .into_iter()
        .find(|(_, v)| v.len() == 1)
        .expect("No unique path found")
        .1
        .into_iter()
        .next()
        .expect("Known to have a path");
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part_1() {
        let input = "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@";
        assert_eq!(get_unique_path_length(input, 10), "RRB@");
        let input = read_to_string("../inputs/everybody/quest05part1.txt").expect("input file");
        assert_eq!(get_unique_path_length(&input, 10), "RRXKQXBSGRMW@");
    }

    #[test]
    fn part_2() {
        let input = read_to_string("../inputs/everybody/quest05part2.txt").expect("input file");
        assert_eq!(get_unique_path_length(&input, 1), "RKRMFGBPJB@");
    }

    #[test]
    fn part_3() {
        let input = read_to_string("../inputs/everybody/quest05part3.txt").expect("input file");
        assert_eq!(get_unique_path_length(&input, 1), "RRPMJQRFPDTZ@");
    }
}
