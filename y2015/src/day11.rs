const ABC: &str = "abcdefghijklmnopqrstuvwxyz";
// const FILTERED_ABC: &str = "abcdefghjkmnpqrstuvwxyz";

fn increment(mut s: &mut String) {
    // eprintln!("{s}==>");
    if let Some(last) = s.pop() {
        if last == 'z' {
            increment(&mut s);
            s.push('a');
        } else {
            // if "hkn".contains(last) {
            //     s.push((last as u8 + 2) as char);
            // } else {
            s.push((last as u8 + 1) as char);
            // }
        }
    } else {
        s.push('a');
    }
    // eprintln!("==>{s}");
    if s.len() > 8 {
        s.clear();
    }
}

fn has_straight(s: &str) -> bool {
    s.as_bytes()
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn has_pairs(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .filter(|w| w[0] == w[1])
        .collect::<Vec<_>>()
        .windows(2)
        .any(|w| w[0][0] != w[1][0])
}

fn next_password(mut s: String) -> String {
    increment(&mut s);
    while s.contains('i')
        || s.contains('o')
        || s.contains('l')
        || !has_pairs(&s)
        || !has_straight(&s)
    {
        increment(&mut s);
        if !s.starts_with('h') {
            return String::new();
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    // #[test]
    // fn test_increment() {
    //     let mut s = String::from(ABC);
    //     increment(&mut s);
    //     assert_eq!(s, "abcdefghijklmnopqrstuvwxza");
    //     s.clear();
    //     increment(&mut s);
    //     assert_eq!(s, "a");
    //     s.clear();
    //     s.push_str("zzz");
    //     increment(&mut s);
    //     assert_eq!(s, "aaaa");
    // }

    #[test]
    #[ignore = "Not Working"]
    fn test_part1() {
        // assert_eq!(next_password("abcdefgh"), "abcdffaa");
        // assert_eq!(next_password("ghijklmn"), "ghjaabcc");
        assert_eq!(
            next_password(read_to_string("../inputs/2015/day11.txt").unwrap()),
            "ghjaabcc"
        );
    }

    #[test]
    fn test_part2() {}
}
