use tracing::instrument;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {:?}, B: {:?}, Prize: {:?}",
            self.a, self.b, self.prize
        )
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

fn parse_input(input: &str) -> Result<Vec<Machine>, &'static str> {
    let mut result = vec![];
    let mut a = Point { x: 0, y: 0 };
    let mut b = Point { x: 0, y: 0 };
    for line in input.lines() {
        let mut split = line
            .split_whitespace()
            .map(|s| s.trim_matches(['X', 'Y', '+', '=', ',']));
        match [split.next(), split.next(), split.next(), split.next()] {
            [Some("Button"), Some("A:"), Some(x), Some(y)] => {
                a = Point {
                    x: x.parse().map_err(|_| "invalid x for A")?,
                    y: y.parse().map_err(|_| "invalid y for A")?,
                };
            }
            [Some("Button"), Some("B:"), Some(x), Some(y)] => {
                b = Point {
                    x: x.parse().map_err(|_| "invalid x for B")?,
                    y: y.parse().map_err(|_| "invalid y for B")?,
                };
            }
            [Some("Prize:"), Some(x), Some(y), None] => {
                let prize = Point {
                    x: x.parse().map_err(|_| "invalid x for Prize")?,
                    y: y.parse().map_err(|_| "invalid y for Prize")?,
                };
                result.push(Machine { a, b, prize });
            }
            _ => {},
        }
    }
    Ok(result)
}

impl Machine {
    #[allow(clippy::similar_names)]
    fn minimize_cost(&self) -> Option<u64> {
        let div_bx = self.prize.x / self.b.x;
        let div_by = self.prize.y / self.b.y;
        for b_multiplier in (0..=div_bx.min(div_by)).rev() {
            let reach_b = Point {
                x: b_multiplier * self.b.x,
                y: b_multiplier * self.b.y,
            };
            let diff_b_prize = Point {
                x: self.prize.x - reach_b.x,
                y: self.prize.y - reach_b.y,
            };
            let (div_ax, div_ay) =
                if !diff_b_prize.x.is_multiple_of(self.a.x) || !diff_b_prize.y.is_multiple_of(self.a.y) {
                    continue;
                } else {
                    (diff_b_prize.x / self.a.x, diff_b_prize.y / self.a.y)
                };
            let a_multiplier = if div_ax == div_ay {
                div_ax
            } else {
                continue;
            };
            return Some(a_multiplier * 3 + b_multiplier);
        }
        None
    }
}

#[instrument(skip_all, level = "info")]
fn sum_cost(machines: &[Machine]) -> u64 {
    machines.iter().filter_map(Machine::minimize_cost).sum()
}

#[instrument(skip_all, level = "info")]
fn sum_cost_bigger(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .filter_map(|m| {
            let mut bigger = *m;
            bigger.prize.x += 10_000_000_000_000;
            bigger.prize.y += 10_000_000_000_000;
            bigger.minimize_cost()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use test_log::test;
    const EXAMPLE: &str = "Button A: X+94, Y+34
                           Button B: X+22, Y+67
                           Prize: X=8400, Y=5400

                           Button A: X+26, Y+66
                           Button B: X+67, Y+21
                           Prize: X=12748, Y=12176

                           Button A: X+17, Y+86
                           Button B: X+84, Y+37
                           Prize: X=7870, Y=6450

                           Button A: X+69, Y+23
                           Button B: X+27, Y+71
                           Prize: X=18641, Y=10279";

    #[test]
    fn example_1() {
        let machines = parse_input(EXAMPLE).unwrap();
        assert_eq!(machines.len(), 4);
        assert_eq!(machines[0].minimize_cost(), Some(280));
        assert_eq!(machines[1].minimize_cost(), None);
        assert_eq!(machines[2].minimize_cost(), Some(200));
        assert_eq!(machines[3].minimize_cost(), None);
        assert_eq!(sum_cost(&machines), 480);
    }

    #[test]
    fn part_1() {
        let machines = read_to_string("../inputs/2024/day13.txt")
            .map_err(|e| panic!("Error reading input: {e}"))
            .and_then(|input| parse_input(&input))
            .unwrap();
        assert_eq!(sum_cost(&machines), 29598);
    }

    #[test]
    #[ignore = "takes too long"]
    fn part_2() {
        let machines = read_to_string("../inputs/2024/day13.txt")
            .map_err(|e| panic!("Error reading input: {e}"))
            .and_then(|input| parse_input(&input))
            .unwrap();
        assert_eq!(sum_cost_bigger(&machines), 29598);
    }
}
