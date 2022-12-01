fn input(example: bool) -> Vec<usize> {
    const PATH: &str = "inputs/2022/day01.txt";
    let data: Vec<String> = if example {
        "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split('\n')
    .map(|s| s.trim().to_string())
    .collect();

    let mut elves = vec![];
    let mut sum: usize = 0;
    for cal in data {
        if cal.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += cal.parse::<usize>().unwrap();
        }
    }
    elves.push(sum);
    elves.sort_unstable();
    elves
}

fn part_1(elves: &[usize]) -> usize {
    elves.iter().max().unwrap().to_owned()
}

fn part_2(elves: &[usize]) -> usize {
    elves[(elves.len() - 3)..].iter().sum()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 24000);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 67633);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 45000);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 199_628);
}
