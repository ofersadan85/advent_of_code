use advent_of_code_common::Solver;
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let y = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let z = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        if parts.next().is_some() {
            return Err(());
        }
        Ok(Self { x, y, z })
    }
}

impl Position {
    #[expect(clippy::cast_precision_loss)]
    fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

#[derive(Clone)]
struct Connection {
    cubes: (Position, Position),
    distance: f64,
}

fn sorted_connections(cubes: &[Position]) -> Vec<Connection> {
    let mut connections = Vec::new();
    for (a, b) in cubes.iter().tuple_combinations() {
        connections.push(Connection {
            cubes: (a.clone(), b.clone()),
            distance: a.euclidean_distance(b),
        });
    }
    connections.sort_unstable_by(|a, b| a.distance.total_cmp(&b.distance));
    connections
}

type Cluster = HashSet<Position>;

fn build_clusters(input: &str, limit: usize) -> (Vec<Cluster>, Connection) {
    let cubes: Vec<Position> = input.lines().filter_map(|line| line.parse().ok()).collect();
    let connections = sorted_connections(&cubes);
    let mut clusters: Vec<Cluster> = Vec::new();
    'outer: for connection in &connections[..limit.min(connections.len())] {
        let mut found = None;
        for (idx, cluster) in clusters.iter_mut().enumerate() {
            if cluster.contains(&connection.cubes.0) && cluster.contains(&connection.cubes.1) {
                continue 'outer;
            }
            if cluster.contains(&connection.cubes.0) || cluster.contains(&connection.cubes.1) {
                cluster.insert(connection.cubes.0.clone());
                cluster.insert(connection.cubes.1.clone());
                found = Some(idx);
                break;
            }
        }
        if let Some(idx) = found {
            let mut to_merge = Vec::new();
            for (other_idx, cluster) in clusters.iter().enumerate() {
                if other_idx != idx
                    && (cluster.contains(&connection.cubes.0)
                        || cluster.contains(&connection.cubes.1))
                {
                    to_merge.push(other_idx);
                }
            }
            for merge_idx in to_merge.into_iter().rev() {
                let merging_cluster = clusters.remove(merge_idx);
                clusters[idx].extend(merging_cluster);
            }
        } else {
            let mut new_cluster = Cluster::new();
            new_cluster.insert(connection.cubes.0.clone());
            new_cluster.insert(connection.cubes.1.clone());
            clusters.push(new_cluster);
        }
        if clusters.len() == 1 && clusters[0].len() == cubes.len() {
            return (clusters, connection.clone());
        }
    }
    let last_idx = limit.min(connections.len()) - 1;
    (clusters, connections[last_idx].clone())
}

struct Part1(usize);
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let (clusters, _) = build_clusters(input, self.0);
        let mut sizes: Vec<usize> = clusters.iter().map(Cluster::len).collect();
        sizes.sort_unstable();
        sizes.iter().rev().take(3).product()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let (_, last) = build_clusters(input, usize::MAX);
        last.cubes.0.x * last.cubes.1.x
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1(10), 0, 40);
        expect_solution!(Part1(1000), 1, 133574);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 25272);
        expect_solution!(Part2, 1, 2435100380);
    }
}
