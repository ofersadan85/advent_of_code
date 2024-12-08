#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PositionedCell<T, D = ()> {
    pub x: isize,
    pub y: isize,
    pub state: T,
    pub data: D,
}

impl<T, D> PositionedCell<T, D> where D: Default{
    pub fn new(x: isize, y: isize, state: T) -> Self {
        Self {
            x,
            y,
            state,
            data: D::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T, D = ()> {
    pub x_range: std::ops::Range<isize>,
    pub y_range: std::ops::Range<isize>,
    pub cells: Vec<PositionedCell<T, D>>,
}

impl<T, D> std::str::FromStr for Grid<T, D>
where
    T: TryFrom<char>,
    D: Default,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for (y, line) in s.lines().enumerate() {
            height += 1;
            let mut line_width = 0;
            for (x, c) in line.trim().chars().enumerate() {
                line_width += 1;
                let x = isize::try_from(x).map_err(|_| "Invalid x coordinate")?;
                let y = isize::try_from(y).map_err(|_| "Invalid y coordinate")?;
                let state = T::try_from(c).map_err(|_| "Invalid character in input")?;
                cells.push(PositionedCell::new(x, y, state));
            }
            width = width.max(line_width);
        }
        Ok(Self {
            x_range: 0..width,
            y_range: 0..height,
            cells,
        })
    }
}

impl<T, D> Grid<T, D>
where
    T: Copy,
{
    fn index_of(&self, x: isize, y: isize) -> Option<usize> {
        if !self.x_range.contains(&x) || !self.y_range.contains(&y) {
            return None;
        }
        usize::try_from(y * self.x_range.end + x).ok()
    }

    pub fn get(&self, x: isize, y: isize) -> Option<T> {
        self.get_cell(x, y).map(|c| c.state)
    }

    pub fn get_cell(&self, x: isize, y: isize) -> Option<&PositionedCell<T, D>> {
        self.cells.get(self.index_of(x, y)?)
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        self.get_cell_mut(x, y).map(|c| &mut c.state)
    }

    pub fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut PositionedCell<T, D>> {
        let index = self.index_of(x, y)?;
        self.cells.get_mut(index)
    }

    pub fn set(&mut self, x: isize, y: isize, state: T) {
        if let Some(index) = self.index_of(x, y) {
            self.cells[index].state = state;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cells.iter().map(|c| &c.state)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells.iter_mut().map(|c| &mut c.state)
    }

    pub fn apply_steps_until(
        &mut self,
        f: impl Fn(&Self) -> Self,
        limit: Option<usize>,
    ) -> (Self, usize)
    where
        T: PartialEq,
        D: PartialEq,
    {
        let mut steps = 0;
        loop {
            let new_grid = f(self);
            steps += 1;
            if new_grid == *self || limit.map_or(false, |limit| steps >= limit) {
                return (new_grid, steps);
            }
            *self = new_grid;
        }
    }

    pub fn apply_step(&mut self, f: impl Fn(&Self) -> Self) -> bool
    where
        T: PartialEq,
        D: PartialEq,
    {
        let new_grid = f(self);
        let changed = new_grid != *self;
        *self = new_grid;
        changed
    }

    pub fn neighbors_orthogonal_n(&self, x: isize, y: isize, n: isize) -> [Option<T>; 4] {
        [
            self.get(x, y - n), // up
            self.get(x + n, y), // right
            self.get(x, y + n), // down
            self.get(x - n, y), // left
        ]
    }

    pub fn neighbors_diagonal_n(&self, x: isize, y: isize, n: isize) -> [Option<T>; 4] {
        [
            self.get(x - n, y - n), // up-left
            self.get(x + n, y - n), // up-right
            self.get(x + n, y + n), // down-right
            self.get(x - n, y + n), // down-left
        ]
    }

    pub fn neighbors_n(&self, x: isize, y: isize, n: isize) -> [Option<T>; 8] {
        let mut neighbors = [None; 8];
        neighbors[0..4].copy_from_slice(&self.neighbors_orthogonal_n(x, y, n));
        neighbors[4..8].copy_from_slice(&self.neighbors_diagonal_n(x, y, n));
        neighbors
    }

    pub fn neighbors_orthogonal(&self, x: isize, y: isize) -> [Option<T>; 4] {
        self.neighbors_orthogonal_n(x, y, 1)
    }

    pub fn neighbors_diagonal(&self, x: isize, y: isize) -> [Option<T>; 4] {
        self.neighbors_diagonal_n(x, y, 1)
    }

    pub fn neighbors(&self, x: isize, y: isize) -> [Option<T>; 8] {
        self.neighbors_n(x, y, 1)
    }

    pub fn neighbors_box_n(&self, x: isize, y: isize, n: isize) -> Vec<Option<T>> {
        let mut neighbors = Vec::new();
        for dy in -n..=n {
            for dx in -n..=n {
                neighbors.push(self.get(x + dx, y + dy));
            }
        }
        neighbors
    }

    pub fn neighbors_box(&self, x: isize, y: isize) -> [Option<T>; 9] {
        let mut neighbors = [None; 9];
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                neighbors[count] = self.get(x + dx, y + dy);
                count += 1;
            }
        }
        neighbors
    }

    pub fn count_state(&self, state: T) -> usize
    where
        T: PartialEq,
    {
        self.cells.iter().filter(|c| c.state == state).count()
    }

    pub fn count_neighbors(&self, x: isize, y: isize, state: T) -> usize
    where
        T: PartialEq,
    {
        self.neighbors(x, y)
            .iter()
            .filter(|&&s| s == Some(state))
            .count()
    }

    pub fn sight_line(&self, x: isize, y: isize, dx: isize, dy: isize, blocks: &[T]) -> Vec<T>
    where
        T: PartialEq,
    {
        let mut result = Vec::new();
        let mut x = x + dx;
        let mut y = y + dy;
        while let Some(state) = self.get(x, y) {
            result.push(state);
            if blocks.contains(&state) {
                break;
            }
            x += dx;
            y += dy;
        }
        result
    }

    pub fn sight_lines_all(&self, x: isize, y: isize, blocks: &[T]) -> Vec<Vec<T>>
    where
        T: PartialEq,
    {
        let mut result = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                result.push(self.sight_line(x, y, dx, dy, blocks));
            }
        }
        result
    }

    pub fn sight_lines_edges(&self, x: isize, y: isize, blocks: &[T]) -> Vec<Option<T>>
    where
        T: PartialEq,
    {
        let mut edges = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                edges.push(self.sight_line(x, y, dx, dy, blocks).last().copied());
            }
        }
        edges
    }

    pub fn sight_line_cells(
        &self,
        x: isize,
        y: isize,
        dx: isize,
        dy: isize,
        blocks: &[T],
    ) -> Vec<&PositionedCell<T, D>>
    where
        T: PartialEq,
    {
        let mut result = Vec::new();
        let mut x = x + dx;
        let mut y = y + dy;
        while let Some(cell) = self.get_cell(x, y) {
            result.push(cell);
            if blocks.contains(&cell.state) {
                break;
            }
            x += dx;
            y += dy;
        }
        result
    }

    pub fn sight_lines_all_cells(&self, x: isize, y: isize, blocks: &[T]) -> Vec<Vec<&PositionedCell<T, D>>>
    where
        T: PartialEq,
    {
        let mut result = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                result.push(self.sight_line_cells(x, y, dx, dy, blocks));
            }
        }
        result
    }

    pub fn sight_lines_edges_cells(&self, x: isize, y: isize, blocks: &[T]) -> Vec<Option<&PositionedCell<T, D>>>
    where
        T: PartialEq,
    {
        let mut edges = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                edges.push(self.sight_line_cells(x, y, dx, dy, blocks).last().copied());
            }
        }
        edges
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_range.clone() {
            for x in self.x_range.clone() {
                let c = self.get(x, y).ok_or(std::fmt::Error)?;
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn grid_from_str_test() {
        let grid: Grid<State> = EXAMPLE.parse().unwrap();
        assert_eq!(grid.x_range, 0..10);
        assert_eq!(grid.y_range, 0..10);
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn test_neighbors() {
        use State::*;
        let grid: Grid<State> = EXAMPLE.parse().unwrap();
        assert_eq!(
            grid.neighbors(0, 0),
            [
                None,        // up
                Some(Floor), // right
                Some(Empty), // down
                None,        // left
                None,        // up-left
                None,        // up-right
                Some(Empty), // down-right
                None,        // down-left
            ]
        );
        assert_eq!(
            grid.neighbors(5, 5),
            [
                Some(Empty), // up
                Some(Empty), // right
                Some(Floor), // down
                Some(Empty), // left
                Some(Floor), // up-left
                Some(Empty), // up-right
                Some(Floor), // down-right
                Some(Empty)  // down-left
            ]
        );
        assert_eq!(
            grid.neighbors(9, 9),
            [
                Some(Empty), // up
                None,        // right
                None,        // down
                Some(Empty), // left
                Some(Floor), // up-left
                None,        // up-right
                None,        // down-right
                None,        // down-left
            ]
        );
    }

    #[test]
    fn test_box() {
        let example = "123\n456\n789";
        let grid: Grid<char> = example.parse().unwrap();
        assert_eq!(
            grid.neighbors_box(1, 1),
            "123456789".chars().map(Some).collect::<Vec<_>>().as_slice()
        );
        assert_eq!(
            grid.neighbors_box(0, 0)
                .iter()
                .map(|c| c.unwrap_or_default())
                .collect::<String>(),
            "\0\0\0\012\045"
        );
        assert_eq!(
            grid.neighbors_box_n(1, 1, 2)
                .iter()
                .map(|c| c.unwrap_or_default())
                .collect::<String>(),
            "\0\0\0\0\0\0123\0\0456\0\0789\0\0\0\0\0\0"
        )
    }
}
