use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum CardValue {
    Joker = 0,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardValue::Joker => write!(f, "*"),
            CardValue::Two => write!(f, "2"),
            CardValue::Three => write!(f, "3"),
            CardValue::Four => write!(f, "4"),
            CardValue::Five => write!(f, "5"),
            CardValue::Six => write!(f, "6"),
            CardValue::Seven => write!(f, "7"),
            CardValue::Eight => write!(f, "8"),
            CardValue::Nine => write!(f, "9"),
            CardValue::Ten => write!(f, "T"),
            CardValue::Jack => write!(f, "J"),
            CardValue::Queen => write!(f, "Q"),
            CardValue::King => write!(f, "K"),
            CardValue::Ace => write!(f, "A"),
        }
    }
}

impl TryFrom<char> for CardValue {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' | '*' => Ok(CardValue::Joker),
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
            'K' => Ok(CardValue::King),
            'Q' => Ok(CardValue::Queen),
            'A' => Ok(CardValue::Ace),
            _ => Err("Invalid card value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardSuit::Clubs => write!(f, "♣"),
            CardSuit::Diamonds => write!(f, "♦"),
            CardSuit::Hearts => write!(f, "♥"),
            CardSuit::Spades => write!(f, "♠"),
        }
    }
}

impl TryFrom<char> for CardSuit {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'C' | 'c' | '♣' => Ok(CardSuit::Clubs),
            'D' | 'd' | '♦' => Ok(CardSuit::Diamonds),
            'H' | 'h' | '♥' => Ok(CardSuit::Hearts),
            'S' | 's' | '♠' => Ok(CardSuit::Spades),
            _ => Err("Invalid card suit"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.chars();
        Ok(Card {
            value: chars.next().ok_or("Invalid card")?.try_into()?,
            suit: chars.next().ok_or("Invalid card")?.try_into()?,
        })
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ValuedHand {
    HighCard(CardValue, CardValue, CardValue, CardValue, CardValue),
    Pair(CardValue, CardValue, CardValue, CardValue),
    TwoPair(CardValue, CardValue, CardValue),
    ThreeOfAKind(CardValue, CardValue, CardValue),
    Straight(CardValue),
    Flush(
        CardSuit,
        CardValue,
        CardValue,
        CardValue,
        CardValue,
        CardValue,
    ),
    FullHouse(CardValue, CardValue),
    FourOfAKind(CardValue, CardValue),
    StraightFlush(CardSuit, CardValue),
    RoyalFlush(CardSuit),
    FiveOfAKind(CardValue),                // Only applies to certain games
    FiveOfAKindFlush(CardSuit, CardValue), // Only applies to certain games
}

impl Display for ValuedHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighCard(a, b, c, d, e) => {
                write!(f, "High card: {a} {b} {c} {d} {e}")
            }
            Self::Pair(a, b, c, d) => write!(f, "Pair: {a}s ({b} {c} {d})"),
            Self::TwoPair(a, b, c) => write!(f, "Two pair: {a}s & {b}s ({c})"),
            Self::ThreeOfAKind(a, b, c) => write!(f, "Three of a kind: {a}s ({b} {c})"),
            Self::Straight(a) => write!(f, "Straight: {a}"),
            Self::Flush(s, a, b, c, d, e) => {
                write!(f, "Flush {s}: {a} {b} {c} {d} {e}")
            }
            Self::FullHouse(a, b) => write!(f, "Full house: {a}s over {b}s"),
            Self::FourOfAKind(a, b) => write!(f, "Four of a kind: {a}s ({b})"),
            Self::StraightFlush(s, a) => write!(f, "Straight flush {s}: {a}"),
            Self::RoyalFlush(s) => write!(f, "Royal flush {s}"),
            Self::FiveOfAKind(a) => write!(f, "Five of a kind: {a}"),
            Self::FiveOfAKindFlush(s, a) => write!(f, "Five of a kind flush {s}: {a}"),
        }
    }
}

impl Into<u8> for ValuedHand {
    fn into(self) -> u8 {
        match self {
            Self::HighCard(..) => 1,
            Self::Pair(..) => 2,
            Self::TwoPair(..) => 3,
            Self::ThreeOfAKind(..) => 4,
            Self::Straight(..) => 5,
            Self::Flush(..) => 6,
            Self::FullHouse(..) => 7,
            Self::FourOfAKind(..) => 8,
            Self::StraightFlush(..) => 9,
            Self::RoyalFlush(..) => 10,
            Self::FiveOfAKind(..) => 11,
            Self::FiveOfAKindFlush(..) => 12,
        }
    }
}

impl ValuedHand {
    fn cmp_hand_type(&self, other: &Self) -> Ordering {
        let self_value: u8 = (*self).into();
        let other_value: u8 = (*other).into();
        self_value.cmp(&other_value)
    }
}

impl PartialOrd for ValuedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValuedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::HighCard(a, b, c, d, e), Self::HighCard(f, g, h, i, j)) => vec![a, b, c, d, e]
                .iter()
                .zip(vec![f, g, h, i, j].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::Pair(a, b, c, d), Self::Pair(e, f, g, h)) => vec![a, b, c, d]
                .iter()
                .zip(vec![e, f, g, h].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::TwoPair(a, b, c), Self::TwoPair(d, e, f)) => vec![a, b, c]
                .iter()
                .zip(vec![d, e, f].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::ThreeOfAKind(a, b, c), Self::ThreeOfAKind(d, e, f)) => vec![a, b, c]
                .iter()
                .zip(vec![d, e, f].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::Straight(a), Self::Straight(b)) => a.cmp(b),
            (Self::Flush(_, a, b, c, d, e), Self::Flush(_, f, g, h, i, j)) => vec![a, b, c, d, e]
                .iter()
                .zip(vec![f, g, h, i, j].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::FullHouse(a, b), Self::FullHouse(c, d)) => vec![a, b]
                .iter()
                .zip(vec![c, d].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::FourOfAKind(a, b), Self::FourOfAKind(c, d)) => vec![a, b]
                .iter()
                .zip(vec![c, d].iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::StraightFlush(_, a), Self::StraightFlush(_, b)) => a.cmp(b),
            (Self::RoyalFlush(_), Self::RoyalFlush(_)) => Ordering::Equal,
            (Self::FiveOfAKind(a), Self::FiveOfAKind(b)) => a.cmp(b),
            _ => self.cmp_hand_type(other),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Hand5 {
    pub cards: [Card; 5],
}

impl FromStr for Hand5 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = if s.len() == 5 {
            let suits = "HDSCC"; // Just to make sure we don't guarantee a flush
            s.chars()
                .zip(suits.chars())
                .filter_map(|(value, suit)| Some(format!("{value}{suit}").parse::<Card>().ok()?))
                .collect_vec()
        } else {
            s.split_whitespace()
                .filter_map(|s| s.parse::<Card>().ok())
                .collect_vec()
        };
        let cards: [Card; 5] = cards.try_into().map_err(|_| "Invalid hand")?;
        if cards.len() == 5 {
            Ok(Self { cards })
        } else {
            Err("Invalid hand")
        }
    }
}

impl Hand5 {
    fn flush_suit(&self) -> Option<CardSuit> {
        let suits: HashSet<CardSuit> = HashSet::from_iter(self.cards.iter().map(|c| c.suit));
        if suits.len() == 1 {
            suits.iter().next().copied()
        } else {
            None
        }
    }

    fn evaluate(&self) -> ValuedHand {
        let flush_suit = self.flush_suit();
        let sorted_cards = self.cards.iter().sorted_unstable().collect_vec();
        let is_straight = sorted_cards
            .windows(2)
            .all(|w| w[0].value as u8 + 1 == w[1].value as u8);
        match (flush_suit, is_straight, sorted_cards.last()) {
            (Some(suit), true, Some(card)) if card.value == CardValue::Ace => {
                return ValuedHand::RoyalFlush(suit)
            }
            (Some(suit), true, Some(card)) => return ValuedHand::StraightFlush(suit, card.value),
            (None, true, Some(card)) => return ValuedHand::Straight(card.value),
            (Some(suit), false, _) => {
                return ValuedHand::Flush(
                    suit,
                    sorted_cards[0].value,
                    sorted_cards[1].value,
                    sorted_cards[2].value,
                    sorted_cards[3].value,
                    sorted_cards[4].value,
                )
            }
            _ => self.evaluate_non_flush(),
        }
    }

    fn evaluate_non_flush(&self) -> ValuedHand {
        use ValuedHand::{
            FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPair,
        };
        let mut counter: HashMap<CardValue, u8> = HashMap::new();
        for card in self.cards.iter() {
            *counter.entry(card.value).or_insert(0) += 1;
        }
        let sorted_vec = counter
            .iter()
            .sorted_unstable_by_key(|(k, v)| (*v, *k))
            .rev()
            .take(5)
            .collect_vec();
        let hand_type = match sorted_vec.as_slice() {
            &[(c, 5)] => FiveOfAKind(*c),
            &[(c1, 4), c2] => FourOfAKind(*c1, *c2.0),
            &[(c1, 3), (c2, 2)] => FullHouse(*c1, *c2),
            &[(c1, 3), c2, c3] => ThreeOfAKind(*c1, *c2.0, *c3.0),
            &[(c1, 2), (c2, 2), c3] => TwoPair(*c1, *c2, *c3.0),
            &[(c1, 2), c2, c3, c4] => Pair(*c1, *c2.0, *c3.0, *c4.0),
            &[c1, c2, c3, c4, c5] => HighCard(*c1.0, *c2.0, *c3.0, *c4.0, *c5.0),
            _ => unreachable!("Invalid hand"),
        };
        hand_type
    }

    pub fn cmp_hand_type(&self, other: &Self) -> Ordering {
        let self_value = self.joker_swap().evaluate();
        let other_value = other.joker_swap().evaluate();
        self_value.cmp_hand_type(&other_value)
    }

    pub fn cmp_with_order(&self, other: &Self) -> Ordering {
        let cmp_type = self.cmp_hand_type(other);
        if cmp_type != Ordering::Equal {
            return cmp_type;
        } else {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| *o != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal)
        }
    }

    pub fn replace_value(&self, old: CardValue, new: CardValue) -> Self {
        let mut cards = self.cards;
        for card in cards.iter_mut() {
            if card.value == old {
                card.value = new;
            }
        }
        Self { cards }
    }

    /// Find the ideal joker swap for this hand
    /// This is the swap that gives the highest valued hand
    ///
    pub fn joker_swap(&self) -> Self {
        let mut counter: HashMap<CardValue, u8> = HashMap::new();
        for card in self.cards.iter() {
            *counter.entry(card.value).or_insert(0) += 1;
        }
        match counter.get(&CardValue::Joker) {
            Some(0) | None => return *self,
            Some(1) | Some(2) => return self.joker_swap_straight(),
            // 3 Jokers or higher are always better swapped for the highest card
            Some(_) => return self.joker_swap_highest(),
        }
    }

    fn joker_swap_straight(&self) -> Self {
        let straight_possible = self
            .cards
            .iter()
            .filter(|c| c.value != CardValue::Joker)
            .minmax()
            .into_option()
            .map(|(min, max)| max.value as u8 - min.value as u8 <= 4)
            .unwrap_or(false);
        if !straight_possible {
            return self.joker_swap_highest();
        }
        
        todo!("Implement joker swap straight")
    }

    fn joker_swap_highest(&self) -> Self {
        let target_value = self
            .cards
            .iter()
            .map(|c| c.value)
            .filter(|&v| v != CardValue::Joker)
            .max()
            .unwrap_or(CardValue::Ace);
        self.replace_value(CardValue::Joker, target_value)
    }

    // pub fn replace_jokers(&self) -> Self {
    //     [
    //         self.replace_value(CardValue::Joker, CardValue::Ace),
    //         self.replace_value(CardValue::Joker, CardValue::King),
    //         self.replace_value(CardValue::Joker, CardValue::Queen),
    //         self.replace_value(CardValue::Joker, CardValue::Jack),
    //         self.replace_value(CardValue::Joker, CardValue::Ten),
    //         self.replace_value(CardValue::Joker, CardValue::Nine),
    //         self.replace_value(CardValue::Joker, CardValue::Eight),
    //         self.replace_value(CardValue::Joker, CardValue::Seven),
    //         self.replace_value(CardValue::Joker, CardValue::Six),
    //         self.replace_value(CardValue::Joker, CardValue::Five),
    //         self.replace_value(CardValue::Joker, CardValue::Four),
    //         self.replace_value(CardValue::Joker, CardValue::Three),
    //         self.replace_value(CardValue::Joker, CardValue::Two),
    //     ]
    //     .iter()
    //     .max()
    //     .copied()
    //     .expect("There should be at least one candidate")
    // }
}

impl PartialOrd for Hand5 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand5 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluate().cmp(&other.evaluate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joker_swap() {
        let expected = "23456".parse::<Hand5>().unwrap();
        let hand = "2345*".parse::<Hand5>().unwrap();
        assert_eq!(hand.joker_swap(), expected);
        let hand = "2*45*".parse::<Hand5>().unwrap();
        assert_eq!(hand.joker_swap(), expected);
        let hand = "234**".parse::<Hand5>().unwrap();
        assert_eq!(hand.joker_swap(), expected);
        let hand = "23***".parse::<Hand5>().unwrap();
        let expected = "23333".parse::<Hand5>().unwrap();
        assert_eq!(hand.joker_swap(), expected);
        let hand = "2****".parse::<Hand5>().unwrap();
        let expected = "22222".parse::<Hand5>().unwrap();
        assert_eq!(hand.joker_swap(), expected);
    }

    // #[test]
    // fn joker_cmp() {
    //     let a = "***3A".parse::<Hand5>().unwrap();
    //     let b = "AAA32".parse::<Hand5>().unwrap();
    //     assert_eq!(a.cmp_with_order(&b), Ordering::Greater, "{}", a.evaluate());
    //     let b = "AAA3A".parse::<Hand5>().unwrap();
    //     assert_eq!(a.cmp_with_order(&b), Ordering::Less);
    //     let b = "2A2JJ".parse::<Hand5>().unwrap();
    //     assert_eq!(a.cmp_with_order(&b), Ordering::Less);
    //     let b = "JJJJJ".parse::<Hand5>().unwrap();
    //     assert_eq!(a.cmp_with_order(&b), Ordering::Equal);
    // }

    #[test]
    fn card_comparison() {
        use CardValue::*;
        assert!(Joker == Joker);
        assert!(Joker < Two);
        assert!(Two == Two);
        assert!(Two < Three);
        assert!(Three == Three);
        assert!(Three < Four);
        assert!(Four == Four);
        assert!(Four < Five);
        assert!(Five == Five);
        assert!(Five < Six);
        assert!(Six == Six);
        assert!(Six < Seven);
        assert!(Seven == Seven);
        assert!(Seven < Eight);
        assert!(Eight == Eight);
        assert!(Eight < Nine);
        assert!(Nine == Nine);
        assert!(Nine < Ten);
        assert!(Ten == Ten);
        assert!(Ten < Jack);
        assert!(Jack == Jack);
        assert!(Jack < Queen);
        assert!(Queen == Queen);
        assert!(Queen < King);
        assert!(King == King);
        assert!(King < Ace);
        assert!(Ace == Ace);
    }

    #[test]
    fn card_suit_comparison() {
        use CardSuit::*;
        assert!(Clubs == Clubs);
        assert!(Diamonds == Diamonds);
        assert!(Hearts == Hearts);
        assert!(Spades == Spades);
    }

    #[test]
    fn hand_value() {
        let high1 = "AT387".parse::<Hand5>().unwrap();
        let high2 = "3TA87".parse::<Hand5>().unwrap();
        let high3 = "632AT".parse::<Hand5>().unwrap();
        assert_eq!(high1.cmp(&high2), Ordering::Equal);
        assert_eq!(high1.cmp_hand_type(&high2), Ordering::Equal);
        assert_eq!(high1.cmp_with_order(&high2), Ordering::Greater);
        assert_eq!(high1.cmp(&high3), Ordering::Greater);
        assert_eq!(high2.cmp_hand_type(&high3), Ordering::Equal);
        let pair = "ATT38".parse::<Hand5>().unwrap();
        assert_eq!(high1.cmp(&pair), Ordering::Less);
        let two_pair = "ATT33".parse::<Hand5>().unwrap();
        assert_eq!(pair.cmp(&two_pair), Ordering::Less);
        let three_of_a_kind = "AT333".parse::<Hand5>().unwrap();
        assert_eq!(two_pair.cmp(&three_of_a_kind), Ordering::Less);
        let straight = "23456".parse::<Hand5>().unwrap();
        assert_eq!(three_of_a_kind.cmp(&straight), Ordering::Less);
        let full_house = "23232".parse::<Hand5>().unwrap();
        assert_eq!(straight.cmp(&full_house), Ordering::Less);
        let four_of_a_kind = "TTTT3".parse::<Hand5>().unwrap();
        assert_eq!(full_house.cmp(&four_of_a_kind), Ordering::Less);
        let five_of_a_kind = "TTTTT".parse::<Hand5>().unwrap();
        assert_eq!(four_of_a_kind.cmp(&five_of_a_kind), Ordering::Less);
    }
}
