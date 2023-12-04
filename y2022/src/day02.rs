use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameChoice {
    Unknown,
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    fn new(c: char) -> Self {
        use GameChoice::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("Illegal choice"),
        }
    }

    const fn value(self) -> usize {
        use GameChoice::*;
        match self {
            Unknown => 0,
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum GameResult {
    LeftWins { left: GameChoice, right: GameChoice },
    Draw { left: GameChoice, right: GameChoice },
    RightWins { left: GameChoice, right: GameChoice },
}

impl GameResult {
    fn play(left: GameChoice, right: GameChoice) -> Self {
        use GameChoice::*;
        use GameResult::*;
        if left == right {
            Draw { right, left }
        } else {
            match (&left, &right) {
                (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => LeftWins { right, left },
                (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => RightWins { right, left },
                _ => panic!("Can't play with unknown choices"),
            }
        }
    }

    fn cheat(&self) -> Self {
        use GameChoice::*;
        use GameResult::*;
        match self {
            Draw { left, right: _ } => Draw {
                left: *left,
                right: *left,
            },
            LeftWins { left, right: _ } => LeftWins {
                left: *left,
                right: match left {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                    Unknown => panic!("Can't cheat against an unknown"),
                },
            },
            RightWins { left, right: _ } => RightWins {
                left: *left,
                right: match left {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                    Unknown => panic!("Can't cheat against an unknown"),
                },
            },
        }
    }

    const fn value(&self) -> usize {
        use GameResult::*;
        match self {
            LeftWins { left: _, right } => right.value(),
            Draw { left: _, right } => 3 + right.value(),
            RightWins { left: _, right } => 6 + right.value(),
        }
    }
}

fn input(example: bool) -> Result<Vec<Vec<char>>> {
    const PATH: &str = "inputs/day02.txt";
    let s = if example {
        "A Y
        B X
        C Z"
        .to_string()
    } else {
        std::fs::read_to_string(PATH).context("Failed to read input file")?
    };
    let result = s
        .trim()
        .lines()
        .map(|row| row.trim().chars().collect())
        .collect();
    Ok(result)
}

fn part_1(data: &[Vec<char>]) -> usize {
    data.iter()
        .filter_map(|row| {
            let left = GameChoice::new(*row.first()?);
            let right = GameChoice::new(*row.last()?);
            Some(GameResult::play(left, right).value())
        })
        .sum()
}

fn part_2(data: &[Vec<char>]) -> usize {
    use GameResult::*;
    data.iter()
        .filter_map(|row| {
            let left = GameChoice::new(*row.first().unwrap());
            let right = GameChoice::Unknown;
            match row.last() {
                Some('X') => Some(LeftWins { left, right }),
                Some('Y') => Some(Draw { left, right }),
                Some('Z') => Some(RightWins { left, right }),
                _ => None, // Can't cheat without knowing the expected result
            }
        })
        .map(|result| result.cheat().value())
        .sum()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap()), 15);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false).unwrap()), 11841);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()), 12);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false).unwrap()), 13022);
}
