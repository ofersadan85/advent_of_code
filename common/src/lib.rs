use itertools::{iproduct, Itertools};
use num::{Integer, Num, NumCast, One, Zero};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub mod algorithms;

/// Find real quadratic roots
pub fn quadratic_roots_real<T>(a: T, b: T, c: T) -> Vec<f64>
where
    T: Into<f64>,
{
    let (a, b, c): (f64, f64, f64) = (a.into(), b.into(), c.into());
    assert!(a > Zero::zero());
    let two: f64 = NumCast::from(2).unwrap();
    let discriminant = b * b - two * two * a * c;
    if discriminant < 0. {
        vec![]
    } else {
        vec![
            (-b + discriminant.sqrt()) / (two * a),
            (-b - discriminant.sqrt()) / (two * a),
        ]
    }
}

/// Quick shortcut to "pretty-print"
pub fn pprint<T: Debug>(item: &T) {
    println!("{:#?}", item);
}

/// A 2 dimensional Vector
pub type V2<T> = Vec<Vec<T>>;

/// A quick shortcut to convert binary strings to integers
pub fn bin2int<T>(s: &str) -> Option<T>
where
    T: Integer,
{
    T::from_str_radix(s, 2).ok()
}

/// Count the recurrence of recurrences
pub fn counts_of_counts<T: Eq + Hash>(v: &[T]) -> HashMap<usize, usize> {
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
    let mut iters: Vec<_> = v.into_iter().map(IntoIterator::into_iter).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

/// Sums the series [1, 2, .., n]
pub fn simple_series_sum<T: Integer + NumCast + Copy>(n: T) -> T {
    series_sum(One::one(), n, One::one())
}

/// Sums the series [n.., m] in steps
pub fn series_sum<T>(start: T, end: T, step: T) -> T
where
    T: Num + NumCast + Copy,
{
    let real_end = end - (end % step);
    let steps = (real_end - start) / step + One::one();
    steps * (start + real_end) / NumCast::from(2).unwrap()
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
    <T as FromStr>::Err: Debug,
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

    #[allow(clippy::float_cmp)] // This comparison is already tested to work
    #[test]
    fn test_series_sum() {
        assert_eq!(series_sum(6., 13., 3.), 27.);
        assert_eq!(series_sum(-6, 13, 2), 30);
        assert_eq!(series_sum(6, 12, 3), series_sum(6, 13, 3));
        assert_eq!(series_sum(1, 13, 1), simple_series_sum(13));
    }

    #[test]
    fn test_quadratic_real() {
        assert_eq!(quadratic_roots_real(3, 3, 3), vec![]);
        assert_eq!(quadratic_roots_real(1, 0, -16), vec![4., -4.]);
    }

    #[test]
    #[should_panic]
    fn test_quadratic_error() {
        quadratic_roots_real(0, 5, 3);
    }

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
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6]]);
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
