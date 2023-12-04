#[derive(Debug)]
struct BoardingPass {
    row: u8,
    column: u8,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Back,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'F' => Self::Forward,
            'B' => Self::Back,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid input direction"),
        }
    }
}

impl BoardingPass {
    fn new(s: &str) -> Self {
        let directions: Vec<Direction> = s.trim().chars().map(Direction::from_char).collect();
        let mut min: u8 = 0;
        let mut max: u8 = 128;
        for dir in &directions[..7] {
            let new_diff = max.abs_diff(min) / 2;
            if *dir == Direction::Back {
                min += new_diff;
            } else {
                max -= new_diff;
            }
        }
        let row = if directions[6] == Direction::Back {
            max - 1
        } else {
            min
        };
        min = 0;
        max = 8;
        for dir in &directions[7..] {
            let new_diff = max.abs_diff(min) / 2;
            if *dir == Direction::Right {
                min += new_diff;
            } else {
                max -= new_diff;
            }
        }
        let column = if directions[9] == Direction::Right {
            max - 1
        } else {
            min
        };
        Self { row, column }
    }

    const fn seat_id(&self) -> usize {
        self.row as usize * 8 + self.column as usize
    }
}

fn input(example: bool) -> Vec<BoardingPass> {
    const PATH: &str = "inputs/day05.txt";
    if example {
        "FBFBBFFRLR
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split('\n')
    .map(BoardingPass::new)
    .collect()
}

fn part_1(data: &[BoardingPass]) -> usize {
    data.iter().map(BoardingPass::seat_id).max().unwrap()
}

fn part_2(data: &[BoardingPass]) -> usize {
    let mut all_ids: Vec<usize> = data.iter().map(BoardingPass::seat_id).collect();
    all_ids.sort_unstable();
    for window in all_ids.windows(2) {
        if window[0] != window[1] - 1 {
            return window[0] + 1;
        }
    }
    0
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 820);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 965);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 524);
}
