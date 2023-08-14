use std::collections::HashMap;

const FORBIDDEN_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn is_nice_string(s: &str) -> bool {
    if FORBIDDEN_STRINGS.iter().any(|fs| s.contains(fs)) {
        return false;
    }
    let mut vowel_count = 0;
    let mut has_double_letter = false;
    let mut previous = ' ';
    s.chars().for_each(|c| {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
        if previous == c {
            has_double_letter = true;
        }
        previous = c;
    });
    vowel_count >= 3 && has_double_letter
}

pub fn has_pairs(s: &str) -> bool {
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut previous = ' ';
    s.chars().enumerate().for_each(|(i, c)| {
        let next = s.chars().nth(i + 1).unwrap_or(' ');
        let triple = next == c && c == previous;
        if previous != ' ' && !triple {
            let pair = format!("{}{}", previous, c);
            *pairs.entry(pair).or_insert(0) += 1;
        }
        previous = c;
    });
    pairs.values().any(|v| *v > 1)
}

pub fn winged_pairs(s: &str) -> bool {
    (1..s.len())
        .find(|i| s.chars().nth(i - 1).unwrap_or('*') == s.chars().nth(i + 1).unwrap_or('?'))
        .is_some()
}

pub fn has_triple(s: &str) -> bool {
    (1..s.len())
        .find(|i| {
            let previous = s.chars().nth(i - 1).unwrap_or('*');
            let current = s.chars().nth(i + 0).unwrap_or('?');
            let next = s.chars().nth(i + 1).unwrap_or('$');
            previous == next && previous == current
        })
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("day05.txt");

    #[test]
    fn nice_string() {
        assert!(!is_nice_string("aeiouaeiouaeiou")); // cspell:disable-line
        assert!(!is_nice_string("aei"));
        assert!(!is_nice_string("xazegov")); // cspell:disable-line
        assert!(!is_nice_string("xx"));
        assert!(!is_nice_string("abcdde")); // cspell:disable-line
    }

    #[test]
    fn part_1() {
        let nice_strings = INPUT.lines().filter(|s| is_nice_string(s)).count();
        assert_eq!(nice_strings, 238);
    }

    #[test]
    fn test_pairs() {
        assert!(has_pairs("xyxy")); // cspell:disable-line
        assert!(has_pairs("aabcdefgaa")); // cspell:disable-line
        assert!(!has_pairs("aaa"));
        assert!(has_pairs("qjhvhtzxzqqjkmpb")); // cspell:disable-line
        assert!(has_pairs("xxyxx")); // cspell:disable-line
        assert!(has_pairs("uurcxstgmygtbstg")); // cspell:disable-line
        assert!(!has_pairs("ieodomkazucvgmuy")); // cspell:disable-line
        assert!(has_pairs("xyxyx")); // cspell:disable-line
    }

    #[test]
    fn test_winged_pairs() {
        assert!(winged_pairs("xyx")); // cspell:disable-line
        assert!(winged_pairs("abcdefeghi")); // cspell:disable-line
        assert!(winged_pairs("aaa"));
        assert!(winged_pairs("qjhvhtzxzqqjkmpb")); // cspell:disable-line
        assert!(winged_pairs("xxyxx")); // cspell:disable-line
        assert!(!winged_pairs("uurcxstgmygtbstg")); // cspell:disable-line
        assert!(winged_pairs("ieodomkazucvgmuy")); // cspell:disable-line
        assert!(winged_pairs("xyxyx")); // cspell:disable-line
    }

    #[test]
    fn part_2() {
        let nice_strings = INPUT
            .lines()
            .filter(|s| winged_pairs(s) && has_pairs(s))
            .count();
        assert_eq!(nice_strings, 69); // todo: 68 is the wrong answer, the right one is 69
    }
}
