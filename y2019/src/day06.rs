use advent_of_code_macros::aoc_solver;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .filter_map(|line| line.split_once(')'))
        .map(|(parent, child)| (child, parent))
        .collect()
}

#[aoc_solver(
    suffix = "example",
    input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L",
    expected = 42
)]
#[aoc_solver(file = "inputs/2019/day06.txt", expected = 142497)]
fn part_1(input: &str) -> usize {
    let mut orbit_map = parse(input);
    let mut count_map: HashMap<&str, usize> = HashMap::new();
    count_map.insert("COM", 0);
    let mut to_remove = Vec::new();
    while !orbit_map.is_empty() {
        to_remove.clear();
        for (&child, &parent) in &orbit_map {
            if let Some(&parent_count) = count_map.get(parent) {
                count_map.insert(child, parent_count + 1);
                to_remove.push(child);
            }
        }
        for child in &to_remove {
            orbit_map.remove(child);
        }
    }
    count_map.values().sum()
}

#[aoc_solver(
    suffix = "example",
    input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
    expected = Some(4)
)]
#[aoc_solver(file = "inputs/2019/day06.txt", expected = Some(301))]
fn part_2(input: &str) -> Option<usize> {
    let orbit_map = parse(input);
    let mut you_path = Vec::new();
    let mut current = "YOU";
    while let Some(&parent) = orbit_map.get(current) {
        you_path.push(parent);
        current = parent;
    }
    let mut san_path = Vec::new();
    let mut current = "SAN";
    while let Some(&parent) = orbit_map.get(current) {
        san_path.push(parent);
        current = parent;
    }
    for (i, &you_parent) in you_path.iter().enumerate() {
        if let Some(j) = san_path
            .iter()
            .position(|&san_parent| san_parent == you_parent)
        {
            return Some(i + j);
        }
    }
    None
}
