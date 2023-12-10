use anyhow::{Context, Result};
use itertools::Itertools;

pub const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";
pub const INPUT: &str = "Time:        63     78     94     68
Distance:   411   1274   2047   1035";

#[derive(Debug, Clone, Copy)]
pub struct Race {
    pub time: f64,
    pub distance: f64,
}

impl Race {
    #[allow(clippy::float_cmp)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn ways_to_win(&self) -> usize {
        let discriminant = self.time.mul_add(self.time, -4.0 * self.distance);
        if discriminant < 0.0 {
            return 0;
        }
        let discriminant = (discriminant).sqrt();
        let low = ((self.time - discriminant) / 2.0).ceil();
        let high = ((self.time + discriminant) / 2.0).floor();
        let mut count = high - low + 1.0;
        if high * (self.time - high) == self.distance {
            count -= 1.0; // high is a solution, but not a win
        }
        if low * (self.time - low) == self.distance {
            count -= 1.0; // low is a solution, but not a win
        }
        count as usize
    }
}

pub fn parse_input1(s: &str) -> Result<Vec<Race>> {
    let mut lines = s.lines();
    let times = lines
        .next()
        .context("No time line")?
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());
    let distances = lines
        .next()
        .context("No distance line")?
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());
    #[allow(clippy::cast_precision_loss)]
    let races = times
        .zip(distances)
        .map(|(time, distance)| Race {
            time: time as f64,
            distance: distance as f64,
        })
        .collect_vec();
    Ok(races)
}

pub fn parse_input2(s: &str) -> Result<Race> {
    let mut lines = s.lines();
    let time = lines
        .next()
        .context("No time line")?
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<f64>()?;
    let distance = lines
        .next()
        .context("No distance line")?
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<f64>()?;
    Ok(Race { time, distance })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = parse_input1(EXAMPLE).unwrap();
        let result = input.iter().map(|r| r.ways_to_win()).collect_vec();
        assert_eq!(result, vec![4, 8, 9]);
        assert_eq!(result.iter().product::<usize>(), 288);
    }

    #[test]
    fn part1() {
        let input = parse_input1(INPUT).unwrap();
        let result = input.iter().map(|r| r.ways_to_win()).product::<usize>();
        assert_eq!(result, 781200);
    }

    #[test]
    fn example2() {
        let input = parse_input2(EXAMPLE).unwrap();
        let result = input.ways_to_win();
        assert_eq!(result, 71503);
    }

    #[test]
    fn part2() {
        let input = parse_input2(INPUT).unwrap();
        let result = input.ways_to_win();
        assert_eq!(result, 49240091);
    }
}
