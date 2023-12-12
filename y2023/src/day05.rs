use advent_of_code_common::range::MultiRange;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
struct RawMapping {
    src_start: i64,
    src_end: i64,
    dst_start: i64,
    dst_end: i64,
    difference: i64,
}

impl TryFrom<&str> for RawMapping {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let dst_start = parts
            .next()
            .ok_or("missing dst_start")?
            .parse::<i64>()
            .map_err(|_| "invalid dst_start")?;
        let src_start = parts
            .next()
            .ok_or("missing src_start")?
            .parse::<i64>()
            .map_err(|_| "invalid src_start")?;
        let length = parts
            .next()
            .ok_or("missing length")?
            .parse::<i64>()
            .map_err(|_| "invalid length")?;
        if parts.next().is_some() {
            return Err("too many parts");
        }
        Ok(Self {
            src_start,
            src_end: src_start + length - 1,
            dst_start,
            dst_end: dst_start + length - 1,
            difference: dst_start - src_start,
        })
    }
}

impl RawMapping {
    fn apply(&self, value: i64) -> Option<i64> {
        if (self.src_start..=self.src_end).contains(&value) {
            Some(value + self.difference)
        } else {
            None
        }
    }

    fn apply_reverse(&self, value: i64) -> Option<i64> {
        if (self.dst_start..=self.dst_end).contains(&value) {
            Some(value - self.difference)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct MultiMapping {
    mappings: Vec<RawMapping>,
}

impl MultiMapping {
    fn apply(&self, value: i64) -> i64 {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.apply(value))
            .unwrap_or(value)
    }

    fn apply_reverse(&self, value: i64) -> i64 {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.apply_reverse(value))
            .unwrap_or(value)
    }
}

#[derive(Debug)]
pub struct SeedLocationMapping {
    pub seeds: Vec<i64>,
    to_soil: MultiMapping,
    to_fertilizer: MultiMapping,
    to_water: MultiMapping,
    to_light: MultiMapping,
    to_temperature: MultiMapping,
    to_humidity: MultiMapping,
    to_location: MultiMapping,
}

impl SeedLocationMapping {
    /// Applies the mapping to the given seed value,
    /// all the way to the location value
    pub fn apply(&self, value: i64) -> i64 {
        let mut value = self.to_soil.apply(value);
        value = self.to_fertilizer.apply(value);
        value = self.to_water.apply(value);
        value = self.to_light.apply(value);
        value = self.to_temperature.apply(value);
        value = self.to_humidity.apply(value);
        self.to_location.apply(value)
    }

    /// Applies the mapping to the given location value,
    /// all the way to the seed value
    pub fn apply_reverse(&self, value: i64) -> i64 {
        let mut value = self.to_location.apply_reverse(value);
        value = self.to_humidity.apply_reverse(value);
        value = self.to_temperature.apply_reverse(value);
        value = self.to_light.apply_reverse(value);
        value = self.to_water.apply_reverse(value);
        value = self.to_fertilizer.apply_reverse(value);
        self.to_soil.apply_reverse(value)
    }

    /// Returns an iterator over all possible seed values
    /// This only applies when the seed values are considered to be ranges.
    /// If the seed values are considered to be individual values,
    /// use `mapping.seeds.iter()`
    pub fn seed_iter(&self) -> impl Iterator<Item = i64> + '_ {
        let mut index = 0;
        let mut ranges = vec![];
        while index < self.seeds.len() {
            let range_start = self.seeds[index];
            let range_end = self.seeds[index] + self.seeds[index + 1];
            let range = range_start..range_end;
            ranges.push(range);
            index += 2;
        }
        ranges.into_iter().flatten()
    }

    /// Returns true if the given value is a valid seed
    /// Checks if the value is in any of the seed ranges
    /// This only applies when the seed values are considered to be ranges.
    /// If the seed values are considered to be individual values,
    /// use `mapping.seeds.contains(value)`
    pub fn contains(&self, value: &i64) -> bool {
        let mut index = 0;
        while index < self.seeds.len() {
            let range_start = self.seeds[index];
            let range_end = self.seeds[index] + self.seeds[index + 1];
            let range = range_start..range_end;
            if range.contains(value) {
                return true;
            }
            index += 2;
        }
        false
    }

    fn reduce_with_multi_range(&self) -> MultiRange<i64> {
        let mut index = 0;
        let mut ranges = vec![];
        while index < self.seeds.len() {
            let range_start = self.seeds[index];
            let range_end = self.seeds[index] + self.seeds[index + 1] - 1;
            let range = range_start..=range_end;
            ranges.push(range);
            index += 2;
        }
        let mut multi_range = MultiRange::from_iter(ranges);
        dbg!(&multi_range);
        for r in &self.to_soil.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(&multi_range);
        for r in &self.to_fertilizer.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(&multi_range);
        for r in &self.to_water.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(&multi_range);
        for r in &self.to_light.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(&multi_range);
        for r in &self.to_temperature.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(&multi_range);
        for r in &self.to_humidity.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(&multi_range);
        for r in &self.to_location.mappings {
            dbg!(r);
            let mut cloned = multi_range.clone();
            cloned -= &(r.src_start..=r.src_end);
            if cloned != multi_range {
                cloned += &(r.dst_start..=r.dst_end);
            }
            multi_range = cloned;
        }
        dbg!(multi_range)
    }

    pub fn minimal_location(&self) -> Option<i64> {
        let multi_range = self.reduce_with_multi_range();
        multi_range.into_iter().flatten().next()
    }
}

pub fn input_parse(s: &str) -> Result<SeedLocationMapping> {
    fn extract_mappings<'a, I>(lines: &mut I) -> MultiMapping
    where
        I: Iterator<Item = &'a str>,
    {
        MultiMapping {
            mappings: lines
                .take_while(|line| !line.is_empty())
                .filter_map(|line| RawMapping::try_from(line).ok())
                .collect(),
        }
    }
    let mut lines = s.lines();
    let seeds = lines
        .next()
        .context("missing seeds")?
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    loop {
        let line = lines.next().unwrap_or_default();
        if line.starts_with("seed-to-soil") {
            break;
        }
    }

    let to_soil = extract_mappings(&mut lines);
    let to_fertilizer = extract_mappings(&mut lines);
    let to_water = extract_mappings(&mut lines);
    let to_light = extract_mappings(&mut lines);
    let to_temperature = extract_mappings(&mut lines);
    let to_humidity = extract_mappings(&mut lines);
    let to_location = extract_mappings(&mut lines);
    Ok(SeedLocationMapping {
        seeds,
        to_soil,
        to_fertilizer,
        to_water,
        to_light,
        to_temperature,
        to_humidity,
        to_location,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_raw_mapping() {
        let example = include_str!("day05_example.txt");
        let mappings = example
            .lines()
            .filter_map(|line| RawMapping::try_from(line).ok())
            .collect_vec();
        dbg!(&mappings[0]);
        assert_eq!(mappings[0].apply(0), None, "0");
        assert_eq!(mappings[0].apply(1), None, "1");
        assert_eq!(mappings[0].apply(98), Some(50), "98");
        assert_eq!(mappings[0].apply(99), Some(51), "99");
        assert_eq!(mappings[0].apply(100), None, "100");
        dbg!(&mappings[1]);
        for i in 50..=97 {
            assert_eq!(mappings[1].apply(i), Some(i + 2), "{i}");
        }
        assert_eq!(mappings[1].apply(98), None, "98");
    }

    #[test]
    fn test_reverse_raw() {
        let example = include_str!("day05_example.txt");
        let mappings = example
            .lines()
            .filter_map(|line| RawMapping::try_from(line).ok())
            .collect_vec();
        dbg!(&mappings[0]);
        assert_eq!(mappings[0].apply_reverse(0), None, "0");
        assert_eq!(mappings[0].apply_reverse(1), None, "1");
        assert_eq!(mappings[0].apply_reverse(50), Some(98), "50");
        assert_eq!(mappings[0].apply_reverse(51), Some(99), "51");
        assert_eq!(mappings[0].apply_reverse(52), None, "52");
        dbg!(&mappings[1]);
        for i in 52..=99 {
            assert_eq!(mappings[1].apply_reverse(i), Some(i - 2), "{i}");
        }
        assert_eq!(mappings[1].apply_reverse(100), None, "100");
    }

    #[test]
    fn test_full_mapping() {
        let example = include_str!("day05_example.txt");
        let mappings = example
            .lines()
            .filter_map(|line| RawMapping::try_from(line).ok())
            .collect_vec();
        let full_mapping = MultiMapping {
            mappings: mappings[0..2].to_vec(),
        };
        for i in 0..50 {
            assert_eq!(full_mapping.apply(i), i, "{i}");
        }
        for i in 50..98 {
            assert_eq!(full_mapping.apply(i), i + 2, "{i}");
        }
        for i in 98..100 {
            assert_eq!(full_mapping.apply(i), i - 48, "{i}");
        }
        for i in 100..110 {
            assert_eq!(full_mapping.apply(i), i, "{i}");
        }
    }

    #[test]
    fn test_seed_to_location() {
        let example = include_str!("day05_example.txt");
        let seed_mapping = input_parse(example).unwrap();
        assert_eq!(seed_mapping.seeds, vec![79, 14, 55, 13]);
        let results = seed_mapping
            .seeds
            .iter()
            .map(|seed| seed_mapping.apply(*seed))
            .collect_vec();
        assert_eq!(results, vec![82, 43, 86, 35]);
        assert_eq!(results.iter().min(), Some(&35));
    }

    #[test]
    fn test_location_to_seed() {
        let example = include_str!("day05_example.txt");
        let seed_mapping = input_parse(example).unwrap();
        let locations = [82, 43, 86, 35];
        assert_eq!(seed_mapping.seeds, vec![79, 14, 55, 13]);
        let results = locations
            .iter()
            .map(|loc| seed_mapping.apply_reverse(*loc))
            .collect_vec();
        assert_eq!(results, seed_mapping.seeds);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("day05.txt");
        let seed_mapping = input_parse(input).unwrap();
        let results = seed_mapping
            .seeds
            .iter()
            .map(|seed| seed_mapping.apply(*seed))
            .min();
        assert_eq!(results, Some(265018614));
    }

    #[test]
    fn test_example_part2_iter() {
        let example = include_str!("day05_example.txt");
        let seed_mapping = input_parse(example).unwrap();
        let results = seed_mapping
            .seed_iter()
            .map(|seed| seed_mapping.apply(seed))
            .min();
        assert_eq!(results, Some(46));
    }

    #[test]
    fn test_example_part2_reverse() {
        let example = include_str!("day05_example.txt");
        let seed_mapping = input_parse(example).unwrap();
        let result = (0..).find(|loc| seed_mapping.contains(&seed_mapping.apply_reverse(*loc)));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_example_part2_reduce() {
        let example = include_str!("day05_example.txt");
        let seed_mapping = input_parse(example).unwrap();
        let result = seed_mapping.minimal_location();
        assert_eq!(result, Some(46));
    }
}
