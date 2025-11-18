use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
    str::FromStr,
};

/// Convenience constant for the engraving limit, 1 million
const M: i64 = 1_000_000;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct ComplexNumber {
    x: i64,
    y: i64,
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl Div for ComplexNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl FromStr for ComplexNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split_once(',')
            .ok_or(())?;
        Ok(Self {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
        })
    }
}

impl ComplexNumber {
    fn cycles(&self, iterations: usize, constant: i64) -> Self {
        let mut result = *self; // We can skip the first cycle this way
        let constant = Self {
            x: constant,
            y: constant,
        };
        for _ in 1..iterations {
            result = result * result;
            result = result / constant;
            result = result + *self;
            if !result.in_range() {
                break;
            }
        }
        result
    }

    const fn in_range(&self) -> bool {
        self.x <= M && self.x >= -M && self.y <= M && self.y >= -M
    }

    fn is_engraved(&self) -> bool {
        self.cycles(100, 100_000).in_range()
    }

    fn count_engraved(&self, limit: i64, step: usize) -> usize {
        let mut count = 0;
        for x in (self.x..=self.x + limit).step_by(step) {
            for y in (self.y..=self.y + limit).step_by(step) {
                let cn = Self { x, y };
                if cn.is_engraved() {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        fn f(s: &str) -> String {
            s.parse::<ComplexNumber>()
                .unwrap()
                .cycles(3, 10)
                .to_string()
        }
        assert_eq!(f("[25,9]"), "[357,862]", "Example 1");
        assert_eq!(f("[163,58]"), "[237221,967434]", "Part 1");
    }

    #[test]
    fn is_engraved() {
        fn f(s: &str) -> bool {
            s.parse::<ComplexNumber>().unwrap().is_engraved()
        }
        assert!(f("[35630,-64880]"), "Example 2, engraved");
        assert!(f("[35630,-64870]"), "Example 2, engraved");
        assert!(f("[35640,-64860]"), "Example 2, engraved");
        assert!(f("[36230,-64270]"), "Example 2, engraved");
        assert!(f("[36250,-64270]"), "Example 2, engraved");
        assert!(!f("[35460,-64910]"), "Example 2, not engraved");
        assert!(!f("[35470,-64910]"), "Example 2, not engraved");
        assert!(!f("[35480,-64910]"), "Example 2, not engraved");
        assert!(!f("[35680,-64850]"), "Example 2, not engraved");
        assert!(!f("[35630,-64830]"), "Example 2, not engraved");
    }

    #[test]
    fn part_2() {
        fn f(s: &str) -> usize {
            s.parse::<ComplexNumber>()
                .unwrap()
                .count_engraved(1000, 10)
        }
        assert_eq!(f("[35300,-64910]"), 4076, "Example 2");
        assert_eq!(f("[-4531,67892]"), 1154, "Part 2");
    }

    #[test]
    fn part_3() {
        fn f(s: &str) -> usize {
            s.parse::<ComplexNumber>()
                .unwrap()
                .count_engraved(1000, 1)
        }
        assert_eq!(f("[35300,-64910]"), 406954, "Example 3");
        assert_eq!(f("[-4531,67892]"), 108057, "Part 3");
    }
}
