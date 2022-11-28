use itertools::{iproduct, Itertools};
use num::{Integer, NumCast, One, Zero};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

/// A 2 dimensional Vector
pub type V2<T> = Vec<Vec<T>>;

/// Count the recurrence of recurrences
pub fn counts_of_counts<T: Eq + Hash>(v: Vec<T>) -> HashMap<usize, usize> {
    v.iter().counts().values().copied().counts()
}

/// Split on lines breaks and trim whitespace from lines
pub fn split_lines(s: &str) -> Vec<String> {
    s.trim().split('\n').map(|x| x.trim().to_string()).collect()
}

/// Read data from file and return it as a Vector of Strings
pub fn get_data(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(split_lines(&std::fs::read_to_string(path)?))
}

/// Flips the axis of 2d Vectors
pub fn transpose<T>(v: V2<T>) -> V2<T> {
    if v.is_empty() {
        return v;
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

/// Sums the series [1, 2, .., n]
pub fn simple_series_sum<T: Integer + NumCast + Copy>(n: T) -> T {
    (n * n + n) / NumCast::from(2).unwrap()
}

/// Get's neighbor coordinates for 2d grids
pub fn get_neighbors<T>(x: T, y: T, w: T, h: T, diagonals: bool) -> Vec<(T, T)>
where
    T: Integer + Clone + Copy,
{
    let mut xs = vec![];
    let mut ys = vec![];
    if x > Zero::zero() {
        xs.push(x - One::one());
    }
    if x < w - One::one() {
        xs.push(x + One::one());
    }
    if y > Zero::zero() {
        ys.push(y - One::one());
    }
    if y < h - One::one() {
        ys.push(y + One::one());
    }

    if diagonals {
        xs.push(x);
        ys.push(y);
        iproduct!(xs, ys).collect()
    } else {
        let mut result: Vec<(T, T)> = xs.iter().map(|xi| (*xi, y)).collect();
        result.extend(ys.iter().map(|yi| (x, *yi)));
        result
    }
}

/// Convert lines of strings into a Vector based on given function
pub fn parse_lines<F, T>(lines: &str, f: F) -> Vec<T>
where
    F: Fn(&String) -> T,
{
    split_lines(lines).iter().map(f).collect()
}

/// Convert lines of strings to integers
pub fn parse_number_lines<T>(lines: &str) -> Vec<T>
where
    T: Integer + FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    parse_lines(lines, |s| s.parse().unwrap())
}

/// Convert lines of strings to 2d Vector of digits
pub fn parse_digit_lines<T>(lines: &str, radix: u32) -> V2<T>
where
    T: Integer + NumCast,
{
    parse_lines(lines, |s| {
        s.chars()
            .map(|c| NumCast::from(c.to_digit(radix).unwrap()).unwrap())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num() {
        let lines = "
        423
        32587
        0
        -3";
        let result: Vec<i32> = parse_number_lines(lines);
        assert_eq!(result, vec![423, 32587, 0, -3]);
    }

    #[test]
    fn test_parse_digits() {
        let lines = "
        123
        456";
        let result = parse_digit_lines::<u32>(lines, 10);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6]])
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(get_neighbors(0, 0, 10, 10, false), vec![(1, 0), (0, 1)]);
        assert_eq!(get_neighbors(10, 10, 10, 10, false), vec![(9, 10), (10, 9)]);
        assert_eq!(
            get_neighbors(5, 5, 10, 10, false),
            vec![(4, 5), (6, 5), (5, 4), (5, 6)]
        );
    }

    #[test]
    fn test_neighbors_diagonals() {
        assert_eq!(
            get_neighbors(0, 0, 10, 10, true),
            vec![(1, 1), (1, 0), (0, 1), (0, 0)]
        );
        assert_eq!(
            get_neighbors(10, 10, 10, 10, true),
            vec![(9, 9), (9, 10), (10, 9), (10, 10)]
        );
        assert_eq!(
            get_neighbors(5, 5, 10, 10, true),
            vec![
                (4, 4),
                (4, 6),
                (4, 5),
                (6, 4),
                (6, 6),
                (6, 5),
                (5, 4),
                (5, 6),
                (5, 5)
            ]
        );
    }
}
