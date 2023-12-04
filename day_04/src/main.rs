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

fn parse_cards(card_contents:String) -> u32 {
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
        println!("adding card points: {}", to_add);
        total_sum += to_add;
    }

    return total_sum;
}

fn main() {
    let file_path = "./puzzle";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let sum_points = parse_cards(contents);
    dbg!(sum_points);
}