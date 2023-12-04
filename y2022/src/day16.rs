use anyhow::{Context, Result};
use std::collections::HashMap;

use regex::Regex;

const PATH: &str = "inputs/day16.txt";
const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    distance: HashMap<String, usize>,
    open_time: Option<usize>,
}

impl TryFrom<&str> for Valve {
    type Error = &'static str;

    fn try_from(row: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); .*valves? (\w+.*)")
            .map_err(|_| "invalid regex")?;
        let captures = re.captures(row).ok_or("invalid row")?;
        let name = captures.get(1).ok_or("invalid row")?.as_str().to_string();
        let flow_rate = captures
            .get(2)
            .ok_or("invalid row")?
            .as_str()
            .parse()
            .map_err(|_| "invalid row")?;
        let distance = captures
            .get(3)
            .ok_or("invalid row")?
            .as_str()
            .split(", ")
            .map(|c| (c.to_string(), 1))
            .collect();
        Ok(Self {
            name,
            flow_rate,
            distance,
            open_time: None,
        })
    }
}

fn input(example: bool) -> Result<HashMap<String, Valve>> {
    let text = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).context("Error reading input file")?
    };
    let map = text
        .lines()
        .filter_map(|row| Valve::try_from(row).ok())
        .map(|valve| (valve.name.clone(), valve))
        .collect();
    Ok(map)
}

#[test]
fn test_input() {
    let input = input(true).unwrap();
    assert_eq!(input.len(), 10);
    println!("{:?}", input["AA"]);
    assert_eq!(input["AA"].name, "AA");
    assert_eq!(input["AA"].flow_rate, 0);
    assert_eq!(input["AA"].distance["DD"], 1);
    assert_eq!(input["AA"].distance["II"], 1);
    assert_eq!(input["AA"].distance["BB"], 1);
    assert_eq!(input["JJ"].name, "JJ");
    assert_eq!(input["JJ"].flow_rate, 21);
    assert_eq!(input["JJ"].distance["II"], 1);
}
