pub use crate::coords::{Coords, Direction, Point};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GridCell<T = char> {
    pub point: Point,
    pub data: T,
}

impl<T> GridCell<T> {
    pub fn new(c: &dyn Coords, data: T) -> Self {
        Self {
            point: c.as_point(),
            data,
        }
    }
}

impl<T: Default> Default for GridCell<T> {
    fn default() -> Self {
        Self {
            point: Point::default(),
            data: T::default(),
        }
    }
}

impl<T> From<GridCell<T>> for Point {
    fn from(cell: GridCell<T>) -> Self {
        Self {
            x: cell.point.x,
            y: cell.point.y,
        }
    }
}

impl<T> From<&GridCell<T>> for Point {
    fn from(cell: &GridCell<T>) -> Self {
        Self {
            x: cell.point.x,
            y: cell.point.y,
        }
    }
}

impl<T> Coords for GridCell<T> {
    fn x(&self) -> isize {
        self.point.x
    }

    fn y(&self) -> isize {
        self.point.y
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Grid<T = char> {
    pub x_range: std::ops::Range<isize>,
    pub y_range: std::ops::Range<isize>,
    pub cells: BTreeMap<Point, GridCell<T>>,
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = BTreeMap<Point, GridCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl<T> std::str::FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Self {
            x_range: 0..0,
            y_range: 0..0,
            cells: BTreeMap::new(),
        };
        let mut height = 0;
        let mut width = 0;
        for (y, line) in s.lines().enumerate() {
            height += 1;
            let mut line_width = 0;
            for (x, c) in line.chars().enumerate() {
                line_width += 1;
                let x = isize::try_from(x).map_err(|_| "Invalid x coordinate")?;
                let y = isize::try_from(y).map_err(|_| "Invalid y coordinate")?;
                let point = (x, y).as_point();
                let data = T::try_from(c).map_err(|_| "Invalid character in input")?;
                grid.insert(point, GridCell::new(&point, data));
            }
            width = width.max(line_width);
        }
        grid.x_range = 0..width;
        grid.y_range = 0..height;
        Ok(grid)
    }
}

impl<T> Grid<T> {
    pub fn new(width: isize, height: isize, data: T) -> Self
    where
        T: Clone,
    {
        let mut grid = Self {
            x_range: 0..width,
            y_range: 0..height,
            cells: BTreeMap::new(),
        };
        for y in 0..height {
            for x in 0..width {
                let point = (x, y).as_point();
                grid.insert(point, GridCell::new(&point, data.clone()));
            }
        }
        grid.x_range = 0..width;
        grid.y_range = 0..height;
        grid
    }

    pub fn new_default(width: isize, height: isize) -> Self
    where
        T: Default,
    {
        let mut grid = Self {
            x_range: 0..width,
            y_range: 0..height,
            cells: BTreeMap::new(),
        };
        for y in 0..height {
            for x in 0..width {
                let point = (x, y).as_point();
                grid.cells
                    .insert(point, GridCell::new(&point, T::default()));
            }
        }
        grid.x_range = 0..width;
        grid.y_range = 0..height;
        grid
    }

    pub const fn width(&self) -> isize {
        self.x_range.end - self.x_range.start
    }

    pub const fn height(&self) -> isize {
        self.y_range.end - self.y_range.start
    }

    pub fn get(&self, c: &dyn Coords) -> Option<&GridCell<T>> {
        self.cells.get(&c.as_point())
    }

    pub fn get_mut(&mut self, c: &dyn Coords) -> Option<&mut GridCell<T>> {
        self.cells.get_mut(&c.as_point())
    }

    pub fn get_wrapped(&self, c: &dyn Coords) -> &GridCell<T> {
        let x = (self.width() + c.x() % self.width()) % self.width();
        let y = (self.height() + c.y() % self.height()) % self.height();
        self.get(&(x, y)).expect("Wrapped cell")
    }

    pub fn set(&mut self, c: &dyn Coords, data: T) {
        if let Some(cell) = self.get_mut(c) {
            cell.data = data;
        } else {
            let point = c.as_point();
            self.insert(point, GridCell::new(&point, data));
        }
    }

    pub fn apply_steps_until(&mut self, f: impl Fn(&Self) -> Self, limit: Option<usize>) -> usize
    where
        T: PartialEq,
    {
        let mut steps = 0;
        loop {
            let new_grid = f(self);
            steps += 1;
            if new_grid == *self || limit.map_or(false, |limit| steps >= limit) {
                *self = new_grid;
                return steps;
            }
            *self = new_grid;
        }
    }

    pub fn apply_step(&mut self, f: impl Fn(&Self) -> Self) -> bool
    where
        T: PartialEq,
    {
        let new_grid = f(self);
        let changed = new_grid != *self;
        *self = new_grid;
        changed
    }

    /// Get the neighbor cell in a given [`Direction`], at distance `n`.
    ///
    /// As with all [`Direction`] methods, `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    pub fn neighbor_at_n(
        &self,
        c: &dyn Coords,
        direction: &Direction,
        n: isize,
    ) -> Option<&GridCell<T>> {
        self.get(&c.neighbor_at_n(direction, n))
    }

    /// Get the closest neighbor cell in a given [`Direction`].
    ///
    /// This is equivalent to calling `self.neighbor_at_n(direction, 1)`.
    ///
    /// As with all [`Direction`] methods, `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    pub fn neighbor_at(&self, c: &dyn Coords, direction: &Direction) -> Option<&GridCell<T>> {
        self.neighbor_at_n(c, direction, 1)
    }

    pub fn neighbors_orthogonal_n(&self, c: &dyn Coords, n: isize) -> [Option<&GridCell<T>>; 4] {
        [
            self.neighbor_at_n(c, &Direction::North, n),
            self.neighbor_at_n(c, &Direction::East, n),
            self.neighbor_at_n(c, &Direction::South, n),
            self.neighbor_at_n(c, &Direction::West, n),
        ]
    }

    pub fn neighbors_orthogonal(&self, c: &dyn Coords) -> [Option<&GridCell<T>>; 4] {
        self.neighbors_orthogonal_n(c, 1)
    }

    pub fn neighbors_diagonal_n(&self, c: &dyn Coords, n: isize) -> [Option<&GridCell<T>>; 4] {
        [
            self.neighbor_at_n(c, &Direction::NorthEast, n),
            self.neighbor_at_n(c, &Direction::SouthEast, n),
            self.neighbor_at_n(c, &Direction::SouthWest, n),
            self.neighbor_at_n(c, &Direction::NorthWest, n),
        ]
    }

    pub fn neighbors_diagonal(&self, c: &dyn Coords) -> [Option<&GridCell<T>>; 4] {
        self.neighbors_diagonal_n(c, 1)
    }

    pub fn neighbors_n(&self, c: &dyn Coords, n: isize) -> [Option<&GridCell<T>>; 8] {
        let mut neighbors = [None; 8];
        neighbors[0..4].copy_from_slice(&self.neighbors_orthogonal_n(c, n));
        neighbors[4..8].copy_from_slice(&self.neighbors_diagonal_n(c, n));
        neighbors
    }

    pub fn neighbors(&self, c: &dyn Coords) -> [Option<&GridCell<T>>; 8] {
        self.neighbors_n(c, 1)
    }

    pub fn neighbors_box_n(&self, c: &dyn Coords, n: isize) -> Vec<Option<&GridCell<T>>> {
        let mut neighbors = Vec::new();
        for dy in -n..=n {
            for dx in -n..=n {
                neighbors.push(self.get(&(c.x() + dx, c.y() + dy)));
            }
        }
        neighbors
    }

    pub fn neighbors_box(&self, c: &dyn Coords) -> [Option<&GridCell<T>>; 9] {
        let mut neighbors = [None; 9];
        neighbors.copy_from_slice(&self.neighbors_box_n(c, 1));
        neighbors
    }

    pub fn count_data(&self, data: &T) -> usize
    where
        T: PartialEq,
    {
        self.cells.values().filter(|c| &c.data == data).count()
    }

    pub fn count_neighbors(&self, c: &dyn Coords, data: &T) -> usize
    where
        T: PartialEq,
    {
        self.neighbors(c)
            .iter()
            .flatten()
            .filter(|c| &c.data == data)
            .count()
    }

    pub fn sight_line(
        &self,
        c: &dyn Coords,
        direction: &Direction,
        blocks: &[T],
    ) -> Vec<&GridCell<T>>
    where
        T: PartialEq,
    {
        let mut result = Vec::new();
        let mut point = c.neighbor_at(direction);
        while let Some(cell) = self.get(&point) {
            result.push(cell);
            if blocks.contains(&cell.data) {
                break;
            }
            point = point.neighbor_at(direction);
        }
        result
    }

    pub fn sight_lines_all(&self, c: &dyn Coords, blocks: &[T]) -> Vec<Vec<&GridCell<T>>>
    where
        T: PartialEq,
    {
        Direction::all()
            .iter()
            .map(|d| self.sight_line(c, d, blocks))
            .collect()
    }

    pub fn sight_lines_edges(&self, c: &dyn Coords, blocks: &[T]) -> Vec<&GridCell<T>>
    where
        T: PartialEq,
    {
        Direction::all()
            .iter()
            .filter_map(|d| self.sight_line(c, d, blocks).last().copied())
            .collect()
    }

    pub fn sight_line_n(
        &self,
        c: &dyn Coords,
        direction: &Direction,
        n: usize,
    ) -> Vec<&GridCell<T>> {
        let mut result = Vec::new();
        let mut point = c.as_point();
        while let Some(cell) = self.get(&point) {
            result.push(cell);
            if result.len() >= n {
                break;
            }
            point = point.neighbor_at(direction);
        }
        result
    }

    pub fn sight_line_wrapped(
        &self,
        c: &dyn Coords,
        direction: &Direction,
        n: usize,
    ) -> Vec<&GridCell<T>> {
        let mut result = Vec::new();
        for i in 0..n {
            let point = c.neighbor_at_n(direction, i as isize); // TODO: usize -> isize
            result.push(self.get_wrapped(&point));
        }
        result
    }

    #[allow(clippy::range_plus_one)] // Can't use inclusive ranges here
    pub fn expand(&mut self)
    where
        T: Default,
    {
        self.x_range = self.x_range.start - 1..self.x_range.end + 1;
        self.y_range = self.y_range.start - 1..self.y_range.end + 1;
        for x in self.x_range.clone() {
            let p = (x, self.y_range.start).as_point();
            self.insert(p, GridCell::new(&p, T::default()));
            let p = (x, self.y_range.end - 1).as_point();
            self.insert(p, GridCell::new(&p, T::default()));
        }
        for y in self.y_range.clone() {
            let p = (self.x_range.start, y).as_point();
            self.insert(p, GridCell::new(&p, T::default()));
            let p = (self.x_range.end - 1, y).as_point();
            self.insert(p, GridCell::new(&p, T::default()));
        }
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_row = self.y_range.start;
        for cell in self.cells.values() {
            if cell.point.y > current_row {
                writeln!(f)?;
                current_row = cell.point.y;
            }
            write!(f, "{}", cell.data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trim_lines;
    const EXAMPLE: &str = "L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L..###
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL";

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum State {
        Floor,
        Empty,
        Occupied,
    }

    impl TryFrom<char> for State {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(State::Floor),
                'L' => Ok(State::Empty),
                '#' => Ok(State::Occupied),
                _ => Err("Invalid character in input"),
            }
        }
    }

    #[test]
    fn grid_from_str() {
        let grid: Grid<State> = trim_lines(EXAMPLE).parse().unwrap();
        assert_eq!(grid.x_range, 0..10);
        assert_eq!(grid.y_range, 0..10);
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn neighbors() {
        use State::*;
        let grid: Grid<State> = trim_lines(EXAMPLE).parse().unwrap();
        assert_eq!(
            grid.neighbors(&(0_isize, 0))
                .iter()
                .map(|c| c.map(|c| c.data))
                .collect::<Vec<_>>(),
            [
                None,        // up
                Some(Floor), // right
                Some(Empty), // down
                None,        // left
                None,        // up-right
                Some(Empty), // down-right
                None,        // down-left
                None,        // up-left
            ]
        );
        assert_eq!(
            grid.neighbors(&(5_isize, 5))
                .iter()
                .map(|c| c.map(|c| c.data))
                .collect::<Vec<_>>(),
            [
                Some(Empty), // up
                Some(Empty), // right
                Some(Floor), // down
                Some(Empty), // left
                Some(Empty), // up-right
                Some(Floor), // down-right
                Some(Empty), // down-left
                Some(Floor), // up-left
            ]
        );
        assert_eq!(
            grid.neighbors(&(9_isize, 9))
                .iter()
                .map(|c| c.map(|c| c.data))
                .collect::<Vec<_>>(),
            [
                Some(Empty), // up
                None,        // right
                None,        // down
                Some(Empty), // left
                None,        // up-right
                None,        // down-right
                None,        // down-left
                Some(Floor), // up-left
            ]
        );
    }

    #[test]
    fn neighbors_box() {
        let example = "123\n456\n789";
        let grid: Grid<char> = example.parse().unwrap();
        assert_eq!(
            grid.neighbors_box(&(1_isize, 1))
                .iter()
                .flatten()
                .map(|c| c.data)
                .collect::<String>(),
            "123456789"
        );
        assert_eq!(
            grid.neighbors_box(&(0_isize, 0))
                .iter()
                .map(|c| c.map(|c| c.data).unwrap_or_default())
                .collect::<String>(),
            "\0\0\0\012\045"
        );
        assert_eq!(
            grid.neighbors_box_n(&(1_isize, 1), 2)
                .iter()
                .map(|c| c.map(|c| c.data).unwrap_or_default())
                .collect::<String>(),
            "\0\0\0\0\0\0123\0\0456\0\0789\0\0\0\0\0\0"
        )
    }

    #[test]
    fn wrapped_grid() {
        let example = "123\n456\n789";
        let grid: Grid<char> = example.parse().unwrap();
        assert_eq!(grid.get_wrapped(&(-1_isize, -1)).data, '9');
        assert_eq!(grid.get_wrapped(&(-2_isize, -1)).data, '8');
        assert_eq!(grid.get_wrapped(&(-3_isize, -1)).data, '7');
        assert_eq!(grid.get_wrapped(&(-4_isize, -1)).data, '9');
        assert_eq!(grid.get_wrapped(&(3_isize, 1)).data, '4');
        assert_eq!(grid.get_wrapped(&(4_isize, 1)).data, '5');
        assert_eq!(grid.get_wrapped(&(5_isize, 1)).data, '6');
        assert_eq!(grid.get_wrapped(&(6_isize, 1)).data, '4');

        let n = grid.sight_line_wrapped(&(1_isize, 1), &Direction::North, 4);
        assert_eq!(n.iter().map(|c| c.data).collect::<String>(), "5285");
        let n = grid.sight_line_wrapped(&(1_isize, 1), &Direction::West, 4);
        assert_eq!(n.iter().map(|c| c.data).collect::<String>(), "5465");
        let n = grid.sight_line_wrapped(&(1_isize, 1), &Direction::East, 4);
        assert_eq!(n.iter().map(|c| c.data).collect::<String>(), "5645");
        let n = grid.sight_line_wrapped(&(1_isize, 1), &Direction::South, 4);
        assert_eq!(n.iter().map(|c| c.data).collect::<String>(), "5825");
    }
}
