fn grow(v: &mut [usize], n: usize) {
    for _ in 0..n {
        v.rotate_left(1);
        v[6] += v[8];
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use advent_of_code_common::{file::get_data, split_lines};
    const PATH: &str = "inputs/day06.txt";
    const EXAMPLE: &str = "3,4,3,1,2";

    fn setup_data(data: &[String]) -> [usize; 9] {
        let mut result = [0; 9];
        let counter = data[0]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .counts();
        for (k, v) in counter {
            result[k] = v;
        }
        result
    }

    #[test]
    fn example_1() {
        let mut data = setup_data(&split_lines(EXAMPLE));
        grow(&mut data, 18);
        let result: usize = data.iter().sum();
        assert_eq!(result, 26);
    }

    #[test]
    fn example_2() {
        let mut data = setup_data(&split_lines(EXAMPLE));
        grow(&mut data, 256);
        let result: usize = data.iter().sum();
        assert_eq!(result, 26_984_457_539);
    }

    #[test]
    fn task_1() {
        let mut data = setup_data(&get_data(PATH).unwrap());
        grow(&mut data, 80);
        let result: usize = data.iter().sum();
        assert_eq!(result, 361_169);
    }

    #[test]
    fn task_2() {
        let mut data = setup_data(&get_data(PATH).unwrap());
        grow(&mut data, 256);
        let result: usize = data.iter().sum();
        assert_eq!(result, 1_634_946_868_992);
    }
}
