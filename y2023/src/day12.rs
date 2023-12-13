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

// pub fn vec_into_tuples(vec: &Vec<usize>) -> Vec<(usize, usize, usize)> {
//     (0..vec.len())
//         .map(|index| {
//             let (before, after) = vec.split_at(index);
//             let (value, after) = after.split_first().unwrap_or((&0, &[]));
//             (
//                 before.iter().copied().sum::<usize>() + before.len(),
//                 *value,
//                 after.iter().copied().sum::<usize>() + after.len(),
//             )
//         })
//         .collect()
// }

// pub fn match_option(option: (usize, usize, usize), target: usize) -> Vec<usize> {
//     let mut result = vec![];
//     // length = minimal bit representation of target
//     let mut length = 1;
//     while 1 << length < target {
//         length += 1;
//     }
//     let (before, value, after) = option;
//     let all_digits = before + value + after;
//     if target == 0 || length < all_digits {
//         return result;
//     }
//     let diff = length - all_digits;
//     let mut shifted = (2 << value) - 1; // 2 << 2 = 0100, 0100 - 1 = 0011
//     shifted <<= after; // 0011 << 2 = 1100 (starting position)
//     for _ in 0..=diff {
//         shifted <<= 1; // 0000_1100 << 2 = 0011_0000
//         if target & shifted == shifted {
//             // 0011_0000 & 0011_0111 == 0011_0000 Is a match
//             result.push(shifted);
//         }
//     }
//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_vec_into_tuples() {
//         assert_eq!(
//             vec_into_tuples(&vec![1, 2, 3]),
//             vec![(0, 1, 7), (2, 2, 4), (5, 3, 0)]
//         );
//         assert_eq!(
//             vec_into_tuples(&vec![1, 2, 3, 4]),
//             vec![(0, 1, 12), (2, 2, 9), (5, 3, 5), (9, 4, 0)]
//         );
//     }

//     #[test]
//     fn match_options() {
//         assert_eq!(match_option((0, 2, 2), 0b1100), vec![0b1100]);
//         let m = match_option((2, 1, 2), 0b1100);
//         let m_str = m
//             .iter()
//             .map(|&x| format!("{:04b}", x))
//             .collect::<Vec<String>>()
//             .join(", ");
//         assert_eq!(m, vec![0b1100], "m = {:?}", m_str);
//     }
// }
