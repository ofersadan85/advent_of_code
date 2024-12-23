pub use crate::grid::{Direction, DxDy, GridCell};

/// A 2D point with x and y coordinates.
/// The number type is `isize` to allow for negative coordinates, or minus operations.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

/// A trait for types that have x and y (2D) coordinates.
pub trait Coords {
    /// Get the x (horizontal) coordinate.
    fn x(&self) -> isize;

    /// Get the y (vertical) coordinate.
    fn y(&self) -> isize;

    /// Get the coordinates as an (x, y) tuple.
    fn coords(&self) -> (isize, isize) {
        (self.x(), self.y())
    }

    /// Get the coordinates as a 2d Point (which also implements Coords).
    fn as_point(&self) -> Point {
        Point {
            x: self.x(),
            y: self.y(),
        }
    }

    /// Get the Manhattan distance to another Coords.
    /// This is the sum of the absolute differences, i.e. `dx.abs()` + `dy.abs()`.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords};
    /// let point = Point::default();
    /// let other = Point { x: 4, y: 6 };
    /// assert_eq!(point.manhattan_distance(&other), 10);
    /// ```
    fn manhattan_distance(&self, other: &dyn Coords) -> isize {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }

    /// Get the Euclidean distance to another Coords.
    /// This is the square root of the sum of the squared differences, i.e. sqrt(dx^2 + dy^2).
    ///
    /// # Note
    /// Values are cast to f64 before the square root is taken.
    /// This might cause a loss of precision for very large numbers.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords};
    /// let point = Point::default();
    /// let other = Point { x: 3, y: 4 };
    /// assert!(point.distance(&other) - 5.0 < f64::EPSILON);
    /// ```
    #[allow(clippy::cast_precision_loss)]
    fn distance(&self, other: &dyn Coords) -> f64 {
        let dx = (self.x() - other.x()) as f64;
        let dy = (self.y() - other.y()) as f64;
        dx.hypot(dy)
    }

    /// Get the [`Direction`] to another Coords.
    ///
    /// Will only return one of the 8 cardinal or diagonal directions, or None if the points are the same.
    ///
    /// For example, if the other point is to the right and up, this will return `Some(Direction::NorthEast)`.
    ///
    /// As with all [`Direction`] methods, `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords, Direction};
    /// let point = Point::default();
    /// let other = Point { x: 3, y: 4 };
    /// assert_eq!(point.direction_to(&other), Some(Direction::SouthEast));
    /// ```
    fn direction_to(&self, other: &dyn Coords) -> Option<Direction> {
        Direction::from_dxdy(other.x() - self.x(), other.y() - self.y())
    }

    /// Get the closest neighbor coordinates in a given [`Direction`].
    ///
    /// As with all [`Direction`] methods, `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords, Direction};
    /// let point = Point::default();
    /// assert_eq!(point.neighbor_at(&Direction::East), Point { x: 1, y: 0 });
    /// assert_eq!(point.neighbor_at(&Direction::NorthWest), Point { x: -1, y: -1 });
    /// ```
    fn neighbor_at(&self, direction: &Direction) -> Point {
        Point {
            x: self.x() + direction.dx(),
            y: self.y() + direction.dy(),
        }
    }

    /// Get the coordinates of the 4 closest orthogonal (cardinal direction) neighbors.
    ///
    /// Specifically, the order will always be clockwise starting from `North`,
    /// i.e. `[North, East, South, West]` where `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords, Direction};
    /// let point = Point::default();
    /// let manual_neighbors = [
    ///    point.neighbor_at(&Direction::North),
    ///    point.neighbor_at(&Direction::East),
    ///    point.neighbor_at(&Direction::South),
    ///    point.neighbor_at(&Direction::West),
    /// ];
    /// assert_eq!(point.neighbors_orthogonal(), manual_neighbors);
    /// ```
    fn neighbors_orthogonal(&self) -> [Point; 4] {
        let mut neighbors = [Point::default(); 4];
        neighbors.copy_from_slice(
            &Direction::orthogonal()
                .iter()
                .map(|dir| self.neighbor_at(dir))
                .collect::<Vec<_>>(),
        );
        neighbors
    }

    /// Get the coordinates of the 4 closest diagonal neighbors.
    ///
    /// Specifically, the order will always be clockwise starting from `NorthEast`,
    /// i.e. `[NorthEast, SouthEast, SouthWest, NorthWest]`.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords, Direction};
    /// let point = Point::default();
    /// let manual_neighbors = [
    ///   point.neighbor_at(&Direction::NorthEast),
    ///   point.neighbor_at(&Direction::SouthEast),
    ///   point.neighbor_at(&Direction::SouthWest),
    ///   point.neighbor_at(&Direction::NorthWest),
    /// ];
    /// assert_eq!(point.neighbors_diagonal(), manual_neighbors);
    /// ```
    fn neighbors_diagonal(&self) -> [Point; 4] {
        let mut neighbors = [Point::default(); 4];
        neighbors.copy_from_slice(
            &Direction::diagonal()
                .iter()
                .map(|dir| self.neighbor_at(dir))
                .collect::<Vec<_>>(),
        );
        neighbors
    }

    /// Get the coordinates of the 8 closest neighbors surrounding this point.
    ///
    /// Specifically, the order will always be clockwise starting from `North`,
    /// i.e. `[North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest]`.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords, Direction};
    /// let point = Point::default();
    /// let manual_neighbors = [
    ///   point.neighbor_at(&Direction::North),
    ///   point.neighbor_at(&Direction::NorthEast),
    ///   point.neighbor_at(&Direction::East),
    ///   point.neighbor_at(&Direction::SouthEast),
    ///   point.neighbor_at(&Direction::South),
    ///   point.neighbor_at(&Direction::SouthWest),
    ///   point.neighbor_at(&Direction::West),
    ///   point.neighbor_at(&Direction::NorthWest),
    /// ];
    /// assert_eq!(point.neighbors(), manual_neighbors);
    /// ```
    fn neighbors(&self) -> [Point; 8] {
        let mut neighbors = [Point::default(); 8];
        neighbors.copy_from_slice(
            &Direction::all()
                .iter()
                .map(|dir| self.neighbor_at(dir))
                .collect::<Vec<_>>(),
        );
        neighbors
    }

    /// Get the coordinates of the neighbors in a box of radius `n` around this point.
    ///
    /// The order will always be from the `NorthWest` corner, row by row, to the `SouthEast` corner.
    ///
    /// As with all [`Direction`] methods, `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords};
    /// let box_n = Point::default().neighbors_box_n(2);
    /// assert_eq!(box_n.len(), 25);
    /// assert_eq!(box_n[0], Point { x: -2, y: -2 });
    /// assert_eq!(box_n[24], Point { x: 2, y: 2 });
    /// ```
    #[allow(clippy::cast_sign_loss)]
    fn neighbors_box_n(&self, n: isize) -> Vec<Point> {
        let size = 2 * n + 1;
        let mut neighbors = Vec::with_capacity((size * size) as usize);
        for y in -n..=n {
            for x in -n..=n {
                neighbors.push(Point {
                    x: self.x() + x,
                    y: self.y() + y,
                });
            }
        }
        neighbors
    }

    /// Get the coordinates of the neighbors in a box of radius 1 around this point.
    ///
    /// A convenience method that calls [`Self::neighbors_box_n`] with `n = 1` but returns an array instead of a `Vec`.
    ///
    /// The order will always be from the `NorthWest` corner, row by row, to the `SouthEast` corner.
    /// i.e. `[NorthWest, North, NorthEast, West, Center, East, SouthWest, South, SouthEast]`.
    ///
    /// ```text
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords};
    /// let box_n = Point::default().neighbors_box();
    /// assert_eq!(box_n.len(), 9);
    /// assert_eq!(box_n.as_slice(), point.neighbors_box_n(1).as_slice());
    /// ```
    fn neighbors_box(&self) -> [Point; 9] {
        let mut neighbors = [Point::default(); 9];
        neighbors.copy_from_slice(&self.neighbors_box_n(1));
        neighbors
    }
}

impl<P> Coords for P
where
    Point: From<P>,
    P: Copy,
{
    fn x(&self) -> isize {
        Point::from(*self).x
    }

    fn y(&self) -> isize {
        Point::from(*self).y
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

impl<N> From<(N, N)> for Point
where
    isize: From<N>,
{
    fn from(coords: (N, N)) -> Self {
        Self {
            x: isize::from(coords.0),
            y: isize::from(coords.1),
        }
    }
}

impl<N> From<&(N, N)> for Point
where
    isize: From<N>,
    N: Copy,
{
    fn from(coords: &(N, N)) -> Self {
        Self {
            x: isize::from(coords.0),
            y: isize::from(coords.1),
        }
    }
}

impl<N> From<Point> for (N, N)
where
    N: From<isize>,
{
    fn from(point: Point) -> Self {
        (N::from(point.x), N::from(point.y))
    }
}

impl<N> From<&Point> for (N, N)
where
    N: From<isize>,
{
    fn from(point: &Point) -> Self {
        (N::from(point.x), N::from(point.y))
    }
}

impl<N> From<[N; 2]> for Point
where
    isize: From<N>,
{
    fn from(coords: [N; 2]) -> Self {
        let [x, y] = coords;
        Self {
            x: isize::from(x),
            y: isize::from(y),
        }
    }
}

impl<N> From<&[N; 2]> for Point
where
    isize: From<N>,
    N: Copy,
{
    fn from(coords: &[N; 2]) -> Self {
        Self {
            x: isize::from(coords[0]),
            y: isize::from(coords[1]),
        }
    }
}

impl<N> From<Point> for [N; 2]
where
    N: From<isize>,
{
    fn from(point: Point) -> Self {
        [N::from(point.x), N::from(point.y)]
    }
}

impl<N> From<&Point> for [N; 2]
where
    N: From<isize>,
{
    fn from(point: &Point) -> Self {
        [N::from(point.x), N::from(point.y)]
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
