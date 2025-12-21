use crate::v2::V2;

/// Split on lines breaks and trim whitespace from lines
pub fn split_lines(s: &str) -> Vec<String> {
    s.split('\n').map(String::from).collect()
}

/// Same as `split_lines` but trims whitespace from start and end of input and from every line
#[must_use]
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
/// Will return `Err` if lines cannot be parsed as the required type.
/// The error type is determined by the provided function.
pub fn parse_lines<T, F, E>(lines: &str, f: F) -> Result<Vec<T>, E>
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

/// Convert lines of strings to 2d Vector of digits
///
/// # Errors
///
/// Will return `None` if characters are not valid digits under given radix
#[must_use]
pub fn lines_as_digits_radix<T>(lines: &str, radix: u32) -> Option<V2<T>>
where
    T: From<u32>,
{
    let mut result = vec![];
    for row in split_lines_trim(lines) {
        let mut row_vec = vec![];
        for c in row.chars() {
            let digit = c.to_digit(radix)?;
            row_vec.push(digit.into());
        }
        result.push(row_vec);
    }
    Some(result)
}

/// Treats lines as 2d Vector of digits where each character i
///
/// # Errors
///
/// Will return `None` if characters are not valid digits under given radix
#[must_use]
pub fn lines_as_digits<T>(lines: &str) -> Option<V2<T>>
where
    T: From<u32>,
{
    lines_as_digits_radix(lines, 10)
}

/// Separates lines to blocks on empty lines
#[must_use]
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
