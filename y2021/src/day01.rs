pub fn count_increments(data: &[i32]) -> usize {
    (1..data.len())
        .filter(|i| data[*i] - data[i - 1] > 0)
        .count()
}

pub fn count_increments_windows(data: &[i32]) -> usize {
    let windows: Vec<i32> = data.windows(3).map(|slice| slice.iter().sum()).collect();
    (1..windows.len())
        .filter(|i| windows[*i] - windows[i - 1] > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::{file::get_data, split_lines};

    fn setup_data(data: &[String]) -> Vec<i32> {
        data.iter().map(|x| x.parse().unwrap_or_default()).collect()
    }

    #[test]
    fn example() {
        let data = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263";
        let data = setup_data(&split_lines(data));
        let result = count_increments(&data);
        assert_eq!(result, 7);
    }

    #[test]
    fn task_1() {
        let data = setup_data(&get_data("inputs/day01.txt").unwrap());
        let result = count_increments(&data);
        assert_eq!(result, 1316);
    }

    #[test]
    fn task_2() {
        let data = setup_data(&get_data("inputs/day01.txt").unwrap());
        let result = count_increments_windows(&data);
        assert_eq!(result, 1344);
    }
}
