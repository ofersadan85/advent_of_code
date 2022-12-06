use advent_of_code_common::{
    file::{lines_as_digits_radix, parse_file},
    v2::V2,
};
use itertools::iproduct;
use std::collections::HashMap;

const PATH: &str = "inputs/day03.txt";
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

fn count_true(data: &V2<bool>) -> HashMap<usize, usize> {
    let mut counter = HashMap::new();
    for (row, i) in iproduct!(data, 0..data[0].len()) {
        if *row.get(i).unwrap_or(&false) {
            counter.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    counter
}

fn power_consumption(data: &V2<bool>) -> usize {
    let counter = count_true(data);
    let mut gamma = String::new();
    let mut epsilon = String::new();
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

fn filter_v2d(data: &V2<bool>, i: usize, common: bool) -> V2<bool> {
    let data = data.clone();
    let counter = count_true(&data);
    let common_test = if common {
        counter.get(&i).unwrap_or(&0) * 2 >= data.len()
    } else {
        counter.get(&i).unwrap_or(&0) * 2 < data.len()
    };
    data.iter()
        .cloned()
        .filter(|row| row[i] == common_test)
        .collect()
}

fn num_bin_vec(data: &V2<bool>) -> Vec<usize> {
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

fn life_support(data: &V2<bool>) -> usize {
    let mut data_oxygen = data.clone();
    let mut data_co2 = data.clone();
    for i in 0..data[0].len() {
        if data_oxygen.len() > 1 {
            data_oxygen = filter_v2d(&data_oxygen, i, true);
        }
        if data_co2.len() > 1 {
            data_co2 = filter_v2d(&data_co2, i, false);
        }
    }
    num_bin_vec(&data_oxygen)[0] * num_bin_vec(&data_co2)[0]
}

fn input(example: bool) -> V2<bool> {
    let data: V2<u32> = if example {
        lines_as_digits_radix(EXAMPLE, 2)
    } else {
        parse_file(PATH, |lines| lines_as_digits_radix(lines, 2))
    }
    .unwrap();
    data.iter()
        .map(|line| line.iter().map(|&b| b == 1).collect())
        .collect()
}

#[test]
fn example_1() {
    assert_eq!(power_consumption(&input(true)), 198);
}

#[test]
fn example_2() {
    assert_eq!(life_support(&input(true)), 230);
}

#[test]
fn task_1() {
    assert_eq!(power_consumption(&input(false)), 2_035_764);
}

#[test]
fn task_2() {
    assert_eq!(life_support(&input(false)), 2_817_661);
}
