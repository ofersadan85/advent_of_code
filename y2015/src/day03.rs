use std::collections::HashMap;

pub struct Santa {
    map: HashMap<(isize, isize), isize>,
    last: (isize, isize),
}

impl Default for Santa {
    fn default() -> Self {
        Self::new()
    }
}

impl Santa {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), 1);
        Self { map, last: (0, 0) }
    }

    pub fn step(&mut self, direction: char) {
        let (x, y) = self.last;
        let (x, y) = match direction {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            _ => panic!("Invalid direction"),
        };
        self.last = (x, y);
        *self.map.entry((x, y)).or_insert(0) += 1;
    }

    pub fn houses(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../inputs/2015/day03.txt");

    #[test]
    fn example_1() {
        for (input, expected) in &[(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)] {
                let mut santa = Santa::new();
                for c in input.chars() {
                    santa.step(c);
                }
                assert_eq!(santa.houses(), *expected);
            }
    }

    #[test]
    fn part_1() {
        let mut santa = Santa::new();
        for c in INPUT.chars() {
            santa.step(c);
        }
        assert_eq!(santa.houses(), 2592);
    }

    #[test]
    fn part_2() {
        let mut santa = Santa::new();
        let mut robot = Santa::new();
        INPUT.chars().enumerate().for_each(|(i, c)| {
            if i % 2 == 0 {
                santa.step(c);
            } else {
                robot.step(c);
            }
        });
        let mut map = HashMap::new();
        map.extend(santa.map);
        map.extend(robot.map);
        assert_eq!(map.len(), 2360);
    }
}
