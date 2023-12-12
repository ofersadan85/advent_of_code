use anyhow::{bail, Result};
use std::{collections::HashSet, str::FromStr};

pub const EXAMPLE: &str = "???.### 1,1,3
                          .??..??...?##. 1,1,3
                          ?#?#?#?#?#?#?#? 1,3,1,6
                          ????.#...#... 4,1,1
                          ????.######..#####. 1,6,5
                          ?###???????? 3,2,1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Filled,
    Empty,
    Unknown,
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Filled),
            '.' => Ok(Self::Empty),
            '?' => Ok(Self::Unknown),
            _ => Err("Invalid spring condition"),
        }
    }
}

impl From<bool> for Cell {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Filled,
            false => Self::Empty,
        }
    }
}

#[derive(Debug)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub groups: Vec<u32>,
}

impl FromStr for Row {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        Ok(Self {
            cells: split
                .next()
                .ok_or("Missing cells")?
                .chars()
                .filter_map(|c| c.try_into().ok())
                .collect(),
            groups: split
                .next()
                .ok_or("Missing groups")?
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        })
    }
}

impl Row {
    #[allow(clippy::cast_possible_truncation)]
    fn bits(&self, fill: bool) -> u32 {
        self.cells
            .iter()
            .rev()
            .enumerate()
            .map(|(index, cell)| match (cell, fill) {
                (Cell::Empty, _) | (Cell::Unknown, false) => 0,
                (Cell::Filled, _) | (Cell::Unknown, true) => 2u32.pow(index as u32),
            })
            .sum()
    }

    fn bit_solutions(&self) -> HashSet<u32> {
        let full = self.bits(true);
        let empty = self.bits(false);
        (empty..=full).filter(|n| n & empty == empty).collect()
    }

    fn from_number(number: u32, length: usize) -> Result<Vec<Cell>> {
        if number >= 1 << length {
            bail!("Number too large");
        }
        Ok((0..length)
            .rev()
            .map(|i| ((number & 1 << i) != 0).into())
            .collect())
    }

    fn is_valid_solution(&self, n: u32) -> bool {
        Self::from_number(n, self.cells.len()).map_or(false, |cells| {
            let bits = self.bits(true);
            n | bits == bits
                && cells
                    .split(|cell| *cell == Cell::Empty)
                    .filter(|g| !g.is_empty())
                    .filter_map(|g| u32::try_from(g.len()).ok())
                    .eq(self.groups.iter().copied())
        })
        // let mut groups = self.groups.iter();
        // let length = self.cells.len();
        // let mut group_index = 0;
        // while let Some(group) = groups.next() {
        //     let solution_cells = Self::from_number(solution, length);
        //     let is_last = group_index == self.groups.len() - 1;
        //     match solution_cells {
        //         Ok(solution_cells) => {
        //             let mut cells = solution_cells.iter();
        //             let mut filled = 0;
        //             while let Some(cell) = cells.next() {
        //                 match cell {
        //                     Cell::Filled => {
        //                         filled += 1;
        //                         if filled > *group {
        //                             println!("{solution} filled too much for {group} ({group_index}) {solution_cells:?}");
        //                             return false;
        //                         }
        //                     }
        //                     Cell::Empty => {
        //                         if filled != 0 {
        //                             break;
        //                         }
        //                     }
        //                     Cell::Unknown => unreachable!("Shouldn't get unknowns"),
        //                 }
        //             }
        //             if *group != filled as u32 {
        //                 println!("{solution} filled too little for {group} ({group_index}) {solution_cells:?}");
        //                 println!("Filled: {filled}");
        //                 return false;
        //             }
        //             if !is_last {
        //                 match cells.next() {
        //                     Some(Cell::Empty) => {}
        //                     _ => {
        //                         println!("{solution} not empty after {group} ({group_index}) {solution_cells:?}");
        //                         return false;
        //                     }
        //                 }
        //             }
        //         }
        //         Err(_) => {
        //             println!(
        //                 "{solution} invalid for {group} ({group_index})",
        //                 solution = solution,
        //                 group = group,
        //                 group_index = group_index
        //             );
        //             return false;
        //         }
        //     }
        //     group_index += 1;
        // }
        // true
    }

    pub fn solutions(&self) -> HashSet<u32> {
        self.bit_solutions()
            .into_iter()
            .filter(|&solution| self.is_valid_solution(solution))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn to_bits() {
        let row = Row::from_str("#??# 1,1,3").unwrap();
        assert_eq!(row.groups, [1, 1, 3]);
        assert_eq!(row.bits(true), 0b1111);
        assert_eq!(row.bits(false), 0b1001);
        assert_eq!(row.bit_solutions(), HashSet::from_iter([9, 11, 13, 15]));
    }

    #[test]
    fn from_number() {
        assert_eq!(
            Row::from_number(0b1111, 4).unwrap(),
            vec![Cell::Filled, Cell::Filled, Cell::Filled, Cell::Filled,]
        );
        assert_eq!(
            Row::from_number(0b1001, 4).unwrap(),
            vec![Cell::Filled, Cell::Empty, Cell::Empty, Cell::Filled,]
        );
        assert_eq!(
            Row::from_number(0b0001, 4).unwrap(),
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filled]
        );
        assert!(Row::from_number(0b10000, 4).is_err());
    }

    #[test]
    fn valid_solutions() {
        // tracing_subscriber::fmt::init();
        let row = Row::from_str("????? 2,1").unwrap();
        // tracing::info!("{:?}", row.bit_solutions());
        assert!(row.is_valid_solution(13));
        // println!("{:?}", row.bit_solutions());
        assert_eq!(
            row.solutions(),
            HashSet::from_iter([0b11010, 0b11001, 0b01101])
        );
    }

    #[test]
    fn example1() {
        let lengths = EXAMPLE
            .lines()
            .filter_map(|line| line.trim().parse::<Row>().ok())
            .map(|row| row.solutions())
            .map(|solutions| solutions.len())
            .collect_vec();
        let expected = vec![1, 4, 1, 1, 4, 10];
        assert_eq!(lengths, expected);
        assert_eq!(lengths.iter().sum::<usize>(), 21);
    }

    #[test]
    fn part1() {
        let lengths: usize = include_str!("day12.txt")
            .lines()
            .filter_map(|line| line.trim().parse::<Row>().ok())
            .map(|row| row.solutions().len())
            .sum();
        assert_eq!(lengths, 7716);
    }
}
