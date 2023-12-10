use advent_of_code_common::cards::{Card, CardValue, Hand5};
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BidHand {
    pub hand: Hand5,
    pub bid: usize,
}

impl FromStr for BidHand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let hand = split.next().ok_or("Invalid hand")?.parse()?;
        let bid = split
            .find_map(|s| s.parse::<usize>().ok())
            .ok_or("Invalid bid")?;
        Ok(Self { hand, bid })
    }
}

fn jokers_for_jacks(input: &[BidHand]) -> Vec<BidHand> {
    input
        .iter()
        .map(|h| {
            let cards = h.hand.cards.map(|c| {
                if c.value == CardValue::Jack {
                    Card {
                        value: CardValue::Joker,
                        suit: c.suit,
                    }
                } else {
                    c
                }
            });
            BidHand {
                hand: Hand5 { cards },
                bid: h.bid,
            }
        })
        .collect_vec()
}

pub fn parse_input(s: &str, jokers: bool) -> Vec<BidHand> {
    let input = s.lines().filter_map(|line| line.parse().ok()).collect_vec();
    if jokers {
        jokers_for_jacks(&input)
    } else {
        input
    }
}

impl Ord for BidHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp_with_order(&other.hand)
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
    fn test_sorting() {
        let mut hands = parse_input(EXAMPLE1, false);
        hands.sort_unstable();
        let bids = hands.iter().map(|h| h.bid).collect_vec();
        assert_eq!(bids, vec![765, 220, 28, 684, 483]);
    }

    #[test]
    fn test_part1_example() {
        let mut hands = parse_input(EXAMPLE1, false);
        hands.sort_unstable();
        let result: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("day07.txt");
        let mut hands = parse_input(input, false);
        hands.sort_unstable();
        let result: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
        assert_eq!(result, 250232501);
    }

    #[test]
    fn test_part2_example() {
        let mut hands = parse_input(EXAMPLE1, true)
            .iter()
            .map(|h| BidHand {
                hand: h.hand.joker_swap(),
                bid: h.bid,
            })
            .collect_vec();
        let j_hand = BidHand {
            hand: "***3A".parse().unwrap(),
            bid: 1,
        };
        hands.push(j_hand);
        hands.sort_unstable_by(|a, b| a.hand.cmp_with_order(&b.hand));
        dbg!(&hands[2].hand);
        dbg!(&hands[2].hand.joker_swap());
        dbg!(hands[2]
            .hand
            .joker_swap()
            .cmp_with_order(&hands[0].hand.joker_swap()));
        dbg!(hands[2]
            .hand
            .joker_swap()
            .cmp_with_order(&hands[1].hand.joker_swap()));
        dbg!(hands[2]
            .hand
            .joker_swap()
            .cmp_with_order(&hands[2].hand.joker_swap()));
        dbg!(hands[2]
            .hand
            .joker_swap()
            .cmp_with_order(&hands[3].hand.joker_swap()));
        dbg!(hands[2]
            .hand
            .joker_swap()
            .cmp_with_order(&hands[4].hand.joker_swap()));
        let result: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
        assert_eq!(result, 5905);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("day07.txt");
        let mut hands = parse_input(input, true)
            .iter()
            .map(|h| BidHand {
                hand: h.hand.joker_swap(),
                bid: h.bid,
            })
            .collect_vec();
        hands.sort_unstable_by(|a, b| a.hand.cmp_with_order(&b.hand));
        let result: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
        assert_ne!(result, 249066234);
    }
}
