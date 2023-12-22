use anyhow::{bail, Result};
use std::{collections::HashMap, iter::once, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModuleType {
    // Button,
    Broadcaster,
    FlipFlop,
    Conjunction,
}

struct Module {
    name: String,
    module_type: ModuleType,
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
    pulse: Pulse,
}

impl FromStr for Module {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or("Invalid input")?;
        let (first, name) = start.split_at(1);
        let outputs = end.split(", ").map(|s| s.to_string()).collect();
        match first {
            "b" => Ok(Module {
                name: start.to_string(),
                module_type: ModuleType::Broadcaster,
                inputs: HashMap::from_iter(once(("button".to_string(), Pulse::Low))),
                outputs,
                pulse: Pulse::Low,
            }),
            "%" => Ok(Module {
                name: name.to_string(),
                module_type: ModuleType::FlipFlop,
                inputs: HashMap::new(),
                outputs,
                pulse: Pulse::Low,
            }),
            "&" => Ok(Module {
                name: name.to_string(),
                module_type: ModuleType::Conjunction,
                inputs: HashMap::from_iter(outputs.iter().map(|s| (s.clone(), Pulse::Low))),
                outputs,
                pulse: Pulse::Low,
            }),
            _ => Err("Invalid input"),
        }
    }
}

impl Module {
    fn input_pulse(&mut self, input: &str, pulse: Pulse) {
        use ModuleType::{Broadcaster, Conjunction, FlipFlop};
        match self.module_type {
            Broadcaster => {
                unimplemented!("Broadcaster modules cannot receive input, use push_button instead")
            } // TODO: Return error instead
            FlipFlop => {
                self.inputs.clear();
                self.inputs.insert(input.to_string(), pulse);
            }
            Conjunction => {
                self.inputs.insert(input.to_string(), pulse);
            }
        }
    }

    fn output_pulse(&mut self) -> HashMap<&str, Pulse> {
        use ModuleType::{Broadcaster, Conjunction, FlipFlop};
        match (self.module_type, self.inputs.values().next(), self.pulse) {
            (Broadcaster, _, _) => {}
            (FlipFlop, Some(Pulse::High), _) => {
                return HashMap::new();
            }
            (FlipFlop, Some(Pulse::Low), Pulse::Low) => self.pulse = Pulse::High,
            (FlipFlop, Some(Pulse::Low), Pulse::High) => self.pulse = Pulse::Low,
            (FlipFlop, None, _) => {} // TODO: Check if this is correct, or if the return value should be empty
            (Conjunction, _, _) => {
                if self.inputs.values().all(|&p| p == Pulse::High) {
                    self.pulse = Pulse::High
                } else {
                    self.pulse = Pulse::Low
                };
            }
        }
        self.outputs
            .iter()
            .map(|s| (s.as_str(), self.pulse))
            .collect()
    }

    fn is_stable(&self) -> bool {
        use ModuleType::{Broadcaster, Conjunction, FlipFlop};
        match self.module_type {
            Conjunction => {
                self.pulse == Pulse::Low && self.inputs.values().all(|&p| p == Pulse::Low)
            }
            FlipFlop => self.pulse == Pulse::Low,
            Broadcaster => true,
        }
    }
}

fn activate_once(broadcaster: Module, modules: &mut HashMap<String, Module>) -> Result<bool> {
    use ModuleType::Broadcaster;
    if broadcaster.module_type != Broadcaster {
        bail!("Invalid input, first module must be a broadcaster");
    }
    for name in broadcaster.outputs.iter() {
        let module = modules.get_mut(name).expect("No module found");
        module.input_pulse(&broadcaster.name, broadcaster.pulse);
    }
    let outputs = modules
        .values_mut()
        .flat_map(|m| m.output_pulse())
        .collect::<HashMap<&str, Pulse>>();
    while let Some(name) = queue.pop() {
        let module = modules.get_mut(&name).expect("No module found");
        module.input_pulse(&broadcaster.name, broadcaster.pulse);
        let pulses = module.output_pulse();
        for (name, pulse) in pulses {
            let module = modules.get_mut(name).unwrap();
            module.input_pulse(&module.name, pulse);
            queue.push(module.name.clone());
        }
    }
    Ok(modules.values().all(|m| m.is_stable()))
}
