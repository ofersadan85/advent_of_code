#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Wire {
    Valued { name: String, value: u16 },
    Gated { name: String, gate: String },
}

impl Wire {
    fn evaluate(&self) -> Self {
        self.gate().and_then(|v| v.parse::<u16>().ok()).map_or_else(
            || self.clone(),
            |value| Self::Valued {
                name: self.name().to_string(),
                value,
            },
        )
    }

    const fn has_value(&self) -> bool {
        match self {
            Self::Valued { .. } => true,
            Self::Gated { .. } => false,
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::Valued { name, .. } | Self::Gated { name, .. } => name,
        }
    }

    fn gate(&self) -> Option<&str> {
        match self {
            Self::Gated { gate, .. } => Some(gate),
            Self::Valued { .. } => None,
        }
    }

    const fn value(&self) -> Option<u16> {
        match self {
            Self::Valued { value, .. } => Some(*value),
            Self::Gated { .. } => None,
        }
    }
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        let mut parts = s.split(" -> ");
        let gate = parts.next().expect("Wire format (gate)").to_string();
        let name = parts.next().expect("Wire format (name)").to_string();
        Self::Gated { name, gate }
    }
}

struct Circuit {
    wires: HashMap<String, Wire>,
}

impl Circuit {
    fn new(input: &str) -> Self {
        let mut wires = HashMap::new();
        for line in input.lines() {
            let wire = Wire::from(line.trim());
            wires.insert(wire.name().to_string(), wire);
        }
        wires.values_mut().for_each(|wire| match wire {
            Wire::Valued { .. } => {}
            Wire::Gated { .. } => {
                *wire = wire.evaluate();
            }
        });
        Self { wires }
    }

    fn count(&self) -> (usize, usize) {
        let mut count_valued = 0;
        let mut count_gated = 0;
        self.wires.values().for_each(|wire| match wire {
            Wire::Valued { .. } => count_valued += 1,
            Wire::Gated { .. } => count_gated += 1,
        });
        (count_valued, count_gated)
    }

    fn parse_value(&self, value: &str) -> Option<u16> {
        value
            .parse::<u16>()
            .ok()
            .or_else(|| self.wires.get(value)?.value())
    }

    fn parse_gate(&self, gate: &str) -> Option<u16> {
        let mut parts = gate.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some(a), Some("AND"), Some(b)) => {
                let a = self.parse_value(a)?;
                let b = self.parse_value(b)?;
                Some(a & b)
            }
            (Some(a), Some("OR"), Some(b)) => {
                let a = self.parse_value(a)?;
                let b = self.parse_value(b)?;
                Some(a | b)
            }
            (Some(a), Some("LSHIFT"), Some(b)) => {
                let a = self.parse_value(a)?;
                let b = self.parse_value(b)?;
                Some(a << b)
            }
            (Some(a), Some("RSHIFT"), Some(b)) => {
                let a = self.parse_value(a)?;
                let b = self.parse_value(b)?;
                Some(a >> b)
            }
            (Some("NOT"), Some(a), None) => {
                let a = self.parse_value(a)?;
                Some(!a)
            }
            (Some(a), None, None) => self.wires.get(a)?.value(),
            _ => unreachable!("Invalid gate: {}", gate),
        }
    }

    fn reduce_entropy_once(&mut self) {
        let mut new_wires = HashMap::new();
        for (name, wire) in &self.wires {
            if wire.has_value() {
                new_wires.insert(name.to_string(), wire.clone());
            } else {
                let gate = wire.gate().expect("Wire has no gate and no value");
                if let Some(value) = self.parse_gate(gate) {
                    new_wires.insert(
                        name.to_string(),
                        Wire::Valued {
                            name: name.to_string(),
                            value,
                        },
                    );
                } else {
                    new_wires.insert(name.to_string(), wire.clone());
                }
            }
        }
        self.wires = new_wires;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_entropy_once() {
        let input = "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";
        let mut circuit = Circuit::new(input);
        assert_eq!(circuit.count(), (2, 6));
        circuit.reduce_entropy_once();
        assert_eq!(circuit.count(), (8, 0));
    }

    #[test]
    fn test_reduce_entropy() {
        let input = "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";
        let mut circuit = Circuit::new(input);
        assert_eq!(circuit.count(), (2, 6));
        let mut count = circuit.count();
        while count.1 > 0 {
            circuit.reduce_entropy_once();
            count = circuit.count();
        }
        assert_eq!(circuit.count(), (8, 0));
    }

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/2015/day07.txt");
        let mut circuit = Circuit::new(input);
        while !circuit.wires.get("a").expect("Wire a").has_value() {
            circuit.reduce_entropy_once();
        }
        assert_eq!(circuit.wires.get("a").expect("Wire a").value(), Some(16076));
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/2015/day07.txt");
        let mut circuit = Circuit::new(input);
        while !circuit.wires.get("a").expect("Wire a").has_value() {
            circuit.reduce_entropy_once();
        }
        let saved_a = circuit
            .wires
            .get("a")
            .expect("Wire a")
            .value()
            .expect("Wire a value");
        let mut circuit = Circuit::new(input);
        circuit.wires.insert(
            "b".to_string(),
            Wire::Valued {
                name: "b".to_string(),
                value: saved_a,
            },
        );
        while !circuit.wires.get("a").expect("Wire a").has_value() {
            circuit.reduce_entropy_once();
        }
        assert_eq!(circuit.wires.get("a").expect("Wire a").value(), Some(2797));
    }
}
