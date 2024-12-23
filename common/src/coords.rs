/// A 2D point with x and y coordinates.
///
/// The number type is `isize` to allow for negative coordinates, or minus operations.
///
/// This struct implements [`Coords`] so it can be used in grid algorithms,
/// and anything that implements [`Coords`] can be converted to a `Point` using `.as_point()`.
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

/// An implementation of [`PartialOrd`] and [`Ord`] is required to use [`Point`] as a key in a [`BTreeMap`].
///
/// The ordering is first by `y` coordinate, then by `x` coordinate.
///
/// Incidentally, this also facilitates printing a grid of points in the correct order.
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
    }
}

/// An implementation of [`PartialOrd`] and [`Ord`] is required to use [`Point`] as a key in a [`BTreeMap`].
///
/// The ordering is first by `y` coordinate, then by `x` coordinate.
///
/// Incidentally, this also facilitates printing a grid of points in the correct order.
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
        let diff = (other.x() - self.x(), other.y() - self.y());
        Direction::try_from(diff.as_point()).ok()
    }

    /// Get the neighbor coordinates in a given [`Direction`], at distance `n`.
    ///
    /// As with all [`Direction`] methods, `x = 0` is to the west of positive x, and `y = 0` is to the north of positive y.
    ///
    /// # Example
    /// ```rust
    /// use advent_of_code_common::coords::{Point, Coords, Direction};
    /// let point = Point::default();
    /// assert_eq!(point.neighbor_at_n(&Direction::East, 2), Point { x: 2, y: 0 });
    /// assert_eq!(point.neighbor_at_n(&Direction::NorthWest, 2), Point { x: -2, y: -2 });
    /// ```
    fn neighbor_at_n(&self, direction: &Direction, n: isize) -> Point {
        Point {
            x: self.x() + direction.x() * n,
            y: self.y() + direction.y() * n,
        }
    }

    /// Get the closest neighbor coordinates in a given [`Direction`].
    ///
    /// This is equivalent to calling `self.neighbor_at_n(direction, 1)`.
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
        self.neighbor_at_n(direction, 1)
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

/// An enum representing the 8 cardinal and diagonal directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "↑"),
            Self::South => write!(f, "↓"),
            Self::East => write!(f, "→"),
            Self::West => write!(f, "←"),
            Self::NorthEast => write!(f, "↗"),
            Self::NorthWest => write!(f, "↖"),
            Self::SouthEast => write!(f, "↘"),
            Self::SouthWest => write!(f, "↙"),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' | 'N' | '↑' | '^' => Ok(Self::North),
            'D' | 'S' | '↓' | 'v' => Ok(Self::South),
            'L' | 'E' | '→' | '>' => Ok(Self::East),
            'R' | 'W' | '←' | '<' => Ok(Self::West),
            '↗' => Ok(Self::NorthEast),
            '↖' => Ok(Self::NorthWest),
            '↘' => Ok(Self::SouthEast),
            '↙' => Ok(Self::SouthWest),
            _ => Err("Invalid character in input"),
        }
    }
}

impl Direction {
    /// Get the 4 cardinal (orthogonal) directions, clockwise starting from `North`.
    pub const fn orthogonal() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    /// Get the 4 diagonal directions, clockwise starting from `NorthEast`.
    pub const fn diagonal() -> [Self; 4] {
        [
            Self::NorthEast,
            Self::SouthEast,
            Self::SouthWest,
            Self::NorthWest,
        ]
    }

    /// Get all 8 cardinal and diagonal directions, clockwise starting from `North`.
    pub const fn all() -> [Self; 8] {
        [
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
    }
}

impl Direction {
    #[must_use]
    pub const fn turn_cw_45(&self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::South => Self::SouthWest,
            Self::East => Self::SouthEast,
            Self::West => Self::NorthWest,
            Self::NorthEast => Self::East,
            Self::NorthWest => Self::North,
            Self::SouthEast => Self::South,
            Self::SouthWest => Self::West,
        }
    }

    #[must_use]
    pub const fn turn_cw_90(&self) -> Self {
        self.turn_cw_45().turn_cw_45()
    }

    #[must_use]
    pub const fn turn_180(&self) -> Self {
        self.turn_cw_90().turn_cw_90()
    }

    #[must_use]
    pub const fn turn_cw_270(&self) -> Self {
        self.turn_180().turn_cw_90()
    }
}

impl From<Direction> for Point {
    fn from(direction: Direction) -> Self {
        let x = match direction {
            Direction::North | Direction::South => 0,
            Direction::East | Direction::NorthEast | Direction::SouthEast => 1,
            Direction::West | Direction::NorthWest | Direction::SouthWest => -1,
        };
        let y = match direction {
            Direction::North | Direction::NorthEast | Direction::NorthWest => -1,
            Direction::South | Direction::SouthEast | Direction::SouthWest => 1,
            Direction::East | Direction::West => 0,
        };
        Self { x, y }
    }
}

impl From<&Direction> for Point {
    fn from(direction: &Direction) -> Self {
        Point::from(*direction)
    }
}

impl TryFrom<Point> for Direction {
    type Error = ();
    fn try_from(value: Point) -> Result<Self, Self::Error> {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match (value.x.cmp(&0), value.y.cmp(&0)) {
            (Less, Less) => Ok(Self::NorthWest),
            (Less, Equal) => Ok(Self::West),
            (Less, Greater) => Ok(Self::SouthWest),
            (Equal, Less) => Ok(Self::North),
            (Equal, Equal) => Err(()),
            (Equal, Greater) => Ok(Self::South),
            (Greater, Less) => Ok(Self::NorthEast),
            (Greater, Equal) => Ok(Self::East),
            (Greater, Greater) => Ok(Self::SouthEast),
        }
    }
}

impl TryFrom<&Point> for Direction {
    type Error = <Self as TryFrom<Point>>::Error;
    fn try_from(value: &Point) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

impl<C: Coords> std::ops::Add<C> for Point {
    type Output = Point;

    fn add(self, rhs: C) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
        }
    }
}

impl<C: Coords> std::ops::Add<C> for &Point {
    type Output = Point;

    fn add(self, rhs: C) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
        }
    }
}

impl<C: Coords> std::ops::AddAssign<C> for Point {
    fn add_assign(&mut self, rhs: C) {
        self.x += rhs.x();
        self.y += rhs.y();
    }
}

impl<C: Coords> std::ops::Sub<C> for Point {
    type Output = Point;

    fn sub(self, rhs: C) -> Self::Output {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
        }
    }
}

impl<C: Coords> std::ops::Sub<C> for &Point {
    type Output = Point;

    fn sub(self, rhs: C) -> Self::Output {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
        }
    }
}

impl<C: Coords> std::ops::SubAssign<C> for Point {
    fn sub_assign(&mut self, rhs: C) {
        self.x -= rhs.x();
        self.y -= rhs.y();
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> std::ops::Mul<T> for Point
where
    T: Into<isize>,
{
    type Output = Point;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> std::ops::Mul<T> for &Point
where
    T: Into<isize>,
{
    type Output = Point;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> std::ops::MulAssign<T> for Point
where
    T: Into<isize>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> std::ops::Div<T> for Point
where
    T: Into<isize>,
{
    type Output = Point;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> std::ops::Div<T> for &Point
where
    T: Into<isize>,
{
    type Output = Point;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> std::ops::DivAssign<T> for Point
where
    T: Into<isize>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> std::ops::Mul<T> for Direction
where
    T: Into<isize>,
{
    type Output = Point;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Point {
            x: self.x() * rhs,
            y: self.y() * rhs,
        }
    }
}

impl<T> std::ops::Mul<T> for &Direction
where
    T: Into<isize>,
{
    type Output = Point;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Point {
            x: self.x() * rhs,
            y: self.y() * rhs,
        }
    }
}
