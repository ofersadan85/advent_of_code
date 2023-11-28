use std::collections::HashMap;

enum LogicGate {
    PASSTHROUGH,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

struct WireInput {
    op: LogicGate,
    inputs: [String; 2],
}

impl WireInput {
    fn output(&self, wires: &HashMap<String, Wire>) -> u16 {
        let a = match wires.get(&self.inputs[0]) {
            Some(w) => w.value(wires),
            None => self.inputs[0]
                .parse::<u16>()
                .expect(format!("Invalid input {}", self.inputs[0]).as_str()),
        };
        let b = match wires.get(&self.inputs[1]) {
            Some(w) => w.value(wires),
            None => self.inputs[1].parse::<u16>().unwrap_or(0),
        };
        match self.op {
            LogicGate::PASSTHROUGH => a,
            LogicGate::AND => a & b,
            LogicGate::OR => a | b,
            LogicGate::LSHIFT => a << b,
            LogicGate::RSHIFT => a >> b,
            LogicGate::NOT => !a,
        }
    }
}

struct Wire {
    name: String,
    input: WireInput,
}

impl TryFrom<&str> for Wire {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid input {}", s));
        }
        let name = parts[1];
        let parts = parts[0].split(" ").collect::<Vec<&str>>();
        match parts.as_slice() {
            ["NOT", a] => Ok(Wire::new(
                name,
                WireInput {
                    op: LogicGate::NOT,
                    inputs: [a.to_string(), "".to_string()],
                },
            )),
            [a, "AND", b] => Ok(Wire::new(
                name,
                WireInput {
                    op: LogicGate::AND,
                    inputs: [a.to_string(), b.to_string()],
                },
            )),
            [a, "OR", b] => Ok(Wire::new(
                name,
                WireInput {
                    op: LogicGate::OR,
                    inputs: [a.to_string(), b.to_string()],
                },
            )),
            [a, "LSHIFT", b] => Ok(Wire::new(
                name,
                WireInput {
                    op: LogicGate::LSHIFT,
                    inputs: [a.to_string(), b.to_string()],
                },
            )),
            [a, "RSHIFT", b] => Ok(Wire::new(
                name,
                WireInput {
                    op: LogicGate::RSHIFT,
                    inputs: [a.to_string(), b.to_string()],
                },
            )),
            [a] => Ok(Wire::new(
                name,
                WireInput {
                    op: LogicGate::PASSTHROUGH,
                    inputs: [a.to_string(), "".to_string()],
                },
            )),
            _ => Err(format!("Invalid input {}", s)),
        }
    }
}

impl Wire {
    fn new(name: &str, input: WireInput) -> Wire {
        Wire {
            name: name.to_string(),
            input: input,
        }
    }

    fn value(&self, wires: &HashMap<String, Wire>) -> u16 {
        self.input.output(wires)
    }
}

fn construct_wire_map(input: &str) -> HashMap<String, Wire> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let wire = Wire::try_from(line.trim()).expect(format!("Invalid input {}", line).as_str());
        wires.insert(wire.name.clone(), wire);
    }
    wires
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";
        let wires = construct_wire_map(input);
        assert_eq!(wires.get("d").unwrap().value(&wires), 72);
        assert_eq!(wires.get("e").unwrap().value(&wires), 507);
        assert_eq!(wires.get("f").unwrap().value(&wires), 492);
        assert_eq!(wires.get("g").unwrap().value(&wires), 114);
        assert_eq!(wires.get("h").unwrap().value(&wires), 65412);
        assert_eq!(wires.get("i").unwrap().value(&wires), 65079);
        assert_eq!(wires.get("x").unwrap().value(&wires), 123);
        assert_eq!(wires.get("y").unwrap().value(&wires), 456);
    }

    #[test]
    fn part_1() {
        let input = include_str!("day07.txt");
        let wires = construct_wire_map(input);
        assert_eq!(wires.get("a").unwrap().value(&wires), 3176);
    }
}
