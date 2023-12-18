use anyhow::{anyhow, Result};
use tracing::instrument;

pub const EXAMPLE1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

pub const EXAMPLE1_ROTATED: &str = "#.##..#
..##...
##..###
#....#.
.#..#.#
.#..#.#
#....#.
##..###
..##...";

pub const EXAMPLE2: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

fn reflection_h(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().collect();
    for index in 0..lines.len() - 1 {
        let mut inspected_index = index;
        let mut reflected_index = index + 1;
        while lines[inspected_index] == lines[reflected_index] {
            if inspected_index == 0 || reflected_index == lines.len() - 1 {
                return Some(index + 1);
            }

            inspected_index -= 1;
            reflected_index += 1;
        }
    }
    None
}

#[instrument(skip_all, level = "debug")]
fn reflection_h_smudged(input: &str) -> Option<usize> {
    let previous_value = reflection_h(input);
    let lines: Vec<_> = input.lines().collect();
    let mut smudge_found = false;
    for index in 0..lines.len() - 1 {
        let mut inspected_index = index;
        let mut reflected_index = index + 1;
        loop {
            let diff_count = lines[inspected_index]
                .chars()
                .zip(lines[reflected_index].chars())
                .filter(|(a, b)| a != b)
                .count();
            match (diff_count, smudge_found) {
                (0, _) => {
                    if (inspected_index == 0 || reflected_index == lines.len() - 1)
                        && previous_value != Some(index + 1)
                        && smudge_found
                    {
                        return Some(index + 1);
                    }
                }
                (1, false) => {
                    // There's one difference, and it's the first one we've found
                    smudge_found = true;
                    if (inspected_index == 0 || reflected_index == lines.len() - 1)
                        && previous_value != Some(index + 1)
                        && smudge_found
                    {
                        return Some(index + 1);
                    }
                }
                _ => {
                    smudge_found = false;
                    break;
                }
            }
            if inspected_index == 0 || reflected_index == lines.len() - 1 {
                break;
            }
            inspected_index -= 1;
            reflected_index += 1;
        }
    }
    None
}

fn rotate_right(input: &str) -> Result<String> {
    let lines: Vec<_> = input.lines().collect();
    let mut rotated = String::new();
    for index in 0..lines[0].len() {
        for line in &lines {
            let c = line
                .chars()
                .nth(index)
                .ok_or_else(|| anyhow!("Index out of bounds"))?;
            rotated.push(c);
        }
        if index != lines[0].len() - 1 {
            rotated.push('\n');
        }
    }
    Ok(rotated)
}

fn reflection_v(input: &str, smudged: bool) -> Option<usize> {
    let rotated = rotate_right(input).ok()?;
    if smudged {
        reflection_h_smudged(rotated.as_str())
    } else {
        reflection_h(rotated.as_str())
    }
}

#[instrument(skip_all, level = "debug")]
fn box_value(input: &str, smudged: bool) -> usize {
    let value = if smudged {
        reflection_h_smudged(input)
    } else {
        reflection_h(input)
    };
    value.map_or_else(
        || reflection_v(input, smudged).unwrap_or(0),
        |index| index * 100,
    )
}

#[instrument(skip(input), level = "debug")]
pub fn mirrors(input: &str, smudged: bool) -> usize {
    let splitter = if input.contains("\n\n") {
        "\n\n"
    } else {
        "\r\n\r\n"
    };
    input
        .split(splitter)
        .map(|box_str| box_value(box_str, smudged))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal() {
        assert_eq!(reflection_h(EXAMPLE1), None);
        assert_eq!(reflection_h(EXAMPLE2), Some(4));
    }

    #[test]
    fn rotate() {
        assert_eq!(rotate_right(EXAMPLE1).unwrap(), EXAMPLE1_ROTATED);
    }

    #[test]
    fn vertical() {
        assert_eq!(reflection_v(EXAMPLE1, false), Some(5));
        assert_eq!(reflection_v(EXAMPLE2, false), None);
    }

    #[test]
    fn value() {
        assert_eq!(box_value(EXAMPLE1, false), 5);
        assert_eq!(box_value(EXAMPLE2, false), 400);
        let both = format!("{}\n\n{}", EXAMPLE1, EXAMPLE2);
        assert_eq!(mirrors(&both, false), 405);
    }

    #[test]
    fn part1() {
        assert_eq!(mirrors(include_str!("day13.txt"), false), 30802);
    }

    #[test]
    fn part2_example() {
        assert_eq!(mirrors(EXAMPLE1, true), 300);
        assert_eq!(mirrors(EXAMPLE1_ROTATED, true), 3);
        assert_eq!(mirrors(EXAMPLE2, true), 100);
    }

    #[test]
    fn part2() {
        assert_eq!(mirrors(include_str!("day13.txt"), true), 37876);
    }
}
