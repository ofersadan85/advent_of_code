use advent_of_code_macros::aoc_tests;
use petgraph::prelude::{Directed, GraphMap, Incoming};
use std::collections::BTreeMap;
use tracing::{debug, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Gate {
    And,
    Or,
    Xor,
    Unknown,
}

impl From<&str> for Gate {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Wire<'a> {
    name: &'a str,
    gate: Gate,
}

impl std::fmt::Display for Wire<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.name, self.gate)
    }
}

impl<'a> Wire<'a> {
    fn new(name: &'a str, gate: impl AsRef<str>) -> Self {
        Self {
            name,
            gate: Gate::from(gate.as_ref()),
        }
    }
}

type WireGraph<'a> = GraphMap<Wire<'a>, u8, Directed>;
type WireValues<'a> = BTreeMap<Wire<'a>, Option<u8>>;

fn parse_graph(input: &str) -> (WireGraph<'_>, WireValues<'_>) {
    let mut graph = GraphMap::new();
    let mut values = BTreeMap::new();
    for line in input.lines() {
        if line.contains(':') {
            let (name, value) = line.split_once(": ").unwrap_or_default();
            let value_wire = Wire::new(name.trim(), value);
            match value.parse::<u8>() {
                Ok(value) => {
                    values.insert(value_wire, Some(value));
                }
                Err(_) => {
                    warn!("Error parsing value: {value} in line {line}");
                }
            }
            graph.add_node(value_wire);
        }
        if line.contains("->") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let gate_wire = Wire::new(parts[4], parts[1]);
            graph.add_node(gate_wire);
        }
    }
    for line in input.lines() {
        if line.contains("->") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let gate_wire = Wire::new(parts[4], parts[1]);
            values.entry(gate_wire).or_insert(None);
            if let Some(left_wire) = graph.nodes().find(|n| n.name == parts[0]) {
                graph.add_edge(left_wire, gate_wire, 1);
                values.entry(left_wire).or_insert(None);
            }
            if let Some(right_wire) = graph.nodes().find(|n| n.name == parts[2]) {
                graph.add_edge(right_wire, gate_wire, 1);
                values.entry(right_wire).or_insert(None);
            }
        }
    }
    (graph, values)
}

fn calc_wire_value<'a>(graph: &WireGraph<'a>, values: &mut WireValues<'a>, wire: Wire<'a>) -> u8 {
    if let Some(Some(value)) = values.get(&wire) {
        return *value;
    }
    debug!("Calculating value for {wire}");
    let inputs = graph.neighbors_directed(wire, Incoming).collect::<Vec<_>>();
    debug_assert!(inputs.len() == 2, "{wire} has {} inputs", inputs.len());
    let left = inputs[0];
    let right = inputs[1];
    let left_value = calc_wire_value(graph, values, left);
    let right_value = calc_wire_value(graph, values, right);
    let value = match wire.gate {
        Gate::And => left_value & right_value,
        Gate::Or => left_value | right_value,
        Gate::Xor => left_value ^ right_value,
        Gate::Unknown => unreachable!(),
    };
    values.insert(wire, Some(value));
    value
}

fn bit_values(input: &str, register: char) -> u64 {
    let (graph, mut values) = parse_graph(input);
    let register_wires = graph
        .nodes()
        .filter(|n| n.name.starts_with(register))
        .collect::<Vec<_>>();
    for wire in register_wires {
        if wire
            .name
            .trim_start_matches(register)
            .parse::<u16>()
            .is_ok()
        {
            calc_wire_value(&graph, &mut values, wire);
        }
    }
    let mut result = 0_u64;
    for (k, v) in &values {
        if k.name.starts_with(register) {
            debug!("{k} has value {v:?}");
            if let Ok(exp) = k.name.trim_start_matches(register).parse::<u16>() {
                result += u64::from(v.expect("register node value")) << exp;
            }
        }
    }
    let g = petgraph::dot::Dot::new(&graph);
    println!("{g}");
    result
}

#[aoc_tests]
mod tests {
    use std::fs::read_to_string;

    const EXAMPLE1: &str = "x00: 1
                            x01: 1
                            x02: 1
                            y00: 0
                            y01: 1
                            y02: 0

                            x00 AND y00 -> z00
                            x01 XOR y01 -> z01
                            x02 OR y02 -> z02";

    #[test]
    fn example_1() {
        assert_eq!(bit_values(EXAMPLE1, 'z'), 4);
        let example2 = read_to_string("../inputs/2024/day24_example.txt").unwrap();
        assert_eq!(bit_values(&example2, 'z'), 2024);
    }

    #[test]
    fn part_1() {
        let input = read_input();
        assert_eq!(bit_values(&input, 'z'), 59336987801432);
    }

    #[test]
    fn reg() {
        let input = read_input();
        assert_eq!(bit_values(&input, 'z'), 59336987801432, "z");
    }
}
