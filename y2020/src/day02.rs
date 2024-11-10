#[derive(Debug)]
#[allow(clippy::struct_field_names)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn new(row: &str) -> Self {
        let mut split = row.split_ascii_whitespace();
        let mut range = split.next().unwrap().split('-');
        let min: usize = range.next().unwrap().parse().unwrap();
        let max: usize = range.next().unwrap().parse().unwrap();
        let letter = split.next().unwrap().chars().next().unwrap();
        Self {
            min,
            max,
            letter,
            password: split.next().unwrap().to_string(),
        }
    }

    fn is_valid_1(&self) -> bool {
        (self.min..=self.max).contains(&self.password.chars().filter(|&c| c == self.letter).count())
    }

    fn is_valid_2(&self) -> bool {
        let char_vec: Vec<char> = self.password.chars().collect();
        let (left, right) = (
            char_vec.get(self.min - 1).unwrap_or(&'@') == &self.letter,
            char_vec.get(self.max - 1).unwrap_or(&'@') == &self.letter,
        );
        left ^ right
    }
}

fn input(example: bool) -> Vec<Password> {
    const PATH: &str = "../inputs/2020/day02.txt";
    if example {
        "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .lines()
    .map(|row| Password::new(row.trim()))
    .collect()
}

fn part_1(data: &[Password]) -> usize {
    data.iter().filter(|p| p.is_valid_1()).count()
}

fn part_2(data: &[Password]) -> usize {
    data.iter().filter(|p| p.is_valid_2()).count()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 2);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 393);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 1);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 690);
}
