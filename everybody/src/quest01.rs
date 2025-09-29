use itertools::Itertools;

#[derive(Debug)]
enum Creature {
    A,
    B,
    C,
    D,
}

impl TryFrom<char> for Creature {
    type Error = ();
    fn try_from(v: char) -> Result<Self, ()> {
        match v {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            _ => Err(()),
        }
    }
}

impl Creature {
    fn potions(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 3,
            Self::D => 5,
        }
    }
}

fn creatures_potions(input: &str, chunk_size: usize) -> usize {
    input
        .chars()
        .map(|c| Creature::try_from(c))
        .chunks(chunk_size)
        .into_iter()
        .map(|c| {
            let mut ok_count: usize = 0;
            let mut potions = 0;
            for v in c {
                match v {
                    Ok(v) => {
                        potions += v.potions();
                        ok_count += 1;
                    }
                    Err(_) => {}
                }
            }
            potions + (ok_count.saturating_sub(1)) * ok_count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = "ABBAC";
        assert_eq!(creatures_potions(input, 1), 5);
        let input = read_to_string("../inputs/everybody/quest01part1.txt").expect("Input file");
        assert_eq!(creatures_potions(&input, 1), 1324);
    }

    #[test]
    fn part2() {
        let input = "AxBCDDCAxD";
        assert_eq!(creatures_potions(input, 2), 28);
        let input = read_to_string("../inputs/everybody/quest01part2.txt").expect("Input file");
        assert_eq!(creatures_potions(&input, 2), 5666);
    }

    #[test]
    fn part3() {
        let input = "xBxAAABCDxCC";
        assert_eq!(creatures_potions(input, 3), 30);
        let input = read_to_string("../inputs/everybody/quest01part3.txt").expect("Input file");
        assert_eq!(creatures_potions(&input, 3), 27834);
    }
}
