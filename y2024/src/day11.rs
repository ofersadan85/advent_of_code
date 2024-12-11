use std::collections::HashMap;

fn transform(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![1];
    }
    let s = n.to_string();
    if s.len() % 2 == 0 {
        let first = s
            .chars()
            .take(s.len() / 2)
            .collect::<String>()
            .parse()
            .expect("first");
        let second = s
            .chars()
            .skip(s.len() / 2)
            .collect::<String>()
            .parse()
            .expect("second");
        vec![first, second]
    } else {
        vec![n * 2024]
    }
}

fn transform_row(row: &mut HashMap<usize, usize>) {
    let mut new_map = HashMap::with_capacity(row.len() * 2);
    for (value, count) in row.iter() {
        for new_value in transform(*value) {
            new_map
                .entry(new_value)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
    }
    *row = new_map;
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_transform() {
        assert_eq!(transform(0), vec![1]);
        assert_eq!(transform(1), vec![2024]);
        assert_eq!(transform(12), vec![1, 2]);
        assert_eq!(transform(253000), vec![253, 0]);
    }

    #[test]
    fn example_1() {
        let mut row = HashMap::from([(125, 1), (17, 1)]);
        for _ in 0..25 {
            transform_row(&mut row);
        }
        assert_eq!(row.values().sum::<usize>(), 55312);
    }

    #[test]
    fn part_1() {
        let mut row = read_to_string("../inputs/2024/day11.txt")
            .unwrap()
            .split_whitespace()
            .map(|s| (s.parse().unwrap(), 1))
            .collect();
        for _ in 0..25 {
            transform_row(&mut row);
        }
        assert_eq!(row.values().sum::<usize>(), 197357);
    }

    #[test]
    fn part_2() {
        let mut row = read_to_string("../inputs/2024/day11.txt")
            .unwrap()
            .split_whitespace()
            .map(|s| (s.parse().unwrap(), 1))
            .collect();
        for _ in 0..75 {
            transform_row(&mut row);
        }
        assert_eq!(row.values().sum::<usize>(), 234568186890978);
    }
}
