use advent_of_code_common::Solver;

#[derive(Debug, PartialEq, Eq)]
struct Password([usize; 6]);

impl Password {
    fn tick_up(&mut self) {
        let mut did_carry = false;
        for i in 0..self.0.len() {
            let i = self.0.len() - 1 - i;
            self.0[i] += 1;
            if self.0[i] == 10 {
                did_carry = true;
            } else {
                break;
            }
        }
        if did_carry {
            for i in 0..self.0.len() {
                if self.0[i] == 10 {
                    self.0[i] = self.0[i - 1];
                }
            }
        }
    }

    fn tick_double(&mut self) {
        loop {
            self.tick_up();
            if self.0.windows(2).any(|w| w[0] == w[1]) {
                break;
            }
        }
    }

    fn tick_double_strict(&mut self) {
        loop {
            self.tick_double();
            let mut counts = [0; 10];
            for &d in &self.0 {
                counts[d] += 1;
            }
            if counts.iter().any(|&c| c == 2) {
                break;
            }
        }
    }
}

impl std::ops::Deref for Password {
    type Target = [usize; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[usize; 6]> for Password {
    fn from(value: [usize; 6]) -> Self {
        Self(value)
    }
}

impl From<&[usize; 6]> for Password {
    fn from(value: &[usize; 6]) -> Self {
        Self(*value)
    }
}

impl From<usize> for Password {
    fn from(n: usize) -> Self {
        let mut current = [0; 6];
        for (i, k) in current.iter_mut().rev().enumerate() {
            let i = u32::try_from(i).expect("fits in u32");
            *k = (n / 10_usize.pow(i)) % 10;
        }
        Self(current)
    }
}

impl From<&Password> for usize {
    fn from(value: &Password) -> Self {
        value
            .0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, k)| {
                let i = u32::try_from(i).expect("fits in u32");
                k * (10_usize.pow(i))
            })
            .sum::<Self>()
    }
}

fn solve(input: &str, tick_f: fn(&mut Password)) -> usize {
    let (start, end) = input.trim().split_once('-').expect("valid input");
    let start = usize::from_str_radix(start, 10).expect("valid input");
    let end = usize::from_str_radix(end, 10).expect("valid input");
    let mut current = Password::from(start);
    // Ensure the first counted password is valid
    while current.0.windows(2).any(|w| w[0] > w[1]) {
        tick_f(&mut current);
    }
    let mut count = 0;
    while usize::from(&current) <= end {
        tick_f(&mut current);
        count += 1;
    }
    count
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        solve(input, Password::tick_double)
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        solve(input, Password::tick_double_strict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "178416-676461";

    #[test]
    fn part_1() {
        assert_eq!(Part1.solve(INPUT), 1650);
    }

    #[test]
    fn part_2() {
        assert_eq!(Part2.solve(INPUT), 1129);
    }

    #[test]
    fn test_password() {
        assert_eq!(Password::from(123456), Password::from([1, 2, 3, 4, 5, 6]));
        assert_eq!(Password::from(987654), Password::from([9, 8, 7, 6, 5, 4]));
        assert_eq!(Password::from(12), Password::from([0, 0, 0, 0, 1, 2]));
        assert_eq!(Password::from(0), Password::from([0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_tick_up() {
        let mut p = Password::from(123456);
        p.tick_up();
        assert_eq!(p, Password::from(123457));
        p.tick_up();
        assert_eq!(p, Password::from(123458));
        p.tick_up();
        assert_eq!(p, Password::from(123459));
        p.tick_up();
        assert_eq!(p, Password::from(123466));
        let mut p = Password::from(123999);
        p.tick_up();
        assert_eq!(p, Password::from(124444));
    }

    #[test]
    fn test_tick_double() {
        let mut p = Password::from(123456);
        p.tick_double();
        assert_eq!(p, Password::from(123466));
        let mut p = Password::from(123499);
        p.tick_double();
        assert_eq!(p, Password::from(123555));
    }
}
