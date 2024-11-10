use advent_of_code_common::math::simple_series_sum;
use euclid::{Box2D, Point2D};
use itertools::iproduct;

const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";
const PATH: &str = "../inputs/2021/day17.txt";

fn input(example: bool) -> Box2D<i32, u32> {
    let data = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    };
    let mut split = data.split_ascii_whitespace().rev();
    let (y_low, y_high) = split.next().unwrap()[2..].split_once("..").unwrap();
    let (x_low, x_high) = split.next().unwrap()[2..].split_once("..").unwrap();
    Box2D::new(
        Point2D::new(x_low.parse().unwrap(), y_low.parse().unwrap()),
        Point2D::new(
            x_high.trim_end_matches(',').parse().unwrap(),
            y_high.parse().unwrap(),
        ),
    )
}

fn launch(x: i32, y: i32, target: &Box2D<i32, u32>) -> bool {
    let mut position: Point2D<i32, u32> = Point2D::origin();
    let mut vx = x;
    let mut vy = y;
    loop {
        position.x += vx;
        position.y += vy;
        vy -= 1;
        if vx > 0 {
            vx -= 1;
        }
        if (target.min.x..=target.max.x).contains(&position.x)
            && (target.min.y..=target.max.y).contains(&position.y)
            || target.max.x < position.x
            || target.min.y > position.y
        {
            break;
        }
    }
    (target.min.x..=target.max.x).contains(&position.x)
        && (target.min.y..=target.max.y).contains(&position.y)
}

fn part_1(target: &Box2D<i32, u32>) -> i32 {
    simple_series_sum(target.min.y.abs() - 1)
}

fn part_2(target: &Box2D<i32, u32>) -> usize {
    iproduct!(0..=target.max.x, target.min.y..=target.min.y.abs())
        .filter(|&(x, y)| launch(x, y, target))
        .count()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 45);
}

#[test]
fn task_1() {
    assert_eq!(part_1(&input(false)), 4656);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 112);
}

#[test]
fn task_2() {
    assert_eq!(part_2(&input(false)), 1908);
}
