use num::{Integer, NumCast};
use std::fmt::Debug;
use std::str::FromStr;

use crate::split_lines;
use crate::v2::V2;

/// Read data from file and return it as a Vector of Strings
pub fn get_data(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(split_lines(&std::fs::read_to_string(path)?))
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
}
