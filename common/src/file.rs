use num::Integer;
use std::fmt::Debug;
use std::num::IntErrorKind;
use std::str::FromStr;

use crate::v2::V2;

/// Split on lines breaks and trim whitespace from lines
pub fn split_lines(s: &str) -> Vec<String> {
    s.split('\n').map(String::from).collect()
}

/// Same as `split_lines` but trims whitespace from start and end of input and from every line
pub fn split_lines_trim(s: &str) -> Vec<String> {
    split_lines(s.trim())
        .iter()
        .map(|row| row.trim().to_string())
        .collect()
}

/// Convert lines of strings into a Vector based on given function
///
/// # Errors
///
/// Will return `Err` if lines cannot be parsed as the required type
pub fn parse_lines<T, E, F>(lines: &str, f: F) -> Result<Vec<T>, E>
where
    F: Fn(&str) -> Result<T, E>,
{
    let lines = split_lines_trim(lines);
    let mut result = vec![];
    for line in lines {
        result.push(f(&line)?);
    }
    Ok(result)
}

/// Convert lines of strings into a Vector based on given function
///
/// # Errors
///
/// Will return `Err` if lines cannot be parsed as the required type
///
/// # Panics
///
/// Will panic if there's an error reading the file
#[allow(clippy::module_name_repetitions)]
pub fn parse_file<T, E, F>(path: &str, f: F) -> Result<Vec<T>, E>
where
    F: Fn(&str) -> Result<Vec<T>, E>,
{
    let lines = std::fs::read_to_string(path).unwrap();
    f(&lines)
}

/// Convert lines of strings to integers
///
/// # Errors
///
/// Will return `Err` if lines cannot be parsed as the required type
pub fn lines_as_numbers<T, E>(lines: &str) -> Result<Vec<T>, E>
where
    T: Integer + FromStr<Err = E>,
    <T as FromStr>::Err: Debug,
{
    parse_lines(lines, |s| s.parse())
}

/// Convert lines of strings to 2d Vector of digits
///
/// # Errors
///
/// Will return `Err` if characters are not valid digits under given radix
pub fn lines_as_digits_radix<T>(lines: &str, radix: u32) -> Result<V2<T>, IntErrorKind>
where
    T: From<u32>,
{
    let mut result = vec![];
    for row in split_lines_trim(lines) {
        let mut row_vec = vec![];
        for c in row.chars() {
            row_vec.push(c.to_digit(radix).ok_or(IntErrorKind::InvalidDigit)?.into());
        }
        result.push(row_vec);
    }
    Ok(result)
}

/// Shortcut to `lines_as_digits_radix` with radix of 10
///
/// # Errors
///
/// Will return `Err` if characters are not valid digits under given radix
pub fn lines_as_digits<T>(lines: &str) -> Result<V2<T>, IntErrorKind>
where
    T: From<u32>,
{
    lines_as_digits_radix(lines, 10)
}

/// Separates lines to blocks on empty lines
pub fn lines_as_blocks(lines: &str) -> V2<String> {
    let mut result = vec![];
    let mut block = vec![];
    for row in split_lines_trim(lines) {
        if row.is_empty() {
            result.push(block);
            block = vec![];
        } else {
            block.push(row);
        }
    }
    result.push(block);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_as_numbers_test() {
        let lines = "
        423
        32587
        0
        -3";
        let result: Vec<i32> = lines_as_numbers(lines).unwrap();
        assert_eq!(result, [423, 32587, 0, -3]);
    }

    #[test]
    fn lines_as_digits_test() {
        let lines = "
            123
            456";
        let result: V2<u64> = lines_as_digits(lines).unwrap();
        assert_eq!(result, [[1, 2, 3], [4, 5, 6]]);
    }

    #[test]
    fn lines_as_blocks_test() {
        let lines = "
        123
        456
        
        789";
        let result = lines_as_blocks(lines);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0], "789");
    }
}
