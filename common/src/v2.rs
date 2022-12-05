use itertools::iproduct;
use num::{Integer, One, Zero};

/// A 2 dimensional Vector
pub type V2<T> = Vec<Vec<T>>;

/// Flips the axis of 2d Vectors
///
/// # Panics
///
/// Will panic if inner vectors have different lengths
pub fn transpose<T>(v: V2<T>) -> V2<T> {
    if v.is_empty() {
        return v;
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(IntoIterator::into_iter).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

/// Get's neighbor coordinates for 2d grids
pub fn get_neighbors<T>(x: T, y: T, w: T, h: T, diagonals: bool) -> Vec<(T, T)>
where
    T: Integer + Clone + Copy,
{
    let mut xs = vec![];
    let mut ys = vec![];
    if x > Zero::zero() {
        xs.push(x - One::one());
    }
    if x < w - One::one() {
        xs.push(x + One::one());
    }
    if y > Zero::zero() {
        ys.push(y - One::one());
    }
    if y < h - One::one() {
        ys.push(y + One::one());
    }

    if diagonals {
        xs.push(x);
        ys.push(y);
        iproduct!(xs, ys).collect()
    } else {
        let mut result: Vec<(T, T)> = xs.iter().map(|xi| (*xi, y)).collect();
        result.extend(ys.iter().map(|yi| (x, *yi)));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        assert_eq!(get_neighbors(0, 0, 10, 10, false), [(1, 0), (0, 1)]);
        assert_eq!(get_neighbors(10, 10, 10, 10, false), [(9, 10), (10, 9)]);
        assert_eq!(
            get_neighbors(5, 5, 10, 10, false),
            [(4, 5), (6, 5), (5, 4), (5, 6)]
        );
    }

    #[test]
    fn test_neighbors_diagonals() {
        assert_eq!(
            get_neighbors(0, 0, 10, 10, true),
            [(1, 1), (1, 0), (0, 1), (0, 0)]
        );
        assert_eq!(
            get_neighbors(10, 10, 10, 10, true),
            [(9, 9), (9, 10), (10, 9), (10, 10)]
        );
        assert_eq!(
            get_neighbors(5, 5, 10, 10, true),
            [
                (4, 4),
                (4, 6),
                (4, 5),
                (6, 4),
                (6, 6),
                (6, 5),
                (5, 4),
                (5, 6),
                (5, 5)
            ]
        );
    }
}
