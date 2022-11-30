use itertools::iproduct;
use std::collections::HashMap;

type V2dBool = Vec<Vec<bool>>;

fn count_true(data: &V2dBool) -> HashMap<usize, usize> {
    let mut counter = HashMap::new();
    for (row, i) in iproduct!(data, 0..data[0].len()) {
        if *row.get(i).unwrap_or(&false) {
            counter.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    counter
}

fn power_consumption(data: V2dBool) -> usize {
    let counter = count_true(&data);
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for i in 0..data[0].len() {
        if counter.get(&i).unwrap_or(&0) * 2 > data.len() {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn filter_v2d(data: &V2dBool, i: usize, common: bool) -> V2dBool {
    let data = data.clone();
    let counter = count_true(&data);
    let common_test = match common {
        true => counter.get(&i).unwrap_or(&0) * 2 >= data.len(),
        false => counter.get(&i).unwrap_or(&0) * 2 < data.len(),
    };
    data.iter()
        .cloned()
        .filter(|row| row[i] == common_test)
        .collect()
}

fn num_bin_vec(data: &V2dBool) -> Vec<usize> {
    data.iter()
        .map(|row| {
            usize::from_str_radix(
                &row.iter()
                    .map(|b| if *b { '1' } else { '0' })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .collect()
}

fn life_support(data: V2dBool) -> usize {
    let mut data_o2 = data.clone();
    let mut data_co2 = data.clone();
    for i in 0..data[0].len() {
        if data_o2.len() > 1 {
            data_o2 = filter_v2d(&data_o2, i, true)
        }
        if data_co2.len() > 1 {
            data_co2 = filter_v2d(&data_co2, i, false)
        }
    }
    num_bin_vec(&data_o2)[0] * num_bin_vec(&data_co2)[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;
    const PATH: &str = "inputs/2021/day03.txt";
    const EXAMPLE: &str = "
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    fn setup_data(data: Vec<String>) -> V2dBool {
        data.iter()
            .map(|line| line.as_bytes().iter().map(|b| *b as char == '1').collect())
            .collect()
    }

    #[test]
    fn example_1() {
        let data = setup_data(split_lines(EXAMPLE));
        let result = power_consumption(data);
        assert_eq!(result, 198);
    }

    #[test]
    fn example_2() {
        let data = setup_data(split_lines(EXAMPLE));
        let result = life_support(data);
        assert_eq!(result, 230);
    }

    #[test]
    fn task_1() {
        let data = setup_data(get_data(PATH).unwrap());
        let result = power_consumption(data);
        assert_eq!(result, 2035764);
    }

    #[test]
    fn task_2() {
        let data = setup_data(get_data(PATH).unwrap());
        let result = life_support(data);
        assert_eq!(result, 2817661);
    }
}
