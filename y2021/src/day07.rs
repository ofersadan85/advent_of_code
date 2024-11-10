use itertools::Itertools;

const PATH: &str = "../inputs/2021/day07.txt";
const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

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

fn input(example: bool) -> Vec<u32> {
    if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect()
}

#[test]
fn example_1() {
    let data = input(true);
    assert_eq!(calc_fuel(&data), 37);
}

#[test]
fn example_2() {
    let data = input(true);
    assert_eq!(calc_fuel_increasing(&data), 168);
}

#[test]
fn task_1() {
    let data = input(false);
    assert_eq!(calc_fuel(&data), 356_958);
}

#[test]
fn task_2() {
    let data = input(false);
    assert_eq!(calc_fuel_increasing(&data), 105_461_913);
}
