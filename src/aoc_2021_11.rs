use crate::common::get_neighbors;
use itertools::iproduct;

fn flash(data: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut counter = 0;
    if data[y][x] >= 10 {
        let (h, w) = (data.len(), data[0].len());
        data[y][x] = 0;
        counter += 1;
        for (xi, yi) in get_neighbors(x, y, w, h, true) {
            if data[yi][xi] > 0 {
                data[yi][xi] += 1;
                counter += flash(data, xi, yi);
            }
        }
    }
    counter
}

fn sync_flash(data: &mut Vec<Vec<u8>>) -> usize {
    let mut counter = 0;
    let (h, w) = (data.len(), data[0].len());

    loop {
        for (y, x) in iproduct!(0..h, 0..w) {
            data[y][x] += 1;
        }
        for (y, x) in iproduct!(0..h, 0..w) {
            if data[y][x] == 10 {
                let result = flash(data, x, y);
                if result == h*w {
                    return counter + 1;
                }
            }
        }
        counter += 1;
    }
}

fn count_flashes(data: &mut Vec<Vec<u8>>, steps: usize) -> usize {
    let mut counter = 0;
    let (h, w) = (data.len(), data[0].len());

    for _ in 0..steps {
        for (y, x) in iproduct!(0..h, 0..w) {
            data[y][x] += 1;
        }
        for (y, x) in iproduct!(0..h, 0..w) {
            if data[y][x] == 10 {
                counter += flash(data, x, y);
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;
    const PATH: &str = "inputs/aoc_2021_11.txt";
    const EXAMPLE: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    fn setup_data(data: Vec<String>) -> Vec<Vec<u8>> {
        data.iter()
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect()
    }

    #[test]
    fn example_1() {
        let mut data = setup_data(split_lines(EXAMPLE));
        let result = count_flashes(&mut data, 100);
        assert_eq!(result, 1656);
    }

    #[test]
    fn example_2() {
        let mut data = setup_data(split_lines(EXAMPLE));
        let result: usize = sync_flash(&mut data);
        assert_eq!(result, 195);
    }

    #[test]
    fn task_1() {
        let mut data = setup_data(get_data(PATH).unwrap());
        let result: usize = count_flashes(&mut data, 100);
        assert_eq!(result, 1741);
    }

    #[test]
    fn task_2() {
        let mut data = setup_data(get_data(PATH).unwrap());
        let result: usize = sync_flash(&mut data);
        assert_eq!(result, 440);
    }
}
