use regex::Regex;

pub fn sum_digits(s: &str) -> u32 {
    let mut digits = s.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap_or(0);
    let last = digits.last().unwrap_or(first);
    first * 10 + last
}

pub fn sum_digit_words(s: &str, re: &Regex) -> u32 {
    let mut digits = re
        .captures_iter(s)
        .map(|digit| {
            match digit
                .get(0)
                .expect("regex should match at least one digit or word")
                .as_str()
            {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                s => s,
            }
        })
        .filter_map(|s| s.parse::<u32>().ok());
    // let digits_v = digits.collect::<Vec<_>>();
    // let digits_s = digits_v.iter().map(|d| d.to_string()).collect::<String>();
    // let mut digits = digits_v.into_iter();
    let first = digits.next().unwrap_or(0);
    let last = digits.last().unwrap_or(first);
    // if digits_s.len() < 2 {
    //    eprintln!("s: {s}      digits_s: {digits_s}     {first}{last}");
    // }
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("day01.txt");

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
        let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").expect("regex");
        assert_eq!(sum_digit_words("two1nine", &re), 29);
        assert_eq!(sum_digit_words("eightwothree", &re), 83);
        assert_eq!(sum_digit_words("abcone2threexyz", &re), 13);
        assert_eq!(sum_digit_words("xtwone3four", &re), 24);
        assert_eq!(sum_digit_words("4nineeightseven2", &re), 42);
        assert_eq!(sum_digit_words("zoneight234", &re), 14);
        assert_eq!(sum_digit_words("7pqrstsixteen", &re), 76);
    }

    #[test]
    fn test_part2() {
        let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").expect("regex");
        let result = INPUT.lines().map(|s| sum_digit_words(s, &re)).sum::<u32>();
        assert_eq!(result, 54194);
    }
}
