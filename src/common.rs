use itertools::iproduct;

pub fn split_lines(s: &str) -> Vec<String> {
    s.trim().split('\n').map(|x| x.trim().to_string()).collect()
}

pub fn get_data(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(split_lines(&std::fs::read_to_string(path)?))
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return v;
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn simple_series_sum(n: usize) -> usize {
    (n.pow(2) + n) / 2
}

pub fn get_neighbors(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    diagonals: bool,
) -> Vec<(usize, usize)> {
    let mut xs = vec![];
    let mut ys = vec![];
    if x > 0 {
        xs.push(x - 1);
    }
    if x < w - 1 {
        xs.push(x + 1);
    }
    if y > 0 {
        ys.push(y - 1);
    }
    if y < h - 1 {
        ys.push(y + 1);
    }

    if diagonals {
        xs.push(x);
        ys.push(y);
        iproduct!(xs, ys).collect()
    } else {
        let mut result: Vec<(usize, usize)> = xs.iter().map(|xi| (*xi, y)).collect();
        result.extend(ys.iter().map(|yi| (x, *yi)));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        assert_eq!(get_neighbors(0, 0, 10, 10, false), vec![(1, 0), (0, 1)]);
        assert_eq!(get_neighbors(10, 10, 10, 10, false), vec![(9, 10), (10, 9)]);
        assert_eq!(
            get_neighbors(5, 5, 10, 10, false),
            vec![(4, 5), (6, 5), (5, 4), (5, 6)]
        );
    }

    #[test]
    fn test_neighbors_diagonals() {
        assert_eq!(
            get_neighbors(0, 0, 10, 10, true),
            vec![(1, 1), (1, 0), (0, 1), (0, 0)]
        );
        assert_eq!(
            get_neighbors(10, 10, 10, 10, true),
            vec![(9, 9), (9, 10), (10, 9), (10, 10)]
        );
        assert_eq!(
            get_neighbors(5, 5, 10, 10, true),
            vec![
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
