use crate::default_input_path;
use advent_of_code_common::Solver;
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
struct SpineNode<T> {
    value: T,
    small: Option<T>,
    large: Option<T>,
}

impl<T> SpineNode<T> {
    const fn new(value: T) -> Self {
        Self {
            value,
            small: None,
            large: None,
        }
    }

    fn push(&mut self, value: T) -> Result<(), T>
    where
        T: PartialOrd,
    {
        match self.value.partial_cmp(&value) {
            Some(std::cmp::Ordering::Greater) => {
                if self.small.is_none() {
                    self.small = Some(value);
                    Ok(())
                } else {
                    Err(value)
                }
            }
            Some(std::cmp::Ordering::Less) => {
                if self.large.is_none() {
                    self.large = Some(value);
                    Ok(())
                } else {
                    Err(value)
                }
            }
            _ => Err(value),
        }
    }

    fn level_value(&self) -> String
    where
        T: ToString,
    {
        let mut s = String::new();
        s.push_str(
            self.small
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default()
                .as_str(),
        );
        s.push_str(self.value.to_string().as_str());
        s.push_str(
            self.large
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default()
                .as_str(),
        );
        s
    }

    fn level_value_n<N>(&self) -> N
    where
        T: ToString,
        N: FromStr + Default,
    {
        let level_str = self.level_value();
        level_str.parse().unwrap_or_default()
    }
}

#[derive(Debug, Default)]
struct Fishbone<T> {
    id: T,
    nodes: Vec<SpineNode<T>>,
}

impl<T> Fishbone<T> {
    fn push(&mut self, mut value: T)
    where
        T: PartialOrd,
    {
        for node in &mut self.nodes {
            match node.push(value) {
                Ok(()) => return,
                Err(v) => value = v,
            }
        }
        self.nodes.push(SpineNode::new(value));
    }

    const fn iter(&self) -> FishIter<'_, T> {
        FishIter {
            fishbone: self,
            index: 0,
        }
    }

    fn quality(&self) -> String
    where
        T: ToString,
    {
        self.iter().map(ToString::to_string).collect()
    }

    fn quality_n<N>(&self) -> N
    where
        T: ToString,
        N: FromStr + Default,
    {
        let quality_str = self.quality();
        quality_str.parse().unwrap_or_default()
    }
}

struct FishIter<'a, T> {
    fishbone: &'a Fishbone<T>,
    index: usize,
}

impl<'a, T> Iterator for FishIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.fishbone.nodes.get(self.index).map(|node| &node.value);
        self.index += 1;
        v
    }
}

impl<T: FromStr + Default + PartialOrd> FromStr for Fishbone<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fishbone = Self::default();
        let (id, input) = s.split_once(':').unwrap_or_default();
        let numbers = input.split(',').filter_map(|num| num.trim().parse().ok());
        for num in numbers {
            fishbone.push(num);
        }
        fishbone.id = id.trim().parse().unwrap_or_default();
        Ok(fishbone)
    }
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = String;

    fn solve(&self, input: &str) -> Self::Output {
        input
            .parse::<Fishbone<usize>>()
            .unwrap_or_default()
            .quality()
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let minmax = input
            .lines()
            .filter_map(|line| line.parse::<Fishbone<usize>>().ok())
            .minmax_by_key(Fishbone::quality_n::<usize>);
        match minmax {
            itertools::MinMaxResult::NoElements => 0,
            itertools::MinMaxResult::OneElement(n) => n.quality_n::<usize>(),
            itertools::MinMaxResult::MinMax(min, max) => {
                max.quality_n::<usize>() - min.quality_n::<usize>()
            }
        }
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut swords: Vec<_> = input
            .lines()
            .filter_map(|line| line.parse::<Fishbone<usize>>().ok())
            .collect();
        swords.sort_unstable_by(|a, b| {
            let mut cmp = b.quality_n::<usize>().cmp(&a.quality_n());
            if cmp == Ordering::Equal {
                for (a, b) in a.nodes.iter().zip(b.nodes.iter()) {
                    cmp = b.level_value_n::<usize>().cmp(&a.level_value_n());
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
            }
            if cmp == Ordering::Equal {
                return b.id.cmp(&a.id);
            }
            cmp
        });
        for (i, sword) in swords.iter().enumerate() {
            println!("Position {}: {:?}", i + 1, sword.id);
        }
        swords
            .iter()
            .enumerate()
            .fold(0, |acc, (i, sword)| acc + sword.id * (i + 1))
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, "581078");
        expect_solution!(Part1, 1, "8762363643");
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 2, 77053);
        expect_solution!(Part2, 3, 8757643946766);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 4, 260);
        expect_solution!(Part3, 5, 4);
        expect_solution!(Part3, 6, 31782246);
    }
}
