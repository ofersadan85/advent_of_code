use crate::common::get_neighbors;
use std::collections::HashSet;

fn get_low_points(data: &Vec<Vec<u8>>) -> usize {
    let mut sum: usize = 0;
    let mut count: usize = 0;
    let (h, w) = (data.len(), data[0].len());
    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let neighbors: Vec<u8> = get_neighbors(x, y, w, h, false)
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

fn get_basin(data: &Vec<Vec<u8>>, x: usize, y: usize, set: &mut HashSet<(usize, usize)>) {
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

fn count_basins(data: &Vec<Vec<u8>>) -> usize {
    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut result: Vec<usize> = vec![];

    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value != &9 && !checked.contains(&(x, y)) {
                let mut set: HashSet<(usize, usize)> = HashSet::new();
                get_basin(data, x, y, &mut set);
                for value in set.iter() {
                    checked.insert(*value);
                }
                result.push(set.len());
            }
        }
    }
    result.sort();
    result[(result.len() - 3)..].iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;
    const PATH: &str = "inputs/aoc_2021_9.txt";
    const EXAMPLE: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    fn setup_data(data: Vec<String>) -> Vec<Vec<u8>> {
        data.iter()
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect()
    }

    #[test]
    fn example_1() {
        let data = setup_data(split_lines(EXAMPLE));
        let result: usize = get_low_points(&data);
        assert_eq!(result, 15);
    }

    #[test]
    fn example_2() {
        let data = setup_data(split_lines(EXAMPLE));
        let result: usize = count_basins(&data);
        assert_eq!(result, 1134);
    }

    #[test]
    fn task_1() {
        let data = setup_data(get_data(PATH).unwrap());
        let result: usize = get_low_points(&data);
        assert_eq!(result, 594);
    }

    #[test]
    fn task_2() {
        let data = setup_data(get_data(PATH).unwrap());
        let result: usize = count_basins(&data);
        assert_eq!(result, 858494);
    }
}
