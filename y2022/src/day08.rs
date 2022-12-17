use advent_of_code_common::{file::lines_as_digits, v2::V2};
use itertools::iproduct;

const PATH: &str = "inputs/day08.txt";
const EXAMPLE: &str = "30373
25512
65332
33549
35390";

fn input(example: bool) -> V2<u32> {
    if example {
        lines_as_digits(EXAMPLE)
    } else {
        lines_as_digits(&std::fs::read_to_string(PATH).unwrap())
    }
    .unwrap()
}

fn is_visible(data: &V2<u32>, x: usize, y: usize) -> bool {
    let (h, w) = (data.len(), data[0].len());
    let current_height = data[y][x];
    if x == 0 || x + 1 == w || y == 0 || y + 1 == h {
        return true;
    } else if current_height == 0 {
        return false;
    }
    let left = (0..x).all(|i| data[y][i] < current_height);
    let right = ((x + 1)..w).all(|i| data[y][i] < current_height);
    let up = (0..y).all(|i| data[i][x] < current_height);
    let down = ((y + 1)..h).all(|i| data[i][x] < current_height);
    left || right || up || down
}

fn view_distance(data: &V2<u32>, x: usize, y: usize) -> usize {
    let (h, w) = (data.len(), data[0].len());
    let current_height = data[y][x];
    if x == 0 || x + 1 == w || y == 0 || y + 1 == h {
        return 0;
    } else if current_height == 0 {
        return 1;
    }
    let mut left = (0..x)
        .rev()
        .take_while(|&i| data[y][i] < current_height)
        .count();
    let mut right = ((x + 1)..w)
        .take_while(|&i| data[y][i] < current_height)
        .count();
    let mut up = (0..y)
        .rev()
        .take_while(|&i| data[i][x] < current_height)
        .count();
    let mut down = ((y + 1)..h)
        .take_while(|&i| data[i][x] < current_height)
        .count();

    left = if x - left > 1 { left + 1 } else { left };
    right = if x + right < w - 1 { right + 1 } else { right };
    up = if y - up > 1 { up + 1 } else { up };
    down = if y + down < h - 1 { down + 1 } else { down };
    left * right * up * down
}

fn part_1(data: &V2<u32>) -> usize {
    let (h, w) = (data.len(), data[0].len());
    iproduct!(0..h, 0..w)
        .filter(|&(y, x)| is_visible(data, x, y))
        .count()
}

fn part_2(data: &V2<u32>) -> usize {
    let (h, w) = (data.len(), data[0].len());
    let (best_y, best_x) = iproduct!(0..h, 0..w)
        .max_by_key(|&(y, x)| view_distance(data, x, y))
        .unwrap();
    view_distance(data, best_x, best_y)
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 21);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 1736);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 8);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 268_800);
}
