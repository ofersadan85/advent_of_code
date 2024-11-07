pub fn count_parentheses(s: &str) -> (i32, usize) {
    let mut first_negative = 0;
    let mut sum = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => (),
        }
        if sum == -1 && first_negative == 0 {
            first_negative = i;
        }
    }
    (sum, first_negative + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../inputs/2015/day01.txt");

    #[test]
    fn examples_1() {
        [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ]
        .iter()
        .for_each(|(s, expected)| {
            assert_eq!(count_parentheses(s).0, *expected);
        });
    }

    #[test]
    fn examples_2() {
        [
            (")", 1),
            ("()())", 5),
        ]
        .iter()
        .for_each(|(s, expected)| {
            assert_eq!(count_parentheses(s).1, *expected);
        });
    }

    #[test]
    fn part_1() {
        assert_eq!(count_parentheses(INPUT).0, 74);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_parentheses(INPUT).1, 1795);
    }
}
