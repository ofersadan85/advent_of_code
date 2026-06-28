use itertools::Itertools;
use num::{integer, Integer, Num, PrimInt, Unsigned, Zero};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    ImaginaryRoots,
}

/// Find real quadratic roots
///
/// # Errors
///
/// Will return `Error::DivisionByZero` if first argument is zero
///
/// Will return `Error::ImaginaryRoots` if no real roots can be found
#[allow(clippy::float_cmp, clippy::missing_panics_doc)] // Checking for strict division by zero error
pub fn quadratic_roots_real<T>(a: T, b: T, c: T) -> Result<(f64, f64), Error>
where
    T: Into<f64>,
{
    let (a, b, c): (f64, f64, f64) = (a.into(), b.into(), c.into());
    let discriminant = b.mul_add(b, -4.0 * a * c); // b * b - two * two * a * c;
    if a.is_zero() {
        return Err(Error::DivisionByZero);
    }
    if discriminant < 0.0 {
        Err(Error::ImaginaryRoots)
    } else {
        let divisor = 2.0 * a;
        Ok((
            (-b + discriminant.sqrt()) / divisor,
            (-b - discriminant.sqrt()) / divisor,
        ))
    }
}

/// Sums the series [1, 2, .., n]
#[allow(clippy::missing_panics_doc)] // False positive - will never panic
pub fn simple_series_sum<T: PrimInt>(n: T) -> T {
    (n * n + n) / (T::one() + T::one())
}

/// Sums the series [n.., m] in steps
pub fn series_sum<T>(start: T, end: T, step: T) -> T
where
    T: Num + Copy,
{
    let real_end = end - (end % step);
    let steps = (real_end - start) / step + T::one();
    steps * (start + real_end) / (T::one() + T::one())
}

/// Calculate the prime factors of positive integers
pub fn prime_factors<T>(n: &T) -> Vec<T>
where
    T: Unsigned + integer::Roots + Copy,
{
    let two = T::one() + T::one();
    let mut n = *n;
    let mut div = two;
    let mut result: Vec<T> = Vec::new();
    let max_div = integer::sqrt(n);
    while n > T::one() {
        if div > max_div {
            result.push(n);
            break;
        } else if n % div == T::zero() {
            result.push(div);
            n = n / div;
            div = two;
        } else {
            div = div + T::one();
        }
    }
    result
}

/// A quick shortcut to convert binary strings to integers
///
/// # Errors
///
/// Will return `T::FromStrRadixErr` if string is not a valid binary of 0s and 1s
pub fn bin2int<T: Integer>(s: &str) -> Result<T, T::FromStrRadixErr> {
    T::from_str_radix(s, 2)
}

/// Count the recurrence of recurrences
pub fn counts_of_counts<T: Eq + Hash>(v: &[T]) -> HashMap<usize, usize> {
    v.iter().counts().values().copied().counts()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(&49_u8), [7, 7], "U8 square");
        assert_eq!(prime_factors(&247_u8), [13, 19], "U8");
        assert_eq!(prime_factors(&251_u8), [251], "U8 prime");
        assert_eq!(prime_factors(&8211_u16), [3, 7, 17, 23], "U16");
        assert_eq!(prime_factors(&8211_u32), [3, 7, 17, 23], "U32");
        assert_eq!(prime_factors(&8211_u64), [3, 7, 17, 23], "U64");
        assert_eq!(prime_factors(&8211_u128), [3, 7, 17, 23], "U128");
        assert_eq!(prime_factors(&8211_usize), [3, 7, 17, 23], "Usize");
    }

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
