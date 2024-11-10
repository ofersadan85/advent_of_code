use anyhow::anyhow;

const EXAMPLE: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: usize,
    endurance: usize,
    rest: usize,
    points: usize,
}

impl std::str::FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let name = split.next().ok_or(anyhow!("No name"))?.to_string();
        let speed = split.nth(2).ok_or(anyhow!("No speed"))?.parse()?;
        let endurance = split.nth(2).ok_or(anyhow!("No endurance"))?.parse()?;
        let rest = split.nth(6).ok_or(anyhow!("No rest"))?.parse()?;
        Ok(Self {
            name,
            speed,
            endurance,
            rest,
            points: 0,
        })
    }
}

impl Reindeer {
    fn position_at_t(&self, t: usize) -> usize {
        let run_length = self.endurance + self.rest;
        let full_runs = t / run_length;
        let modulo = t - (full_runs * run_length);
        let total =
            (full_runs * self.speed * self.endurance) + (modulo.min(self.endurance) * self.speed);
        total
    }
}

fn step_winners(racers: &mut Vec<Reindeer>, t: usize) {
    let best = racers.iter().map(|r| r.position_at_t(t)).max().unwrap_or(0);
    racers.iter_mut().for_each(|r| {
        if r.position_at_t(t) == best {
            r.points += 1
        }
    });
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_parse_line() {
        let mut lines = EXAMPLE.lines();
        let comet: Reindeer = lines.next().unwrap().parse().unwrap();
        assert_eq!(
            comet,
            Reindeer {
                name: "Comet".to_string(),
                speed: 14,
                endurance: 10,
                rest: 127,
                points: 0
            }
        );
        let dancer: Reindeer = lines.next().unwrap().parse().unwrap();
        assert_eq!(
            dancer,
            Reindeer {
                name: "Dancer".to_string(),
                speed: 16,
                endurance: 11,
                rest: 162,
                points: 0
            }
        )
    }

    #[test]
    fn test_example_1() {
        let mut lines = EXAMPLE.lines();
        let comet: Reindeer = lines.next().unwrap().parse().unwrap();
        assert_eq!(comet.position_at_t(1000), 1120);
        let dancer: Reindeer = lines.next().unwrap().parse().unwrap();
        assert_eq!(dancer.position_at_t(1000), 1056);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day14.txt").unwrap();
        let max = input
            .lines()
            .map(|s| s.parse::<Reindeer>().unwrap().position_at_t(2503))
            .max()
            .unwrap();
        assert_eq!(max, 2640);
    }

    #[test]
    fn test_example_2() {
        // let input = read_to_string("../inputs/2015/day14.txt").unwrap();
        let mut racers: Vec<Reindeer> = EXAMPLE.lines().map(|s| s.parse().unwrap()).collect();
        for t in 1..=1000 {
            step_winners(&mut racers, t);
        }
        assert_eq!(racers[0].points, 312);
        assert_eq!(racers[1].points, 689);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day14.txt").unwrap();
        let mut racers: Vec<Reindeer> = input.lines().map(|s| s.parse().unwrap()).collect();
        for t in 1..=2503 {
            step_winners(&mut racers, t);
        }
        let max = racers.iter().map(|r| r.points).max().unwrap_or(0);
        assert_eq!(max, 1102);
    }
}
