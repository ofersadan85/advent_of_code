use anyhow::{Context, Result};

fn input(example: bool) -> Result<Vec<(usize, usize, usize, usize)>> {
    const PATH: &str = "inputs/day04.txt";
    let s = if example {
        "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).context("Failed to read input file")?
    };
    let result = s.trim()
    .lines()
    .filter_map(|row| {
        let (range1, range2) = row.trim().split_once(',')?;
        let (min1, max1) = range1.split_once('-')?;
        let (min2, max2) = range2.split_once('-')?;
        Some((
            min1.parse().ok()?,
            max1.parse().ok()?,
            min2.parse().ok()?,
            max2.parse().ok()?,
        ))
    })
    .collect();
    Ok(result)
}

fn part_1(data: &[(usize, usize, usize, usize)]) -> usize {
    data.iter()
        .filter(|(min1, max1, min2, max2)| {
            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count()
}
fn part_2(data: &[(usize, usize, usize, usize)]) -> usize {
    data.iter()
        .filter(|(min1, max1, min2, max2)| {
            (min1 <= min2 && max1 >= max2)
                || (min2 <= min1 && max2 >= max1)
                || (max1 >= min2 && max1 <= max2)
                || (min1 >= min2 && min1 <= max2)
        })
        .count()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap()), 2);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false).unwrap()), 513);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()), 4);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false).unwrap()), 878);
}
