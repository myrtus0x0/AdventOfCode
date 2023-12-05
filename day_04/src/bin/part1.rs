use core::num;
use std::fs;

struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

fn parse_card(line:&str) -> Card {
    let mut parts = line.split(":");
    let card_id = parts.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();

    let mut result = Card{
        card_id: card_id,
        winning_numbers: Vec::new(),
        my_numbers: Vec::new(),
    };
    let mut split_parts = parts.next().unwrap().split("|");
    
    let winning_numbers = split_parts.next().unwrap();
    let parsed_ints = winning_numbers.split(" ").filter(|x| *x != "").map(|x| x.parse::<u32>().unwrap());
    result.winning_numbers.extend(parsed_ints);
    
    let my_numbers = split_parts.next().unwrap();
    let parsed_ints = my_numbers.split(" ").filter(|x| *x != "").map(|x| x.parse::<u32>().unwrap());
    result.my_numbers.extend(parsed_ints);
    
    return result;
}

fn part1(card_contents:&str) -> u32 {
    let split_lines = card_contents.split("\n");
    
    let mut total_sum: u32 = 0;
    for line in split_lines {
        let parsed_card = parse_card(line);
        let matching_numbers = parsed_card.my_numbers.iter().filter(|&x| parsed_card.winning_numbers.contains(&x)).collect::<Vec<&u32>>().len() as u32;
        
        let mut to_add = 0;
        if matching_numbers == 1 {
            to_add = 1;
        } else if matching_numbers > 1 {
            let base: u32 = 2;
            to_add = base.pow(matching_numbers-1);
        }
        total_sum += to_add;
    }

    return total_sum;
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
        let result = part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 13);
    }
}