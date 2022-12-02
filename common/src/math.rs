use itertools::Itertools;
use num::{Integer, Num, NumCast, One, Zero};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum MathError {
    DivisionByZero,
    ImaginaryRoots,
}

/// Find real quadratic roots
#[allow(clippy::float_cmp)] // Checking for strict division by zero error
pub fn quadratic_roots_real<T>(a: T, b: T, c: T) -> Result<(f64, f64), MathError>
where
    T: Into<f64>,
{
    let (a, b, c): (f64, f64, f64) = (a.into(), b.into(), c.into());
    let two: f64 = NumCast::from(2).unwrap();
    let discriminant = b * b - two * two * a * c;
    if a == Zero::zero() {
        return Err(MathError::DivisionByZero);
    }
    if discriminant < 0. {
        Err(MathError::ImaginaryRoots)
    } else {
        Ok((
            (-b + discriminant.sqrt()) / (two * a),
            (-b - discriminant.sqrt()) / (two * a),
        ))
    }
}

/// Sums the series [1, 2, .., n]
#[allow(clippy::missing_panics_doc)] // False positive - will never panic
pub fn simple_series_sum<T: Integer + NumCast + Copy>(n: T) -> T {
    (n * n + n) / NumCast::from(2).unwrap()
}

/// Sums the series [n.., m] in steps
#[allow(clippy::missing_panics_doc)] // False positive - will never panic
pub fn series_sum<T>(start: T, end: T, step: T) -> T
where
    T: Num + NumCast + Copy,
{
    let real_end = end - (end % step);
    let steps = (real_end - start) / step + One::one();
    steps * (start + real_end) / NumCast::from(2).unwrap()
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
        assert!(quadratic_roots_real(3, 3, 3).is_err());
        assert!(quadratic_roots_real(0, 5, 3).is_err());
        assert_eq!(quadratic_roots_real(1, 0, -16).unwrap(), (4., -4.));
    }
}

/// A quick shortcut to convert binary strings to integers
pub fn bin2int<T>(s: &str) -> Result<T, T::FromStrRadixErr>
where
    T: Integer,
{
    T::from_str_radix(s, 2)
}

/// Count the recurrence of recurrences
pub fn counts_of_counts<T: Eq + Hash>(v: &[T]) -> HashMap<usize, usize> {
    v.iter().counts().values().copied().counts()
}
