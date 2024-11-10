use itertools::iproduct;
use std::collections::HashMap;

#[derive(Debug)]
pub enum EnginePart {
    Number(u32),
    Symbol(char),
}

pub type EngineMap = HashMap<(usize, usize), EnginePart>;

pub fn create_engine_map(s: &str) -> EngineMap {
    let mut map = HashMap::new();
    let mut current_number = String::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if "0123456789".contains(c) {
                current_number.push(c);
            } else {
                if !current_number.is_empty() {
                    let new_part = EnginePart::Number(current_number.parse().expect("number"));
                    map.insert((x - current_number.len(), y), new_part);
                    current_number.clear();
                }
                if c != '.' {
                    map.insert((x, y), EnginePart::Symbol(c));
                }
            }
        }
        if !current_number.is_empty() {
            let new_part = EnginePart::Number(current_number.parse().expect("number"));
            map.insert((line.len() - current_number.len(), y), new_part);
            current_number.clear();
        }
    }
    map
}

pub fn numbers_with_neighbors(map: &EngineMap) -> Vec<u32> {
    let mut numbers = Vec::new();
    for ((x, y), part) in map {
        if let EnginePart::Number(number) = part {
            let x_range = (x.saturating_sub(1))..=(x + number.to_string().len());
            let y_range = (y.saturating_sub(1))..=(*y + 1);
            for (x_s, y_s) in iproduct!(x_range, y_range) {
                if let Some(EnginePart::Symbol { .. }) = map.get(&(x_s, y_s)) {
                    numbers.push(*number);
                    break;
                }
            }
        }
    }
    numbers
}

pub fn find_number_neighbors(map: &EngineMap, x: &usize, y: &usize) -> Vec<u32> {
    map.iter()
        .filter(|((x_n, y_n), part)| {
            if let EnginePart::Number(n) = part {
                let x_range = (x_n.saturating_sub(1))..=(x_n + n.to_string().len());
                let y_range = (y_n.saturating_sub(1))..=(y_n + 1);
                x_range.contains(x) && y_range.contains(y)
            } else {
                false
            }
        })
        .map(|((_, _), part)| match part {
            EnginePart::Number(n) => *n,
            EnginePart::Symbol(_) => unreachable!("Symbols are filtered out"),
        })
        .collect()
}

pub fn find_gears(map: &EngineMap) -> Vec<u32> {
    let gears: Vec<u32> = map
        .iter()
        .filter(|(_, part)| match part {
            EnginePart::Symbol(c) => *c == '*',
            EnginePart::Number(_) => false,
        })
        .map(|((x, y), _)| find_number_neighbors(map, x, y))
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .collect();
    gears
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = r"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn test_example_map() {
        let map = create_engine_map(EXAMPLE1);
        assert_eq!(map.len(), 16);
    }

    #[test]
    fn test_example_numbers() {
        let map = create_engine_map(EXAMPLE1);
        let result: u32 = numbers_with_neighbors(&map).iter().sum();
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part1() {
        let map = create_engine_map(include_str!("../../inputs/2023/day03.txt"));
        let numbers = numbers_with_neighbors(&map);
        let result: u32 = numbers.iter().sum();
        assert_eq!(result, 522726);
    }

    #[test]
    fn test_gears() {
        let map = create_engine_map(EXAMPLE1);
        let result: u32 = find_gears(&map).iter().sum();
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part2() {
        let map = create_engine_map(include_str!("../../inputs/2023/day03.txt"));
        let result: u32 = find_gears(&map).iter().sum();
        assert_eq!(result, 81721933);
    }
}
