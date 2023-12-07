use std::collections::HashMap;
use std::iter::zip;
use core::cmp::Ordering;

#[derive(Debug, Clone)]
struct Hand {
    bid: u32,
    cards: Vec<u32>,
    highest_val: u32,
}

const FIVE_OF_A_KIND: u32 = 600;
const FOUR_OF_A_KIND: u32 = 500;
const FULL_HOUSE: u32 = 400;
const THREE_OF_A_KIND: u32 = 300;
const TWO_PAIR: u32 = 200;
const ONE_PAIR: u32 = 100;
const HIGHEST_CARD: u32 = 0;

impl Hand {
    fn calc_highest_point(&mut self) {

        if self.check_x_of_kind(5) {
            self.highest_val = FIVE_OF_A_KIND;
        } else if self.check_x_of_kind(4) {
            self.highest_val = FOUR_OF_A_KIND;
        } else if self.is_full_house() {
            self.highest_val = FULL_HOUSE;
        } else if self.check_x_of_kind(3) {
            self.highest_val = THREE_OF_A_KIND;
        } else if self.check_double_pair() {
            self.highest_val = TWO_PAIR;
        } else if self.check_x_of_kind(2) {
            self.highest_val = ONE_PAIR;
        } else {
            self.highest_val = HIGHEST_CARD;
        }
    }

    fn is_full_house(&self) -> bool {
        let mut triple_pair: u32 = 0;
        for candidate in &self.cards {
            let mut count = 0;
            for c in &self.cards {
                if candidate == c {
                    count += 1;
                }

                if count >= 3 && triple_pair == 0 {
                    triple_pair = *candidate;

                }
            }
        }
        
        if triple_pair == 0 {
            return false;
        }

        for candidate in &self.cards {
            let mut count = 0;
            for c in &self.cards {
                if candidate == c {
                    count += 1;
                }

                if count >= 2 && *candidate != triple_pair {
                    return true;
                }
            }
        }
        return false;
    }

    fn check_double_pair(&self) -> bool {
        let mut found_pairs: Vec<u32> = Vec::new();
        for candidate in &self.cards {
            let mut count = 0;
            for c in &self.cards {
                if candidate == c {
                    count += 1;
                }

                if count >= 2 && !found_pairs.contains(candidate) {
                    found_pairs.push(*candidate);
                }
            }

            if found_pairs.len() == 2 {
                return true;
            }
        }
        return false;
    }

    fn check_x_of_kind(&self, n:u32) -> bool {
        for candidate in &self.cards {
            let mut count = 0;
            for c in &self.cards {
                if candidate == c {
                    count += 1;
                }
            }

            if count >= n {
                return true;
            }
        }
        return false;
    }

    fn compare(&self, other_hand:Hand) -> Ordering {
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

fn part1(src_info:&str) -> u32 {
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
            highest_val: 0
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

    hands.sort_by(|a, b| a.compare(b.clone()));

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += ((i as u32)+1) * hand.bid;
    }

    return total_winnings;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, 5905);
    }
}