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

fn input(example: bool) -> Vec<Vec<char>> {
    const PATH: &str = "inputs/day02.txt";
    if example {
        "A Y
        B X
        C Z"
        .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split('\n')
    .map(|row| row.trim().chars().collect())
    .collect()
}

fn part_1(data: &[Vec<char>]) -> usize {
    data.iter()
        .map(|row| {
            let left = GameChoice::new(*row.first().unwrap());
            let right = GameChoice::new(*row.last().unwrap());
            GameResult::play(left, right).value()
        })
        .sum()
}

fn part_2(data: &[Vec<char>]) -> usize {
    use GameResult::*;
    data.iter()
        .map(|row| {
            let left = GameChoice::new(*row.first().unwrap());
            let right = GameChoice::Unknown;
            match row.last().unwrap() {
                'X' => LeftWins { left, right },
                'Y' => Draw { left, right },
                'Z' => RightWins { left, right },
                _ => panic!("Can't cheat without knowing the expected result"),
            }
            .cheat()
            .value()
        })
        .sum()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 15);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 11841);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 12);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 13022);
}
