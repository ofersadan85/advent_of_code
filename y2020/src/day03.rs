use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct PositionVector {
    x: usize,
    y: usize,
}

impl Add for PositionVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn input(example: bool) -> Vec<Vec<bool>> {
    const PATH: &str = "../inputs/2020/day03.txt";
    if example {
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .lines()
    .map(|row| row.trim().chars().map(|c| c == '#').collect())
    .collect()
}

fn count_trees(data: &[Vec<bool>], step: PositionVector) -> usize {
    let mut mover = PositionVector { x: 0, y: 0 };
    let mut tree_counter = 0;
    let row_len = data[0].len();
    while mover.y < data.len() {
        if mover.x >= row_len {
            mover.x %= row_len;
        }
        if data[mover.y][mover.x] {
            tree_counter += 1;
        }
        mover = mover + step;
    }
    tree_counter
}

fn part_1(data: &[Vec<bool>]) -> usize {
    let step = PositionVector { x: 3, y: 1 };
    count_trees(data, step)
}

fn part_2(data: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|s| count_trees(data, PositionVector { x: s.0, y: s.1 }))
        .product()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 7);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 294);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 336);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 5_774_564_250);
}
