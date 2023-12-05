use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
    
    // TODO: can probably simplify these .next() calls
    let winning_numbers = split_parts.next().unwrap();
    let parsed_ints = winning_numbers.split(" ").filter(|x| *x != "").map(|x| x.parse::<u32>().unwrap());
    result.winning_numbers.extend(parsed_ints);
    
    let my_numbers = split_parts.next().unwrap();
    let parsed_ints = my_numbers.split(" ").filter(|x| *x != "").map(|x| x.parse::<u32>().unwrap());
    result.my_numbers.extend(parsed_ints);
    
    return result;
}

fn part2(card_contents:&str) -> u32 {
    let mut total_cards = 0;
    let mut copy_counter: HashMap<u32, u32> = HashMap::new();
    
    for line in card_contents.split("\n") {
        let parsed_card = parse_card(line);
        
        // calculate how many winning numbers we have
        let matching_numbers = parsed_card.my_numbers.iter().filter(|&x| parsed_card.winning_numbers.contains(&x)).collect::<Vec<&u32>>().len() as u32;
        
        // we are always going to have at least 1 istance of a card in our stack, 
        // so we make sure that key exists 
        if !copy_counter.contains_key(&parsed_card.card_id) {
            copy_counter.insert(parsed_card.card_id, 1);
        }

        // we want to increment the following X cards Y times 
        // Y: the amount of matching numbers we have
        // X: the amount of copies of the card we have
        for _x in 0..copy_counter[&parsed_card.card_id] {
            for y in 1..matching_numbers+1 {
                if !copy_counter.contains_key(&(parsed_card.card_id + y)) {
                    copy_counter.insert(parsed_card.card_id + y, 1);
                }
                copy_counter.insert(parsed_card.card_id + y, &copy_counter[&(&parsed_card.card_id + y)]+1);
            }
        }
        
        println!("parsed card: {} - {:?} - adding {} copies", parsed_card.card_id, matching_numbers, copy_counter[&parsed_card.card_id]);
        
        // add our copy of cards
        total_cards += copy_counter[&parsed_card.card_id];
    }

    return total_cards;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 30);
    }
}