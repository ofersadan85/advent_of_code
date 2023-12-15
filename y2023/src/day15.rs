pub fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |h, c| h.wrapping_add(c as u8).wrapping_mul(17))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_one() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn hash_all() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let hashes: usize = input.split(',').map(|s| hash(s) as usize).sum();
        assert_eq!(hashes, 1320);
    }

    #[test]
    fn part1() {
        let input = include_str!("day15.txt");
        let hashes: usize = input.split(',').map(|s| hash(s) as usize).sum();
        assert_eq!(hashes, 505427);
    }
}
