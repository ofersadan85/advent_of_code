use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct UnconnectedNode;

impl fmt::Display for UnconnectedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct Directed {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for Directed {
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

pub struct Undirected {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for Undirected {
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mutable()
                    .insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table_mutable()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    /// Get connected neighbors
    ///
    /// # Errors
    ///
    /// Will return `UnconnectedNode` if the node is not in the graph
    fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, UnconnectedNode> {
        self.adjacency_table().get(node).ok_or(UnconnectedNode)
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbors) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::Undirected;
    #[test]
    fn test_add_edge() {
        let mut graph = Undirected::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in &expected_edges {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbors() {
        let mut graph = Undirected::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbors("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
        );
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::Directed;
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph = Directed::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&String::from("a"), &String::from("b"), &String::from("c")]
                .iter()
                .copied()
                .collect()
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Directed::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("b"), &String::from("c"), 10),
        ];
        for edge in &expected_edges {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbors() {
        let mut graph = Directed::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(graph.neighbors("a").unwrap(), &vec![(String::from("b"), 5)]);
    }

    #[test]
    fn test_contains() {
        let mut graph = Directed::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
        assert!(!graph.contains("d"));
    }
}
