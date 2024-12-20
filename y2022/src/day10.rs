use advent_of_code_common::file::split_lines_trim;
use anyhow::{Context, Result};

const PATH: &str = "../inputs/2022/day10.txt";
const EXAMPLE: &str = "../inputs/2022/day10_example.txt";
const EXPECTED_EXAMPLE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
const EXPECTED_PART_2: &str = "###...##..###..#..#.####.#..#.####...##.
...#.#..#.#..#.#.#..#....#.#..#.......#.
...#.#..#.#..#.##...###..##...###.....#.
###..####.###..#.#..#....#.#..#.......#.
.....#..#.#....#.#..#....#.#..#....#..#.
.....#..#.#....#..#.#....#..#.####..##.."; // PAPKFKEJ

fn input(example: bool) -> Result<Vec<String>> {
    let path = if example { EXAMPLE } else { PATH };
    let s = std::fs::read_to_string(path).context("Failed to read input file")?;
    Ok(split_lines_trim(&s))
}

#[allow(clippy::cast_sign_loss)] // TODO: Find a better alternative
fn calc_sprite_position(data: &[String]) -> [usize; 240] {
    let mut cycles: [i16; 240] = [0; 240]; // 240 refers to part_2 (40 * 6)
    let mut count = 0;
    cycles[0] = 1;
    for (i, row) in data.iter().enumerate() {
        if row.starts_with("add") {
            count += 1;
            cycles[i + count] = row
                .split_ascii_whitespace()
                .last()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
        }
    }
    let mut result: [usize; 240] = [0; 240];
    for i in 0..240 {
        result[i] = cycles[..i].iter().sum::<i16>() as usize;
    }
    result
}

fn part_1(data: &[String]) -> usize {
    let sprite_positions = calc_sprite_position(data);
    sprite_positions[19..220]
        .iter()
        .step_by(40)
        .enumerate()
        .map(|(index, n)| (index * 40 + 20) * n)
        .sum()
}

fn part_2(data: &[String]) -> String {
    let mut result = String::new();
    let sprite_positions = calc_sprite_position(data);
    for (i, &p) in sprite_positions.iter().enumerate() {
        let idx = i % 40;
        if idx + 1 == p || idx == p || (idx > 0 && idx - 1 == p) {
            result.push('#');
        } else {
            result.push('.');
        }
        if i % 40 == 39 && i < 239 {
            result.push('\n');
        }
    }
    result
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap()), 13140);
}

#[test]
fn task_1() {
    assert_eq!(part_1(&input(false).unwrap()), 14060);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()), EXPECTED_EXAMPLE);
}

#[test]
fn task_2() {
    let output = part_2(&input(false).unwrap());
    assert_eq!(output, EXPECTED_PART_2, "\n{output}\n");
}
