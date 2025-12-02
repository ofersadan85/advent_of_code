use advent_of_code_macros::aoc_tests;
use std::collections::{HashMap, HashSet};

fn buyer_prices(buyer: isize) -> Vec<isize> {
    let magic_number = 16_777_215;
    (0..2000).fold(Vec::with_capacity(2000), |mut prices, _| {
        let mut buyer = *prices.last().unwrap_or(&buyer);
        buyer = ((buyer << 6) ^ buyer) & magic_number;
        buyer = (buyer >> 5) ^ buyer;
        buyer = (buyer.overflowing_shl(11).0 ^ buyer) & magic_number;
        prices.push(buyer);
        prices
    })
}

fn all_buyers_prices(buyers: Vec<isize>) -> Vec<Vec<isize>> {
    let mut all_prices = Vec::with_capacity(buyers.len());
    all_prices.extend(buyers.into_iter().map(buyer_prices));
    all_prices
}

fn sum_final_prices(buyers: Vec<isize>) -> isize {
    all_buyers_prices(buyers)
        .iter()
        .map(|p| p.last().unwrap_or(&0))
        .sum()
}

fn map_best_price(buyers: Vec<isize>) -> isize {
    let mut result = HashMap::with_capacity(2000 * buyers.len());
    let mut buyer_sequences = HashSet::with_capacity(2000);
    let prices = all_buyers_prices(buyers);
    for buyer in prices {
        let buyer: Vec<isize> = buyer.iter().map(|v| v % 10).collect();
        for seq in buyer.windows(5) {
            let diffs = [
                seq[1] - seq[0],
                seq[2] - seq[1],
                seq[3] - seq[2],
                seq[4] - seq[3],
            ];
            if buyer_sequences.insert(diffs) {
                result
                    .entry(diffs)
                    .and_modify(|e| *e += seq[4])
                    .or_insert(seq[4]);
            }
        }
        buyer_sequences.clear();
    }
    *result.values().max().unwrap_or(&0)
}

#[aoc_tests]
mod tests {
    #[test]
    fn example_1() {
        let buyers = vec![1, 10, 100, 2024];
        assert_eq!(sum_final_prices(buyers), 37327623);
    }

    #[test]
    fn part_1() {
        let buyers = read_input().lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(sum_final_prices(buyers), 20332089158);
    }

    #[test]
    fn example_2() {
        let buyers = vec![1, 2, 3, 2024];
        assert_eq!(map_best_price(buyers), 23);
    }

    #[test]
    fn part_2() {
        let buyers = read_input().lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(map_best_price(buyers), 2191);
    }
}
