use std::collections::HashMap;

pub fn is_nice_str(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double_letter = false;
    let mut previous = ' '; // dummy value, will never match
    for c in s.chars() {
        match (previous, c) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            (_, 'a' | 'e' | 'i' | 'o' | 'u') => vowel_count += 1,
            _ => (),
        }
        if previous == c {
            has_double_letter = true;
        }
        previous = c;
    }
    vowel_count >= 3 && has_double_letter
}

pub fn is_even_nicer_str(s: &str) -> (bool, bool, HashMap<(char, char), Vec<i32>>) {
    let mut chars = s.chars();
    let mut window = [' '; 3];
    let mut pairs: HashMap<(char, char), Vec<i32>> = HashMap::new();
    let mut has_wings = false;
    let mut has_pairs = false;
    let mut index: i32 = -3;
    loop {
        if window[0] == window[2] && window[0] != ' ' {
            has_wings = true;
        }
        if !has_pairs && index >= 0 {
            pairs
                .entry((window[0], window[1]))
                .and_modify(|v: &mut Vec<i32>| {
                    if v.last().expect("Vec should not be empty") + 1 != index {
                        has_pairs = true;
                    }
                    v.push(index);
                })
                .or_insert_with(|| vec![index]);
        }
        window = [window[1], window[2], chars.next().unwrap_or(' ')];
        index += 1;
        if window[2] == ' ' || (has_pairs && has_wings) {
            break;
        }
    }
    (has_wings, has_pairs, pairs)
}

pub fn has_pairs(s: &str) -> bool {
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut previous = ' ';
    s.chars().enumerate().for_each(|(i, c)| {
        let next = s.chars().nth(i + 1).unwrap_or(' ');
        let triple = next == c && c == previous;
        if previous != ' ' && !triple {
            let pair = format!("{previous}{c}");
            *pairs.entry(pair).or_insert(0) += 1;
        }
        previous = c;
    });
    pairs.values().any(|v| *v > 1)
}

pub fn winged_pairs(s: &str) -> bool {
    (1..s.len()).any(|i| s.chars().nth(i - 1).unwrap_or('*') == s.chars().nth(i + 1).unwrap_or('?'))
}

pub fn has_triple(s: &str) -> bool {
    (1..s.len()).any(|i| {
        let previous = s.chars().nth(i - 1).unwrap_or('*');
        let current = s.chars().nth(i).unwrap_or('?');
        let next = s.chars().nth(i + 1).unwrap_or('$');
        previous == next && previous == current
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("day05.txt");

    #[test]
    fn nice_string() {
        assert!(!is_nice_str("aeiouaeiouaeiou")); // cspell:disable-line
        assert!(!is_nice_str("aei"));
        assert!(!is_nice_str("xazegov")); // cspell:disable-line
        assert!(!is_nice_str("xx"));
        assert!(!is_nice_str("abcdde")); // cspell:disable-line
    }

    #[test]
    fn part_1() {
        let nice_strings = INPUT.lines().filter(|s| is_nice_str(s)).count();
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

    #[test]
    fn part_2_nicer() {
        let count = INPUT
            .lines()
            .filter(|s| {
                let result = is_even_nicer_str(s);
                if !result.1 {
                    println!("{}: {:?}", s, result.1);
                }
                result.0 && result.1
            })
            .count();

        assert_eq!(count, 69); // todo: 61 is the wrong answer, the right one is 69
    }
}
