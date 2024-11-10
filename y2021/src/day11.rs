use advent_of_code_common::v2::{get_neighbors, V2};
use itertools::iproduct;

const PATH: &str = "../inputs/2021/day11.txt";
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

fn flash(data: &mut V2<u32>, x: usize, y: usize) -> usize {
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

fn sync_flash(data: &mut V2<u32>) -> usize {
    let mut counter = 0;
    let (h, w) = (data.len(), data[0].len());

    loop {
        for (y, x) in iproduct!(0..h, 0..w) {
            data[y][x] += 1;
        }
        for (y, x) in iproduct!(0..h, 0..w) {
            if data[y][x] == 10 {
                let result = flash(data, x, y);
                if result == h * w {
                    return counter + 1;
                }
            }
        }
        counter += 1;
    }
}

fn count_flashes(data: &mut V2<u32>, steps: usize) -> usize {
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

fn setup_data(data: &[String]) -> V2<u32> {
    data.iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::file::{lines_as_digits, parse_file};

    #[test]
    fn example_1() {
        let mut data = lines_as_digits(EXAMPLE).unwrap();
        let result = count_flashes(&mut data, 100);
        assert_eq!(result, 1656);
    }

    #[test]
    fn example_2() {
        let mut data = lines_as_digits(EXAMPLE).unwrap();
        let result: usize = sync_flash(&mut data);
        assert_eq!(result, 195);
    }

    #[test]
    fn task_1() {
        let mut data = parse_file(PATH, lines_as_digits).unwrap();
        let result: usize = count_flashes(&mut data, 100);
        assert_eq!(result, 1741);
    }

    #[test]
    fn task_2() {
        let mut data = parse_file(PATH, lines_as_digits).unwrap();
        let result: usize = sync_flash(&mut data);
        assert_eq!(result, 440);
    }
}
