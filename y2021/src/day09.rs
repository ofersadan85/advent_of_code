use advent_of_code_common::v2::{get_neighbors, V2};
use std::collections::HashSet;

const PATH: &str = "../inputs/2021/day09.txt";
const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

fn get_low_points(data: &V2<u32>) -> usize {
    let mut sum: usize = 0;
    let mut count: usize = 0;
    let (h, w) = (data.len(), data[0].len());
    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let neighbors: Vec<u32> = get_neighbors(x, y, w, h, false)
                .iter()
                .map(|(xi, yi)| data[*yi][*xi])
                .collect();
            if neighbors.iter().all(|v| v > value) {
                sum += *value as usize;
                count += 1;
            }
        }
    }
    sum + count
}

fn get_basin(data: &V2<u32>, x: usize, y: usize, set: &mut HashSet<(usize, usize)>) {
    if data[y][x] == 9 {
        return;
    }
    set.insert((x, y));
    let (h, w) = (data.len(), data[0].len());
    for (xi, yi) in get_neighbors(x, y, w, h, false) {
        if data[yi][xi] != 9 && !set.contains(&(xi, yi)) {
            set.insert((xi, yi));
            get_basin(data, xi, yi, set);
        }
    }
}

fn count_basins(data: &V2<u32>) -> usize {
    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut result: Vec<usize> = vec![];

    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value != &9 && !checked.contains(&(x, y)) {
                let mut set: HashSet<(usize, usize)> = HashSet::new();
                get_basin(data, x, y, &mut set);
                for value in &set {
                    checked.insert(*value);
                }
                result.push(set.len());
            }
        }
    }
    result.sort_unstable();
    result[(result.len() - 3)..].iter().product()
}

fn setup_data(data: &[String]) -> V2<u32> {
    data.iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::file::lines_as_digits;

    #[test]
    fn example_1() {
        let data = lines_as_digits(EXAMPLE).unwrap();
        assert_eq!(get_low_points(&data), 15);
    }

    #[test]
    fn example_2() {
        let data = lines_as_digits(EXAMPLE).unwrap();
        assert_eq!(count_basins(&data), 1134);
    }

    #[test]
    fn task_1() {
        let input = std::fs::read_to_string(PATH).unwrap();
        let data = lines_as_digits(&input).unwrap();
        assert_eq!(get_low_points(&data), 594);
    }

    #[test]
    fn task_2() {
        let input = std::fs::read_to_string(PATH).unwrap();
        let data = lines_as_digits(&input).unwrap();
        assert_eq!(count_basins(&data), 858_494);
    }
}
