use itertools::{iproduct, Product};
use std::{num::ParseIntError, ops::RangeInclusive};

#[derive(Debug)]
enum CustomError {
    ParseIntError,
    InvalidPoint,
    InvalidRect,
    InvalidAction,
}

impl From<ParseIntError> for CustomError {
    fn from(_: ParseIntError) -> Self {
        CustomError::ParseIntError
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = CustomError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // "461,550"
        let mut words = s.split(',');
        let x = match words.next() {
            Some(p) => p.parse()?,
            None => return Err(CustomError::InvalidPoint),
        };
        let y = match words.next() {
            Some(y) => y.parse()?,
            None => return Err(CustomError::InvalidPoint),
        };
        Ok(Point { x, y })
    }
}

struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl TryFrom<&str> for Rect {
    type Error = CustomError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // "461,550 through 564,900"
        let mut words = s.split_whitespace();
        let top_left = match words.next() {
            Some(p) => Point::try_from(p)?,
            None => return Err(CustomError::InvalidPoint),
        };
        match words.next() {
            Some("through") => (),
            _ => return Err(CustomError::InvalidRect),
        };
        let bottom_right = match words.next() {
            Some(x) => Point::try_from(x)?,
            None => return Err(CustomError::InvalidPoint),
        };
        Ok(Rect {
            top_left,
            bottom_right,
        })
    }
}

struct GridOnOff {
    lights: [[bool; 1000]; 1000],
}

trait LightGrid {
    fn new() -> Self;
    fn get_product(&self, rect: &Rect) -> Product<RangeInclusive<usize>, RangeInclusive<usize>> {
        let from = Point {
            x: rect.top_left.x.min(rect.bottom_right.x),
            y: rect.top_left.y.min(rect.bottom_right.y),
        };
        let to = Point {
            x: rect.top_left.x.max(rect.bottom_right.x),
            y: rect.top_left.y.max(rect.bottom_right.y),
        };
        iproduct!(from.x..=to.x, from.y..=to.y)
    }
    fn toggle(&mut self, rect: &Rect);
    fn turn_up(&mut self, rect: &Rect);
    fn turn_down(&mut self, rect: &Rect);
    fn count_visible(&self) -> usize;
}

impl LightGrid for GridOnOff {
    fn new() -> Self {
        GridOnOff {
            lights: [[false; 1000]; 1000],
        }
    }

    fn toggle(&mut self, rect: &Rect) {
        self.get_product(rect).for_each(|(x, y)| {
            self.lights[x][y] = !self.lights[x][y];
        });
    }

    fn turn_up(&mut self, rect: &Rect) {
        self.get_product(rect).for_each(|(x, y)| {
            self.lights[x][y] = true;
        });
    }

    fn turn_down(&mut self, rect: &Rect) {
        self.get_product(rect).for_each(|(x, y)| {
            self.lights[x][y] = false;
        });
    }

    fn count_visible(&self) -> usize {
        self.lights.iter().flatten().filter(|&&x| x).count()
    }
}

enum Action {
    Toggle(Rect),
    TurnUp(Rect),
    TurnDown(Rect),
}

impl TryFrom<&str> for Action {
    type Error = CustomError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // "toggle 461,550 through 564,900"
        // or
        // "turn on 370,39 through 425,839"
        // or
        // "turn off 464,858 through 833,915"
        let mut words = s.split_whitespace();
        let unknown_error = Err(CustomError::InvalidAction);
        let action = match words.next() {
            Some("toggle") => "toggle",
            Some("turn") => match words.next() {
                Some("on") => "on",
                Some("off") => "off",
                _ => return unknown_error,
            },
            _ => return unknown_error,
        };
        let rest = words.collect::<Vec<_>>().join(" ");
        match action {
            "toggle" => Ok(Action::Toggle(Rect::try_from(rest.as_str())?)),
            "on" => Ok(Action::TurnUp(Rect::try_from(rest.as_str())?)),
            "off" => Ok(Action::TurnDown(Rect::try_from(rest.as_str())?)),
            _ => unknown_error,
        }
    }
}

struct GridBrightness {
    // TODO: Convert to another type that can be used with adding 2 * 300 times
    // u8 seems to be enough by chance
    lights: [[u8; 1000]; 1000],
}

impl LightGrid for GridBrightness {
    fn new() -> Self {
        GridBrightness {
            lights: [[0; 1000]; 1000],
        }
    }

    fn toggle(&mut self, rect: &Rect) {
        self.get_product(rect).for_each(|(x, y)| {
            self.lights[x][y] += 2;
        });
    }

    fn turn_up(&mut self, rect: &Rect) {
        self.get_product(rect).for_each(|(x, y)| {
            self.lights[x][y] += 1;
        });
    }

    fn turn_down(&mut self, rect: &Rect) {
        self.get_product(rect).for_each(|(x, y)| {
            self.lights[x][y] = self.lights[x][y].saturating_sub(1);
        });
    }

    fn count_visible(&self) -> usize {
        self.lights.iter().flatten().map(|&x| x as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid = GridOnOff::new();
        assert_eq!(grid.count_visible(), 0);
        let rect = Rect {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 999, y: 999 },
        };
        grid.turn_up(&rect);
        assert_eq!(grid.count_visible(), 1_000_000);
        let rect = Rect {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 999, y: 0 },
        };
        grid.toggle(&rect);
        assert_eq!(grid.count_visible(), 999_000);
        let rect = Rect {
            top_left: Point { x: 499, y: 499 },
            bottom_right: Point { x: 500, y: 500 },
        };
        grid.turn_down(&rect);
        assert_eq!(grid.count_visible(), 998_996);
    }

    #[test]
    fn part_1() {
        let input = include_str!("day06.txt");
        let mut grid = GridOnOff::new();
        input.lines().for_each(|line| {
            let action = Action::try_from(line).unwrap();
            match action {
                Action::Toggle(rect) => grid.toggle(&rect),
                Action::TurnUp(rect) => grid.turn_up(&rect),
                Action::TurnDown(rect) => grid.turn_down(&rect),
            }
        });
        assert_eq!(grid.count_visible(), 400410);
    }

    #[test]
    fn test_grid_brightness() {
        let mut grid = GridBrightness::new();
        assert_eq!(grid.lights.iter().flatten().count(), 1_000_000);
        assert_eq!(grid.count_visible(), 0);
        let rect = Rect {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 1, y: 1 },
        };
        grid.turn_up(&rect);
        assert_eq!(grid.count_visible(), 4);
        let rect = Rect {
            top_left: Point { x: 3, y: 0 },
            bottom_right: Point { x: 4, y: 0 },
        };
        grid.toggle(&rect);
        assert_eq!(grid.count_visible(), 8);
        let rect = Rect {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 10, y: 10 },
        };
        grid.turn_down(&rect);
        assert_eq!(grid.count_visible(), 2);
    }

    #[test]
    fn part_2() {
        let input = include_str!("day06.txt");
        let mut grid = GridBrightness::new();
        input.lines().for_each(|line| {
            let action = Action::try_from(line).unwrap();
            match action {
                Action::Toggle(rect) => grid.toggle(&rect),
                Action::TurnUp(rect) => grid.turn_up(&rect),
                Action::TurnDown(rect) => grid.turn_down(&rect),
            }
        });
        assert_eq!(grid.count_visible(), 15343601);
    }
}
