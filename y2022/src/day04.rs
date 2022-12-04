fn input(example: bool) -> Vec<(usize, usize, usize, usize)> {
    const PATH: &str = "inputs/day04.txt";
    if example {
        "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split('\n')
    .map(|row| {
        let (range1, range2) = row.trim().split_once(',').unwrap();
        let (min1, max1) = range1.split_once('-').unwrap();
        let (min2, max2) = range2.split_once('-').unwrap();
        (
            min1.parse().unwrap(),
            max1.parse().unwrap(),
            min2.parse().unwrap(),
            max2.parse().unwrap(),
        )
    })
    .collect()
}

fn part_1(data: &[(usize, usize, usize, usize)]) -> usize {
    data.iter()
        .filter(|(min1, max1, min2, max2)| {
            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count()
}
fn part_2(data: &[(usize, usize, usize, usize)]) -> usize {
    data.iter()
        .filter(|(min1, max1, min2, max2)| {
            (min1 <= min2 && max1 >= max2)
                || (min2 <= min1 && max2 >= max1)
                || (max1 >= min2 && max1 <= max2)
                || (min1 >= min2 && min1 <= max2)
        })
        .count()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 2);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 513);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 4);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 878);
}
