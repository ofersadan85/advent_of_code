use anyhow::Result;
use std::collections::HashSet;

pub const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", value)),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { x: 0, y: 1 },
            Direction::Down => Self { x: 0, y: -1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
        }
    }
}

impl std::ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

type Path = Vec<(Vec2, String)>;

fn parse_input(input: &str) -> Result<(Path, Vec<Vec2>, Vec<Vec2>)> {
    use Direction::{Down, Left, Right, Up};
    let mut path = Path::new();
    let mut a = Vec::new();
    let mut b = Vec::new();
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next()?.chars().next()?;
            let direction = Direction::try_from(direction).ok()?;
            let distance = parts.next()?.parse::<i32>().ok()?;
            let color = parts.next()?;
            Some((direction, distance, color))
        })
        .for_each(|(direction, distance, color)| {
            let mut distance = distance;
            let direction_v: Vec2 = direction.into();
            while distance > 0 {
                let last_point = path.last().map(|(pos, _)| *pos).unwrap_or_default();
                let next_point = last_point + direction_v;
                let (other_a, other_b) = match direction {
                    Up => (next_point + Left.into(), next_point + Right.into()),
                    Down => (next_point + Right.into(), next_point + Left.into()),
                    Left => (next_point + Down.into(), next_point + Up.into()),
                    Right => (next_point + Up.into(), next_point + Down.into()),
                };
                path.push((next_point, color.to_string()));
                a.push(other_a);
                b.push(other_b);
                distance -= 1;
            }
        });
    Ok((path, a, b))
}

fn flood_fill_sides(path: &Path, a: &[Vec2], b: &[Vec2], print: bool) -> usize {
    use Direction::{Down, Left, Right, Up};
    let mut visited_a: HashSet<Vec2> = path.iter().map(|(pos, _)| *pos).collect();
    let mut queue_a: Vec<Vec2> = a.iter().copied().collect();
    let mut visited_b: HashSet<Vec2> = path.iter().map(|(pos, _)| *pos).collect();
    let mut queue_b: Vec<Vec2> = b.iter().copied().collect();
    loop {
        if let Some(a) = queue_a.pop() {
            if !visited_a.contains(&a) {
                visited_a.insert(a);
                let a_neighbors = [
                    a + Up.into(),
                    a + Down.into(),
                    a + Left.into(),
                    a + Right.into(),
                ];
                for neighbor in &a_neighbors {
                    if !visited_a.contains(neighbor) {
                        queue_a.push(*neighbor);
                    }
                }
            }
        } else {
            break;
        }

        if let Some(b) = queue_b.pop() {
            if !visited_b.contains(&b) {
                visited_b.insert(b);
                let b_neighbors = [
                    b + Up.into(),
                    b + Down.into(),
                    b + Left.into(),
                    b + Right.into(),
                ];
                for neighbor in &b_neighbors {
                    if !visited_b.contains(neighbor) {
                        queue_b.push(*neighbor);
                    }
                }
            }
        } else {
            break;
        }
    }
    if print {
        print_path(&path, &visited_a, &visited_b);
    }
    if queue_a.is_empty() {
        visited_a.len()
    } else {
        visited_b.len()
    }
}

fn print_path(path: &Path, a: &HashSet<Vec2>, b: &HashSet<Vec2>) {
    let min_x = path.iter().map(|(pos, _)| pos.x).min().unwrap_or_default();
    let max_x = path.iter().map(|(pos, _)| pos.x).max().unwrap_or_default();
    let min_y = path.iter().map(|(pos, _)| pos.y).min().unwrap_or_default();
    let max_y = path.iter().map(|(pos, _)| pos.y).max().unwrap_or_default();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let pos = Vec2 { x, y };
            if let Some(_) = path.iter().find(|(p, _)| *p == pos) {
                print!("#");
            } else if a.contains(&pos) {
                print!("A");
            } else if b.contains(&pos) {
                print!("B");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn loop_area(input: &str) -> Result<usize> {
    let (path, a, b) = parse_input(input)?;
    let result = flood_fill_sides(&path, &a, &b, false);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = loop_area(EXAMPLE).unwrap();
        assert_eq!(result, 62);
    }

    #[test]
    fn part1() {
        let input = include_str!("day18.txt");
        let result = loop_area(input).unwrap();
        assert_eq!(result, 62500);
    }
}
