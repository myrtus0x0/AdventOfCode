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
    
    let winning_numbers = split_parts.next().unwrap();
    let parsed_ints = winning_numbers.split(" ").filter(|x| *x != "").map(|x| x.parse::<u32>().unwrap());
    result.winning_numbers.extend(parsed_ints);
    
    let my_numbers = split_parts.next().unwrap();
    let parsed_ints = my_numbers.split(" ").filter(|x| *x != "").map(|x| x.parse::<u32>().unwrap());
    result.my_numbers.extend(parsed_ints);
    
    return result;
}

fn parse_cards(card_contents:String) -> u32 {
    let split_lines = card_contents.split("\n");
    let mut cards: Vec<Card> = Vec::new();

    let mut copy_counter: HashMap<u32, u32> = HashMap::new();
    for line in split_lines {
        
        let parsed_card = parse_card(line);
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
        // for each copy push so we can gather length
        for _x in 0..copy_counter[&parsed_card.card_id] {
            cards.push(parsed_card.clone());
        }
    }

    return cards.len() as u32;
}

fn main() {
    let file_path = "./puzzle";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let sum_points = parse_cards(contents);
    dbg!(sum_points);
}