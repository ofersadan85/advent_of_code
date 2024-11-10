use anyhow::anyhow;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
enum LightState {
    On,
    #[default]
    Off,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Light {
    x: usize,
    y: usize,
    state: LightState,
}

impl Light {
    fn neighbors(&self, max: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for x in self.x.saturating_sub(1)..=max.min(self.x + 1) {
            for y in self.y.saturating_sub(1)..=max.min(self.y + 1) {
                if x != self.x || y != self.y {
                    v.push((x, y));
                }
            }
        }
        v
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    lights: Vec<Light>,
    size: usize,
}

impl std::str::FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s
            .lines()
            .next()
            .ok_or_else(|| anyhow!("No first line"))?
            .trim()
            .len();
        let lights = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        let state = match c {
                            '.' => Ok(LightState::Off),
                            '#' => Ok(LightState::On),
                            c => Err(anyhow!("Unknown light state {c}")),
                        };
                        state.map(|state| Light { x, y, state })
                    })
                    .filter_map(std::result::Result::ok)
            })
            .collect();
        Ok(Self { lights, size })
    }
}

impl Board {
    fn corner_on_step(&mut self) {
        self.lights.get_mut(0).expect("top left").state = LightState::On;
        self.lights.get_mut(self.size - 1).expect("top right").state = LightState::On;
        self.lights
            .get_mut(self.size * self.size - self.size)
            .expect("bottom left")
            .state = LightState::On;
        self.lights
            .get_mut(self.size * self.size - 1)
            .expect("bottom right")
            .state = LightState::On;
        self.step();
        self.lights.get_mut(0).expect("top left").state = LightState::On;
        self.lights.get_mut(self.size - 1).expect("top right").state = LightState::On;
        self.lights
            .get_mut(self.size * self.size - self.size)
            .expect("bottom left")
            .state = LightState::On;
        self.lights
            .get_mut(self.size * self.size - 1)
            .expect("bottom right")
            .state = LightState::On;
    }

    fn corner_on_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.corner_on_step();
        }
    }

    fn step(&mut self) {
        let mut new_board = Vec::with_capacity(self.lights.len());
        for y in 0..self.size {
            for x in 0..self.size {
                let mut on_count = 0;
                let mut this = self.lights[y * self.size + x];
                let neighbors = this.neighbors(self.size - 1);
                for (xn, yn) in neighbors {
                    if self.lights[yn * self.size + xn].state == LightState::On {
                        on_count += 1;
                    }
                }
                this.state = match (this.state, on_count) {
                    (LightState::On, 2 | 3) | (LightState::Off, 3) => LightState::On,
                    _ => LightState::Off,
                };
                new_board.push(this);
            }
        }
        self.lights = new_board;
    }

    fn steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn count_on(&self) -> usize {
        self.lights
            .iter()
            .filter(|light| light.state == LightState::On)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: [&str; 5] = [
        ".#.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####..",
        "..##..
    ..##.#
    ...##.
    ......
    #.....
    #.##..",
        "..###.
    ......
    ..###.
    ......
    .#....
    .#....",
        "...#..
    ......
    ...#..
    ..##..
    ......
    ......",
        "......
    ......
    ..##..
    ..##..
    ......
    ......",
    ];

    #[test]
    fn test_neighbors() {
        let mut light = Light {
            x: 0,
            y: 0,
            ..Default::default()
        };
        assert_eq!(light.neighbors(99).len(), 3, "{:?}", light.neighbors(99));
        light.x = 1;
        assert_eq!(light.neighbors(99).len(), 5, "{:?}", light.neighbors(99));
        light.y = 1;
        assert_eq!(light.neighbors(99).len(), 8, "{:?}", light.neighbors(99));
        light.x = 50;
        light.y = 50;
        assert_eq!(light.neighbors(99).len(), 8, "{:?}", light.neighbors(99));
        light.y = 99;
        assert_eq!(light.neighbors(99).len(), 5, "{:?}", light.neighbors(99));
        light.x = 99;
        assert_eq!(light.neighbors(99).len(), 3, "{:?}", light.neighbors(99));
    }

    #[test]
    fn test_parse() {
        let board: Board = EXAMPLE[0].parse().unwrap();
        assert_eq!(board.lights.len(), 36);
        assert_eq!(board.size, 6);
    }

    #[test]
    fn test_steps() {
        let mut board0: Board = EXAMPLE[0].parse().unwrap();
        let board1: Board = EXAMPLE[1].parse().unwrap();
        let board2: Board = EXAMPLE[2].parse().unwrap();
        let board3: Board = EXAMPLE[3].parse().unwrap();
        let board4: Board = EXAMPLE[4].parse().unwrap();
        board0.step();
        assert_eq!(board0, board1);
        board0.step();
        assert_eq!(board0, board2);
        board0.step();
        assert_eq!(board0, board3);
        board0.step();
        assert_eq!(board0, board4);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day18.txt").unwrap();
        let mut board: Board = input.parse().unwrap();
        board.steps(100);
        assert_eq!(board.count_on(), 1061);
    }

    #[test]
    fn test_corner_on_steps() {
        let mut board: Board = EXAMPLE[0].parse().unwrap();
        board.corner_on_steps(5);
        assert_eq!(board.count_on(), 17);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day18.txt").unwrap();
        let mut board: Board = input.parse().unwrap();
        board.corner_on_steps(100);
        assert_eq!(board.count_on(), 1006);
    }
}
