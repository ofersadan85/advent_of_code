use crate::{default_input_path, Solver};
use std::collections::HashMap;

struct Quest06 {
    part: u8,
}

impl Solver<'_> for Quest06 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut count = 0;
        let mut map: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, c) in input.chars().enumerate() {
            match c {
                'A'..='Z' => map.entry(c).or_default().push(i),
                'a' => count += map.entry(c.to_ascii_uppercase()).or_default().len(),
                'b'..='z' if self.part != 1 => {
                    count += map.entry(c.to_ascii_uppercase()).or_default().len();
                }
                _ => (),
            }
        }
        count
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expect_solution;

    #[test]
    fn part_1() {
        let solver = Quest06 { part: 1 };
        expect_solution!(solver, 0, 5);
        expect_solution!(solver, 1, 135);
    }

    #[test]
    fn part_2() {
        let solver = Quest06 { part: 2 };
        expect_solution!(solver, 0, 11);
        expect_solution!(solver, 2, 3916);
    }
}
