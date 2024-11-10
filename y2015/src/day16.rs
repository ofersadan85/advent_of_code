use std::collections::HashMap;

use anyhow::anyhow;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SueData {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl std::str::FromStr for SueData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Quick hack to turn the lines into valid json
        let new_line = format!("{{\"{}}}", s.replace(", ", ", \"").replace(": ", "\": "));
        serde_json::from_str(&new_line).map_err(|_| anyhow!("Could not deserialize"))
    }
}

fn parse_input(s: &str) -> HashMap<usize, SueData> {
    s.lines()
        .flat_map(|s| {
            let (id, line) = s.split_once(": ")?;
            let id = id
                .split_whitespace()
                .last()
                .and_then(|id| id.parse().ok())?;
            let data = line.parse::<SueData>().ok()?;
            Some((id, data))
        })
        .collect()
}

fn match_sue(data: SueData, mut map: HashMap<usize, SueData>) -> Option<usize> {
    map.retain(|_k, v| {
        (v.akitas.is_none() || v.akitas == data.akitas)
            && (v.cars.is_none() || v.cars == data.cars)
            && (v.cats.is_none() || v.cats == data.cats)
            && (v.children.is_none() || v.children == data.children)
            && (v.goldfish.is_none() || v.goldfish == data.goldfish)
            && (v.perfumes.is_none() || v.perfumes == data.perfumes)
            && (v.pomeranians.is_none() || v.pomeranians == data.pomeranians)
            && (v.samoyeds.is_none() || v.samoyeds == data.samoyeds)
            && (v.trees.is_none() || v.trees == data.trees)
            && (v.vizslas.is_none() || v.vizslas == data.vizslas)
    });
    assert_eq!(map.len(), 1, "Map length is {}", map.len());
    map.keys().copied().next()
}

fn match_sue_ranged(data: SueData, mut map: HashMap<usize, SueData>) -> Option<usize> {
    map.retain(|_k, v| {
        (v.akitas.is_none() || v.akitas == data.akitas)
            && (v.cars.is_none() || v.cars == data.cars)
            && (v.cats.is_none() || v.cats > data.cats)
            && (v.children.is_none() || v.children == data.children)
            && (v.goldfish.is_none() || v.goldfish < data.goldfish)
            && (v.perfumes.is_none() || v.perfumes == data.perfumes)
            && (v.pomeranians.is_none() || v.pomeranians < data.pomeranians)
            && (v.samoyeds.is_none() || v.samoyeds == data.samoyeds)
            && (v.trees.is_none() || v.trees > data.trees)
            && (v.vizslas.is_none() || v.vizslas == data.vizslas)
    });
    assert_eq!(map.len(), 1, "Map length is {}", map.len());
    map.keys().copied().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse() {
        let input = read_to_string("../inputs/2015/day16.txt").unwrap();
        let sues = parse_input(&input);
        assert_eq!(sues.len(), 500);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day16.txt").unwrap();
        let sues = parse_input(&input);
        let known = SueData {
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        };
        let result = match_sue(known, sues).unwrap();
        assert_eq!(result, 103);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day16.txt").unwrap();
        let sues = parse_input(&input);
        let known = SueData {
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        };
        let result = match_sue_ranged(known, sues).unwrap();
        assert_eq!(result, 405);
    }
}
