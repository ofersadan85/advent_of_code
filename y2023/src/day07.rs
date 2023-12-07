use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Knight,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for CardValue {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(CardValue::Two),
            '3' => Ok(CardValue::Three),
            '4' => Ok(CardValue::Four),
            '5' => Ok(CardValue::Five),
            '6' => Ok(CardValue::Six),
            '7' => Ok(CardValue::Seven),
            '8' => Ok(CardValue::Eight),
            '9' => Ok(CardValue::Nine),
            'T' => Ok(CardValue::Ten),
            'J' => Ok(CardValue::Jack),
            'K' => Ok(CardValue::Knight),
            'Q' => Ok(CardValue::Queen),
            'A' => Ok(CardValue::Ace),
            _ => Err("Invalid card value"),
        }
    }
}

impl Into<u8> for CardValue {
    fn into(self) -> u8 {
        match self {
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
            CardValue::Jack => 11,
            CardValue::Knight => 12,
            CardValue::Queen => 13,
            CardValue::King => 14,
            CardValue::Ace => 15,
        }
    }
}

impl PartialOrd for CardValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardValue {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value: u8 = (*self).into();
        let other_value: u8 = (*other).into();
        self_value.cmp(&other_value)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Hand {
    HighCard(CardValue, CardValue, CardValue, CardValue, CardValue),
    Pair(CardValue, CardValue, CardValue, CardValue),
    TwoPair(CardValue, CardValue, CardValue),
    ThreeOfAKind(CardValue, CardValue, CardValue),
    // Straight(CardValue),
    // Flush(CardValue, CardValue, CardValue, CardValue, CardValue),
    FullHouse(CardValue, CardValue),
    FourOfAKind(CardValue, CardValue),
    // StraightFlush(CardValue),
    // RoyalFlush,
    FiveOfAKind(CardValue), // Only applies to certain games
}

impl TryFrom<&[CardValue]> for Hand {
    type Error = &'static str;

    fn try_from(hand: &[CardValue]) -> Result<Self, Self::Error> {
        let mut counter: HashMap<CardValue, u8> = HashMap::new();
        for card in hand.iter() {
            *counter.entry(*card).or_insert(0) += 1;
        }
        let sorted_vec = counter
            .iter()
            .sorted_unstable_by_key(|(_k, v)| *v) // (*v, *k) For part 2?
            .rev()
            .take(5)
            .collect_vec();
        let hand_type = match sorted_vec.as_slice() {
            &[(c, 5)] => Self::FiveOfAKind(*c),
            &[(c1, 4), c2] => Self::FourOfAKind(*c1, *c2.0),
            &[(c1, 3), (c2, 2)] => Self::FullHouse(*c1, *c2),
            &[(c1, 3), c2, c3] => Self::ThreeOfAKind(*c1, *c2.0, *c3.0),
            &[(c1, 2), (c2, 2), c3] => Self::TwoPair(*c1, *c2, *c3.0),
            &[(c1, 2), c2, c3, c4] => Self::Pair(*c1, *c2.0, *c3.0, *c4.0),
            &[c1, c2, c3, c4, c5] => Self::HighCard(*c1.0, *c2.0, *c3.0, *c4.0, *c5.0),
            _ => return Err("Invalid hand"),
        };
        assert_eq!(hand.len(), 5); // At this point this should always be true
        Ok(hand_type)
    }
}

impl TryFrom<&str> for Hand {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let values = s
            .chars()
            .filter_map(|c| CardValue::try_from(c).ok())
            .collect_vec();
        Self::try_from(values.as_slice())
    }
}

impl Into<u8> for Hand {
    fn into(self) -> u8 {
        match self {
            Self::HighCard(_, _, _, _, _) => 1,
            Self::Pair(_, _, _, _) => 2,
            Self::TwoPair(_, _, _) => 3,
            Self::ThreeOfAKind(_, _, _) => 4,
            Self::FullHouse(_, _) => 5,
            Self::FourOfAKind(_, _) => 6,
            Self::FiveOfAKind(_) => 7,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value: u8 = (*self).into();
        let other_value: u8 = (*other).into();
        match self_value.cmp(&other_value) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let self_vec = match self {
                    Self::HighCard(a, b, c, d, e) => vec![a, b, c, d, e],
                    Self::Pair(a, b, c, d) => vec![a, b, c, d],
                    Self::TwoPair(a, b, c) => vec![a, b, c],
                    Self::ThreeOfAKind(a, b, c) => vec![a, b, c],
                    Self::FullHouse(a, b) => vec![a, b],
                    Self::FourOfAKind(a, b) => vec![a, b],
                    Self::FiveOfAKind(a) => vec![a, a, a, a, a],
                };
                let other_vec = match other {
                    Self::HighCard(a, b, c, d, e) => vec![a, b, c, d, e],
                    Self::Pair(a, b, c, d) => vec![a, b, c, d],
                    Self::TwoPair(a, b, c) => vec![a, b, c],
                    Self::ThreeOfAKind(a, b, c) => vec![a, b, c],
                    Self::FullHouse(a, b) => vec![a, b],
                    Self::FourOfAKind(a, b) => vec![a, b],
                    Self::FiveOfAKind(a) => vec![a, a, a, a, a],
                };
                self_vec
                    .iter()
                    .zip(other_vec.iter())
                    .map(|(a, b)| a.cmp(b))
                    .filter(|o| *o != Ordering::Equal)
                    .next()
                    .unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BidHand {
    pub hand: Hand,
    pub bid: usize,
}

impl TryFrom<&str> for BidHand {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        let hand_str = split.next().ok_or("Invalid hand")?;
        let hand = Hand::try_from(hand_str)?;
        let bid = split
            .filter_map(|s| s.parse::<usize>().ok())
            .next()
            .ok_or("Invalid bid")?;
        Ok(BidHand { hand, bid })
    }
}

pub fn parse_input(s: &str) -> Vec<BidHand> {
    s.lines()
        .filter_map(|line| BidHand::try_from(line).ok())
        .collect_vec()
}

impl Ord for BidHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for BidHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(EXAMPLE1);
        assert_eq!(
            parsed,
            vec![
                BidHand {
                    hand: Hand::try_from("32T3K").unwrap(),
                    bid: 765
                },
                BidHand {
                    hand: Hand::try_from("T55J5").unwrap(),
                    bid: 684
                },
                BidHand {
                    hand: Hand::try_from("KK677").unwrap(),
                    bid: 28
                },
                BidHand {
                    hand: Hand::try_from("KTJJT").unwrap(),
                    bid: 220
                },
                BidHand {
                    hand: Hand::try_from("QQQJA").unwrap(),
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn test_sorting() {
        let mut hands = parse_input(EXAMPLE1);
        hands.sort_unstable();
        let bids = hands.iter().map(|h| h.bid).collect_vec();
        assert_eq!(bids, vec![765, 220, 28, 684, 483]);
    }

    #[test]
    fn test_part1_example() {
        let mut hands = parse_input(EXAMPLE1);
        hands.sort_unstable();
        let result: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("day07.txt");
        let mut hands = parse_input(input);
        hands.sort_unstable();
        let result: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
        assert_eq!(result, 249644759);
    }
}
