struct Point {
    x: i64,
    y: i64,
}

impl std::str::FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Self {
                x: x.trim_start_matches(['p', 'v', '='])
                    .parse()
                    .map_err(|_| "invalid x")?,
                y: y.parse().map_err(|_| "invalid y")?,
            }),
            _ => Err("invalid point"),
        }
    }
}

struct Robot {
    position: Point,
    velocity: Point,
}

impl std::str::FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split_once(' ') {
            Some((position, velocity)) => Ok(Self {
                position: position.parse()?,
                velocity: velocity.parse()?,
            }),
            _ => Err("invalid robot"),
        }
    }
}

fn get_positions(robots: &[Robot], width: i64, height: i64, steps: i64) -> Vec<Point> {
    let positions: Vec<Point> = robots
        .iter()
        .map(|robot| {
            let mut x = (robot.position.x + robot.velocity.x * steps) % width;
            if x < 0 {
                x += width;
            }
            let mut y = (robot.position.y + robot.velocity.y * steps) % height;
            if y < 0 {
                y += height;
            }
            Point { x, y }
        })
        .collect();
    positions
}

fn get_quadrants(robots: &[Robot], width: i64, height: i64, steps: i64) -> i64 {
    let positions = get_positions(robots, width, height, steps);
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    for position in positions {
        let in_top = position.y < height / 2;
        let in_left = position.x < width / 2;
        let in_bottom = position.y > height / 2;
        let in_right = position.x > width / 2;
        match (in_top, in_left, in_bottom, in_right) {
            (true, true, false, false) => top_left += 1,
            (true, false, false, true) => top_right += 1,
            (false, true, true, false) => bottom_left += 1,
            (false, false, true, true) => bottom_right += 1,
            _ => (),
        }
    }
    top_left * top_right * bottom_left * bottom_right
}

fn grid_str(positions: &[Point], width: i64, height: i64) -> String {
    let width = usize::try_from(width).expect("invalid width");
    let height = usize::try_from(height).expect("invalid height");
    let mut grid = vec![vec![' '; width]; height];
    for position in positions {
        let x = usize::try_from(position.x).expect("invalid x");
        let y = usize::try_from(position.y).expect("invalid y");
        grid[y][x] = '#';
    }
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

#[allow(clippy::maybe_infinite_iter)]
fn find_tree(robots: &[Robot], width: i64, height: i64) -> usize {
    (0..)
        .map(|i| grid_str(&get_positions(robots, width, height, i), width, height))
        .position(|grid| grid.contains("###############################"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_macros::read_input;
    const EXAMPLE: &str = "p=0,4 v=3,-3
                           p=6,3 v=-1,-3
                           p=10,3 v=-1,2
                           p=2,0 v=2,-1
                           p=0,0 v=1,3
                           p=3,0 v=-2,-2
                           p=7,6 v=-1,-3
                           p=3,0 v=-1,-2
                           p=9,3 v=2,3
                           p=7,3 v=-1,2
                           p=2,4 v=2,-3
                           p=9,5 v=-3,-3";

    #[test]
    fn example_1() {
        let robots = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Robot>, _>>()
            .unwrap();
        assert_eq!(get_quadrants(&robots, 11, 7, 100), 12);
    }

    #[test]
    fn part_1() {
        read_input!();
        let robots = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Robot>, _>>()
            .unwrap();
        assert_eq!(get_quadrants(&robots, 101, 103, 100), 224554908);
    }

    #[test]
    fn part_2() {
        read_input!();
        let robots = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Robot>, _>>()
            .unwrap();
        assert_eq!(find_tree(&robots, 101, 103), 6644);
    }
}
