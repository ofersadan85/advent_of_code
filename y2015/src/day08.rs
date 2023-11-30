#![allow(dead_code)]

fn str_unescape(s: &str) -> usize {
    let mut chars = s.char_indices();
    let mut result = vec![];
    while let Some((i, c)) = chars.next() {
        match (i, c) {
            (0, '"') => {}                     // First char
            (i, '"') if i == s.len() - 1 => {} // Last char
            (_, '\\') => {
                let (_, c) = chars.next().expect("Escape char");
                match c {
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    'x' => {
                        let (_, first) = chars.next().expect("Hex char");
                        let (_, second) = chars.next().expect("Hex char");
                        // Convert hex to char
                        let hex = format!("{}{}", first, second);
                        if let Ok(c) = u8::from_str_radix(hex.as_str(), 16) {
                            result.push(c as char);
                        } else {
                            result.push('\\');
                            result.push('x');
                            result.push(first);
                            result.push(second);
                        }
                    }
                    _ => panic!("Unknown escape char"),
                }
            }
            _ => result.push(c),
        }
    }
    result.len()
}

fn str_escape(s: &str) -> usize {
    s.chars()
        .map(|c| match c {
            '\\' => 2,
            '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 2 // Plus 2 for the surrounding quotes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape() {
        let r = r#""v\x1b\"lgs\"t\x7ar""#;
        let s = "v\x1b\"lgs\"t\x7ar";
        // assert_eq!(s.len(), 23);
        assert_eq!(str_unescape(s), str_unescape(r));
        assert_eq!(str_unescape(r), s.len());
    }

    #[test]
    fn test_part1() {
        let input = include_str!("day08.txt");
        let total: usize = input.lines().map(|l| l.len() - str_unescape(l)).sum();
        assert_eq!(total, 1342);
    }

    #[test]
    fn test_escape() {
        let s = r#""""#;
        assert_eq!(s.len(), 2);
        assert_eq!(str_escape(s), 6);
        let s = r#""abc""#;
        assert_eq!(s.len(), 5);
        assert_eq!(str_escape(s), 9);
        let s = r#""aaa\"aaa""#;
        assert_eq!(s.len(), 10);
        assert_eq!(str_escape(s), 16);
        let s = r#""\x27""#;
        assert_eq!(s.len(), 6);
        assert_eq!(str_escape(s), 11);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("day08.txt");
        let total: usize = input.lines().map(|l| str_escape(l) - l.len()).sum();
        assert_eq!(total, 2074);
    }
}
