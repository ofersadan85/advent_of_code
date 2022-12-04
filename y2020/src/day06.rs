use std::collections::HashSet;

fn input(example: bool) -> Vec<String> {
    const PATH: &str = "inputs/day06.txt";
    if example {
        "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b"
        .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split('\n')
    .map(|s| s.trim().to_string())
    .collect()
}

fn part_1(data: &[String]) -> usize {
    let mut sum = 0;
    let mut local_set = HashSet::new();
    for row in data {
        if row.is_empty() {
            sum += local_set.len();
            local_set = HashSet::new();
        } else {
            for c in row.chars() {
                local_set.insert(c);
            }
        }
    }
    sum += local_set.len();
    sum
}

fn part_2(data: &[String]) -> usize {
    let mut sum = 0;
    let mut local_set = HashSet::new();
    let mut is_first = true;
    for row in data {
        if row.is_empty() {
            sum += local_set.len();
            local_set = HashSet::new();
            is_first = true;
        } else {
            if is_first {
                local_set = row.chars().collect();
                is_first = false;
            }
            local_set = local_set.intersection(&row.chars().collect()).copied().collect();
        }
    }
    sum += local_set.len();
    sum
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 11);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 6542);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 6);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 3299);
}
