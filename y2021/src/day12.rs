use advent_of_code_common::{
    algorithms::graph::{Graph, Undirected},
    file::split_lines_trim,
};
use itertools::Itertools;

const PATH: &str = "../inputs/2021/day12.txt";
const EXAMPLE_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
const EXAMPLE_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
const EXAMPLE_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

fn visit_caves(graph: &Undirected, allow_double: bool) -> usize {
    let mut known_paths = vec![vec!["start".to_string()]];
    let mut result: Vec<Vec<String>> = vec![];
    while let Some(path) = known_paths.pop() {
        if path.last().unwrap() == "end" {
            result.push(path);
            continue;
        }
        for (next_node, _) in graph.neighbors(path.last().unwrap()).unwrap() {
            let cave_count = path.iter().counts();
            let top_small_cave = path
                .iter()
                .filter(|s| *s != "start" && *s != "end" && s.to_lowercase() == **s)
                .map(|s| cave_count.get(s).unwrap().to_owned())
                .max()
                .unwrap_or(0);
            let is_uppercase = &next_node.to_uppercase() == next_node;

            let allow_double = *cave_count.get(next_node).unwrap_or(&0) < 2
                && top_small_cave < 2
                && allow_double
                && next_node != "start";

            if is_uppercase || !path.contains(next_node) || allow_double {
                let mut new_path = path.clone();
                new_path.extend(vec![next_node.clone()]);
                known_paths.push(new_path);
            }
        }
    }
    result.len()
}

fn setup_data(data: &str) -> Undirected {
    let data = split_lines_trim(data);
    let mut graph = Undirected::new();
    for row in data {
        let mut split = row.split('-');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        graph.add_edge((first, second, 1));
    }
    graph
}

#[test]
fn example_1() {
    let data = setup_data(EXAMPLE_1);
    let result = visit_caves(&data, false);
    assert_eq!(result, 10);
    let data = setup_data(EXAMPLE_2);
    let result = visit_caves(&data, false);
    assert_eq!(result, 19);
    let data = setup_data(EXAMPLE_3);
    let result = visit_caves(&data, false);
    assert_eq!(result, 226);
}

#[test]
fn example_2() {
    let data = setup_data(EXAMPLE_1);
    let result = visit_caves(&data, true);
    assert_eq!(result, 36);
    let data = setup_data(EXAMPLE_2);
    let result = visit_caves(&data, true);
    assert_eq!(result, 103);
    let data = setup_data(EXAMPLE_3);
    let result = visit_caves(&data, true);
    assert_eq!(result, 3509);
}

#[test]
fn task_1() {
    let data = setup_data(&std::fs::read_to_string(PATH).unwrap());
    let result: usize = visit_caves(&data, false);
    assert_eq!(result, 3369);
}

#[test]
#[ignore = "Takes too long"]
fn task_2() {
    let data = setup_data(&std::fs::read_to_string(PATH).unwrap());
    let result: usize = visit_caves(&data, true);
    assert_eq!(result, 85883);
}
