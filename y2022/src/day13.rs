use advent_of_code_common::file::lines_as_blocks;
use anyhow::{Context, Result};
use serde_json::Value as Json;
use std::cmp::Ordering;

const PATH: &str = "../inputs/2022/day13.txt";
const EXAMPLE: &str = "../inputs/2022/day13_example.txt";

#[derive(Debug, Clone)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn is_sorted(&self) -> bool {
        self.left <= self.right
    }
}

#[derive(Debug, Eq, Clone)]
struct Packet(Json);

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Json::Array(_), Json::Number(_)) => self.eq(&Self(Json::Array(vec![other.0.clone()]))),
            (Json::Number(_), Json::Array(_)) => other.eq(&Self(Json::Array(vec![self.0.clone()]))),
            (a, b) => a.eq(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (Json::Number(left), Json::Number(right)) => left.as_u64().cmp(&right.as_u64()),
            (Json::Array(_), Json::Number(_)) => {
                self.cmp(&Self(Json::Array(vec![other.0.clone()])))
            }
            (Json::Number(_), Json::Array(_)) => Self(Json::Array(vec![self.0.clone()])).cmp(other),
            (Json::Array(left), Json::Array(right)) => {
                for (a, b) in left.iter().zip(right.iter()) {
                    if Self(a.clone()) != Self(b.clone()) {
                        return Self(a.clone()).cmp(&Self(b.clone()));
                    }
                }
                left.len().cmp(&right.len())
            }
            _ => Ordering::Equal,
        }
    }
}

impl TryFrom<&str> for Packet {
    type Error = serde_json::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::from_str(s)?))
    }
}

fn input(example: bool) -> Result<Vec<PacketPair>> {
    let path = if example { EXAMPLE } else { PATH };
    let text = std::fs::read_to_string(path).context("Failed to read input file")?;
    let blocks = lines_as_blocks(&text);
    let result = blocks
        .iter()
        .filter_map(|b| {
            let left = Packet::try_from(b[0].as_str()).ok()?;
            let right = Packet::try_from(b[1].as_str()).ok()?;
            Some(PacketPair { left, right })
        })
        .collect();
    Ok(result)
}

fn part_1(data: &[PacketPair]) -> usize {
    data.iter()
        .enumerate()
        .filter(|(_, p)| p.is_sorted())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2(data: &[PacketPair]) -> usize {
    let dividers = PacketPair {
        left: Packet::try_from("[[2]]").expect("Known to be valid"),
        right: Packet::try_from("[[6]]").expect("Known to be valid"),
    };
    let mut data = data.to_vec();
    data.push(dividers.clone());
    let mut all_packets: Vec<_> = data
        .iter()
        .flat_map(|p| [p.left.clone(), p.right.clone()])
        .collect();
    all_packets.sort_unstable();
    all_packets
        .iter()
        .enumerate()
        .filter(|&(_, p)| p == &dividers.left || p == &dividers.right)
        .map(|(i, _)| i + 1)
        .product()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap()), 13);
}

#[test]
fn task_1() {
    assert_eq!(part_1(&input(false).unwrap()), 5529);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()), 140);
}

#[test]
fn task_2() {
    assert_eq!(part_2(&input(false).unwrap()), 27690);
}
