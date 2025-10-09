use itertools::Itertools;

pub fn sum_digits(s: &str) -> u32 {
    let mut digits = s.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap_or(0);
    let last = digits.next_back().unwrap_or(first);
    first * 10 + last
}

const DIGIT_WORDS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

pub fn word_to_num(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => s.parse::<u32>().ok(),
    }
}

pub fn sum_digit_words(s: &str) -> u32 {
    let found = DIGIT_WORDS
        .iter()
        .map(|&word| (word, s.find(word), s.rfind(word)))
        .collect_vec();
    let first = found
        .iter()
        .filter_map(|(word, first, _)| first.map(|f| (word, f)))
        .min_by_key(|(_, f)| *f)
        .expect("At least one word");
    let last = found
        .iter()
        .filter_map(|(word, _, last)| last.map(|l| (word, l)))
        .max_by_key(|(_, l)| *l)
        .expect("At least one word");
    let first_num = word_to_num(first.0).expect("First word is a number");
    let last_num = word_to_num(last.0).expect("Last word is a number");
    first_num * 10 + last_num
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../inputs/2023/day01.txt");

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits("1abc2"), 12);
        assert_eq!(sum_digits("pqr3stu8vwx"), 38);
        assert_eq!(sum_digits("a1b2c3d4e5f"), 15);
        assert_eq!(sum_digits("treb7uchet"), 77);
    }

    #[test]
    fn test_part1() {
        let result = INPUT.lines().map(sum_digits).sum::<u32>();
        assert_eq!(result, 54940);
    }

    #[test]
    fn test_sum_digit_words() {
        assert_eq!(sum_digit_words("two1nine"), 29);
        assert_eq!(sum_digit_words("eightwothree"), 83);
        assert_eq!(sum_digit_words("abcone2threexyz"), 13);
        assert_eq!(sum_digit_words("xtwone3four"), 24);
        assert_eq!(sum_digit_words("4nineeightseven2"), 42);
        assert_eq!(sum_digit_words("zoneight234"), 14);
        assert_eq!(sum_digit_words("7pqrstsixteen"), 76);
        let all_lines = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = all_lines.lines().map(sum_digit_words).sum::<u32>();
        assert_eq!(result, 281);
    }

    #[test]
    fn test_part2() {
        let result = INPUT.lines().map(sum_digit_words).sum::<u32>();
        assert_eq!(result, 54208);
    }
}
