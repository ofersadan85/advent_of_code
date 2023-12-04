use itertools::Itertools;

pub const EXAMPLE1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub copies: usize,
    pub win_numbers: Vec<usize>,
    pub have_numbers: Vec<usize>,
}

impl TryFrom<&str> for Card {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (id, rest) = value.split_once(':').ok_or("No ':'")?;
        let (left, right) = rest.split_once('|').ok_or("No '|'")?;
        let win_numbers = left
            .split_whitespace()
            .filter_map(|part| part.trim_matches(':').parse::<usize>().ok())
            .collect_vec();
        let id = id
            .split_whitespace()
            .last()
            .ok_or("No id")?
            .parse::<usize>()
            .map_err(|_| "Invalid id")?;
        let have_numbers = right
            .split_whitespace()
            .filter_map(|part| part.trim_matches(':').parse::<usize>().ok())
            .collect_vec();
        Ok(Self {
            id,
            copies: 1,
            win_numbers,
            have_numbers,
        })
    }
}

impl Card {
    pub fn count(&self) -> usize {
        self.have_numbers
            .iter()
            .filter(|n| self.win_numbers.contains(n))
            .count()
    }

    pub fn score(&self) -> usize {
        let count = self.count();
        if count == 0 {
            0
        } else {
            1 << (count - 1)
            // 2usize.pow((count - 1) as u32) OR 2^(count-1)
            // Subtracting is ok because count is at least 1
        }
    }
}

pub fn card_explosion(cards: &mut Vec<Card>) {
    let mut index = 0;
    let last_index = cards.len() - 1;
    loop {
        let card = cards.get(index).cloned();
        if let Some(card) = card {
            let lookup_start = index + 1;
            let lookup_end = (index + card.count()).min(last_index);
            let lookup_range = lookup_start..=lookup_end;
            for other_index in lookup_range {
                cards
                    .get_mut(other_index)
                    .expect("We know the index exists")
                    .copies += card.copies;
            }
        }
        index += 1;
        if index >= last_index {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result: usize = EXAMPLE1
            .lines()
            .filter_map(|line| Card::try_from(line).ok())
            .map(|card| card.score())
            .sum();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1() {
        let result: usize = include_str!("day04.txt")
            .lines()
            .filter_map(|line| Card::try_from(line).ok())
            .map(|card| card.score())
            .sum();
        assert_eq!(result, 23441);
    }

    #[test]
    fn test_example2() {
        let mut cards = EXAMPLE1
            .lines()
            .filter_map(|line| Card::try_from(line).ok())
            .collect_vec();
        card_explosion(&mut cards);
        let result: usize = cards.iter().map(|card| card.copies).sum();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_2() {
        let mut cards = include_str!("day04.txt")
            .lines()
            .filter_map(|line| Card::try_from(line).ok())
            .collect_vec();
        card_explosion(&mut cards);
        let result: usize = cards.iter().map(|card| card.copies).sum();
        assert_eq!(result, 5923918);
    }
}
