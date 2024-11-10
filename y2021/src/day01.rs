const PATH: &str = "../inputs/2021/day01.txt";
const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

fn count_increments(data: &[i32]) -> usize {
    (1..data.len())
        .filter(|i| data[*i] - data[i - 1] > 0)
        .count()
}

fn count_increments_windows(data: &[i32]) -> usize {
    let windows: Vec<i32> = data.windows(3).map(|slice| slice.iter().sum()).collect();
    (1..windows.len())
        .filter(|i| windows[*i] - windows[i - 1] > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::file::{lines_as_numbers, parse_file};

    #[test]
    fn example() {
        let data = lines_as_numbers(EXAMPLE).unwrap();
        assert_eq!(count_increments(&data), 7);
    }

    #[test]
    fn task_1() {
        let data = parse_file(PATH, lines_as_numbers).unwrap();
        assert_eq!(count_increments(&data), 1316);
    }

    #[test]
    fn task_2() {
        let data = parse_file(PATH, lines_as_numbers).unwrap();
        assert_eq!(count_increments_windows(&data), 1344);
    }
}
