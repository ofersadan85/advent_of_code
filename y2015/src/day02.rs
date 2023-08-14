pub fn wrapper(x: usize, y: usize, z: usize) -> usize {
    let w = 2 * x * y;
    let h = 2 * y * z;
    let l = 2 * x * z;
    let min = w.min(h).min(l);
    w + h + l + min / 2
}

pub fn ribbon(x: usize, y: usize, z: usize) -> usize {
    let max = x.max(y).max(z);
    (x + y + z - max) * 2 + x * y * z
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("day02.txt");

    #[test]
    fn test_wrapper() {
        assert_eq!(wrapper(2, 3, 4), 58);
        assert_eq!(wrapper(1, 1, 10), 43);
    }

    #[test]
    fn part_1() {
        let result: usize = INPUT
            .lines()
            .map(|line| {
                let dims: Vec<usize> = line.splitn(3, 'x').flat_map(|s| s.parse()).collect();
                wrapper(dims[0], dims[1], dims[2])
            })
            .sum();
        assert_eq!(result, 1588178);
    }

    #[test]
    fn test_ribbon() {
        assert_eq!(ribbon(2, 3, 4), 34);
        assert_eq!(ribbon(1, 1, 10), 14);
    }

    #[test]
    fn part_2() {
        let result: usize = INPUT
            .lines()
            .map(|line| {
                let dims: Vec<usize> = line.splitn(3, 'x').flat_map(|s| s.parse()).collect();
                ribbon(dims[0], dims[1], dims[2])
            })
            .sum();
        assert_eq!(result, 3783758);
    }
}
