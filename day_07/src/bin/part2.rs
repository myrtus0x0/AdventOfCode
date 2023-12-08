use core::cmp::Ordering;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

#[derive(Debug, Clone)]
struct Hand {
    bid: u32,
    cards: Vec<u32>,
    highest_val: u32,
}

// hand types we care about
const FIVE_OF_A_KIND: u32 = 600;
const FOUR_OF_A_KIND: u32 = 500;
const FULL_HOUSE: u32 = 400;
const THREE_OF_A_KIND: u32 = 300;
const TWO_PAIR: u32 = 200;
const ONE_PAIR: u32 = 100;
const HIGHEST_CARD: u32 = 0;

impl Hand {
    fn calc_highest_point(&mut self) {
        let card_counts = self.cards.iter().counts();
        let counts = card_counts
            .iter()
            .filter(|&(&x, _)| *x != 1)
            .map(|(_, &v)| v)
            .collect::<Vec<_>>();

        // count how many jokers we have and our max number of duplicates
        let n_jokers = self
            .cards
            .clone()
            .into_iter()
            .filter(|x| *x == 1 as u32)
            .collect::<Vec<u32>>()
            .len() as u32;
        let max_duplicates = counts.clone().into_iter().max().unwrap_or(0);

        match (max_duplicates as u32, n_jokers) {
            (a, b) if a + b == 5 => self.highest_val = FIVE_OF_A_KIND,
            (a, b) if a + b == 4 => self.highest_val = FOUR_OF_A_KIND,
            (3, 0) => {
                if counts.contains(&&2) {
                    self.highest_val = FULL_HOUSE
                } else {
                    self.highest_val = THREE_OF_A_KIND
                }
            }
            (2, _) => {
                // with knowing jokers and pairs we can group things into their best possible
                let pairs = counts.into_iter().filter(|&v| v == 2).count();
                match (pairs, n_jokers) {
                    (2, 1) => self.highest_val = FULL_HOUSE,
                    (2, 0) => self.highest_val = TWO_PAIR,
                    (1, 1) => self.highest_val = THREE_OF_A_KIND,
                    _ => self.highest_val = ONE_PAIR,
                }
            }
            (a, b) if a + b == 3 => self.highest_val = THREE_OF_A_KIND,
            (a, b) if a + b == 2 => self.highest_val = ONE_PAIR,
            _ => self.highest_val = HIGHEST_CARD,
        }
    }

    fn compare(&self, other_hand: Hand) -> Ordering {
        if self.highest_val > other_hand.highest_val {
            return Ordering::Greater;
        } else if self.highest_val < other_hand.highest_val {
            return Ordering::Less;
        } else {
            for (x, y) in zip(self.cards.clone(), other_hand.cards.clone()) {
                if x > y {
                    return Ordering::Greater;
                } else if y > x {
                    return Ordering::Less;
                }
            }
        }
        return Ordering::Equal;
    }
}

fn part2(src_info: &str) -> u32 {
    let mut total_winnings = 0;

    let mut card_map: HashMap<char, u32> = HashMap::new();
    card_map.insert('T', 10);
    card_map.insert('J', 1);
    card_map.insert('Q', 11);
    card_map.insert('K', 12);
    card_map.insert('A', 13);

    let mut hands = Vec::new();
    for hand_str in src_info.split("\n") {
        let mut hand = Hand {
            bid: hand_str.split(" ").nth(1).unwrap().parse::<u32>().unwrap(),
            cards: Vec::new(),
            highest_val: 0,
        };

        for c in hand_str.split(" ").nth(0).unwrap().chars() {
            if c.is_digit(10) {
                hand.cards.push(c.to_digit(10).unwrap());
            } else {
                hand.cards.push(card_map[&c]);
            }
        }

        hand.calc_highest_point();
        hands.push(hand);
    }

    hands.sort_unstable_by(|a, b| a.compare(b.clone()));

    for (i, hand) in hands.iter().enumerate() {
        println!("{} += {} * {}", total_winnings, hand.bid, (i as u32) + 1);
        total_winnings += (i as u32) * hand.bid + hand.bid;
    }

    return total_winnings;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    assert_eq!(253473930, answer);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 5905);
    }
}
