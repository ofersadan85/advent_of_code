use std::collections::HashMap;

pub fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |h, c| h.wrapping_add(c as u8).wrapping_mul(17))
}

#[derive(Debug, Default)]
pub struct LightBox<'a> {
    labels: Vec<&'a str>,
    values: HashMap<&'a str, u8>,
}

impl LightBox<'_> {
    pub fn total_value(&self) -> usize {
        self.labels
            .iter()
            .filter(|&label| self.values.contains_key(label))
            .enumerate()
            .map(|(i, label)| self.values[label] as usize * (i + 1))
            .sum()
    }
}

pub fn parse_input(s: &str) -> Vec<LightBox<'_>> {
    let mut boxes: Vec<LightBox> = (0..256).map(|_| LightBox::default()).collect();
    for item in s.split(',') {
        let (label, value_s) = item
            .split_once('=')
            .or_else(|| item.split_once('-'))
            .expect("Invalid input");
        let value: u8 = value_s.parse().unwrap_or(0); // Works because values are 1-9
        let box_ = &mut boxes[hash(label) as usize];
        match value {
            0 => box_.labels.retain(|l| *l != label),
            _ => {
                if !box_.labels.contains(&label) {
                    box_.labels.push(label);
                }
            }
        }
        box_.values.insert(label, value);
    }
    boxes
}

pub fn sum_boxes(boxes: &[LightBox]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| b.total_value() * (i + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_one() {
        let expected = [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        for (i, s) in EXAMPLE.split(',').enumerate() {
            assert_eq!(hash(s), expected[i], "{s}");
        }
    }

    #[test]
    fn hash_all() {
        let hashes: usize = EXAMPLE.split(',').map(|s| hash(s) as usize).sum();
        assert_eq!(hashes, 1320);
    }

    #[test]
    fn part1() {
        let input = include_str!("../../inputs/2023/day15.txt");
        let hashes: usize = input.split(',').map(|s| hash(s) as usize).sum();
        assert_eq!(hashes, 505427);
    }

    #[test]
    fn boxes_example() {
        let boxes = parse_input(EXAMPLE);
        let result = sum_boxes(&boxes);
        assert_eq!(result, 145);
    }

    #[test]
    fn part2() {
        let input = include_str!("../../inputs/2023/day15.txt");
        let boxes = parse_input(input);
        let result = sum_boxes(&boxes);
        assert_eq!(result, 243747);
    }
}
