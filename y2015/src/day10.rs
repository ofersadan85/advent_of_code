pub fn look_and_say(numbers: &mut Vec<u8>) {
    let mut result = Vec::with_capacity(numbers.len() * 2);
    let mut i = 0;
    while i < numbers.len() {
        let a = numbers[i]; // Safe because we know i < numbers.len()
        let b = numbers.get(i + 1).copied().unwrap_or(0);
        let c = numbers.get(i + 2).copied().unwrap_or(0);
        match (a, b, c) {
            (a, b, c) if a == b && b == c => {
                result.push(3);
                result.push(a);
                i += 3;
            }
            (a, b, _) if a == b => {
                result.push(2);
                result.push(a);
                i += 2;
            }
            (a, _, _) => {
                result.push(1);
                result.push(a);
                i += 1;
            }
        }
    }
    *numbers = result;
}

pub fn look_and_say_many(s: &str, n: usize) -> String {
    #[allow(clippy::cast_possible_truncation)] // c is always a single digit
    let mut numbers = s
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
    for _ in 0..n {
        look_and_say(&mut numbers);
    }
    numbers
        .iter()
        .filter_map(|n| char::from_digit(u32::from(*n), 10))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(look_and_say_many("1", 1), "11");
        assert_eq!(look_and_say_many("11", 1), "21");
        assert_eq!(look_and_say_many("21", 1), "1211");
        assert_eq!(look_and_say_many("1211", 1), "111221");
        assert_eq!(look_and_say_many("111221", 1), "312211");
    }

    #[test]
    fn test_part1() {
        let result = look_and_say_many("3113322113", 40);
        assert_eq!(result.len(), 329356);
    }

    #[test]
    fn test_part2() {
        let result = look_and_say_many("3113322113", 50);
        assert_eq!(result.len(), 4666278);
    }
}
