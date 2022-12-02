use itertools::Itertools;

fn calc_fuel(data: &[u32]) -> u32 {
    let (min, max) = data.iter().minmax().into_option().unwrap();
    (*min..*max)
        .map(|i| data.iter().map(|x| x.abs_diff(i)).sum())
        .min()
        .unwrap()
}

fn calc_fuel_increasing(data: &[u32]) -> u32 {
    let (min, max) = data.iter().minmax().into_option().unwrap();
    (*min..*max)
        .map(|i| {
            data.iter()
                .map(|&x| {
                    let diff = x.abs_diff(i);
                    (diff * diff + diff) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::{get_data, split_lines};
    const PATH: &str = "inputs/day07.txt";
    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    fn setup_data(data: &[String]) -> Vec<u32> {
        data[0].split(',').map(|s| s.parse().unwrap()).collect()
    }

    #[test]
    fn example_1() {
        let data = setup_data(&split_lines(EXAMPLE));
        let result: u32 = calc_fuel(&data);
        assert_eq!(result, 37);
    }

    #[test]
    fn example_2() {
        let data = setup_data(&split_lines(EXAMPLE));
        let result: u32 = calc_fuel_increasing(&data);
        assert_eq!(result, 168);
    }

    #[test]
    fn task_1() {
        let data = setup_data(&get_data(PATH).unwrap());
        let result: u32 = calc_fuel(&data);
        assert_eq!(result, 356_958);
    }

    #[test]
    fn task_2() {
        let data = setup_data(&get_data(PATH).unwrap());
        let result: u32 = calc_fuel_increasing(&data);
        assert_eq!(result, 105_461_913);
    }
}
