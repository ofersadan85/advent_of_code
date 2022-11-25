use itertools::Itertools;

use crate::common::simple_series_sum;

fn calc_fuel(data: Vec<usize>) -> usize {
    let (min, max) = data.iter().minmax().into_option().unwrap();
    (*min..*max)
        .map(|i| data.iter().map(|x| x.abs_diff(i)).sum())
        .min()
        .unwrap()
}

fn calc_fuel_increasing(data: Vec<usize>) -> usize {
    let (min, max) = data.iter().minmax().into_option().unwrap();
    (*min..*max)
        .map(|i| data.iter().map(|x| simple_series_sum(x.abs_diff(i))).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;
    const PATH: &str = "inputs/aoc_2021_7.txt";
    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    fn setup_data(data: Vec<String>) -> Vec<usize> {
        data[0].split(',').map(|s| s.parse().unwrap()).collect()
    }

    #[test]
    fn example_1() {
        let data = setup_data(split_lines(EXAMPLE));
        let result: usize = calc_fuel(data);
        assert_eq!(result, 37);
    }

    #[test]
    fn example_2() {
        let data = setup_data(split_lines(EXAMPLE));
        let result: usize = calc_fuel_increasing(data);
        assert_eq!(result, 168);
    }

    #[test]
    fn task_1() {
        let data = setup_data(get_data(PATH).unwrap());
        let result: usize = calc_fuel(data);
        assert_eq!(result, 356958);
    }

    #[test]
    fn task_2() {
        let data = setup_data(get_data(PATH).unwrap());
        let result: usize = calc_fuel_increasing(data);
        assert_eq!(result, 356958);
    }
}
