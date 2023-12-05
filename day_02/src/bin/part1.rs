use std::fs;

struct GameResult {
    game_id: u32,
    red_cubes: Vec<u32>,
    blue_cubes: Vec<u32>,
    green_cubes: Vec<u32>,
}

fn parse_game(line:&str) -> GameResult {
    let mut parts = line.split(":");
    let game_id = parts.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();

    let mut result = GameResult{
        game_id: game_id,
        red_cubes: Vec::new(),
        blue_cubes: Vec::new(),
        green_cubes: Vec::new()
    };

    for pull in parts.next().unwrap().split(";") {
        for individual_cube in pull[1..].split(", ") {
            let num_cubes = individual_cube.split(" ").next().unwrap().parse::<u32>().unwrap();
            
            if individual_cube.contains("blue") {
                result.blue_cubes.push(num_cubes);
            } else if individual_cube.contains("green") {
                result.green_cubes.push(num_cubes);
            } else {
                result.red_cubes.push(num_cubes);
            }
        }
    }

    return result;
}

fn part1(game_records:&str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    let mut valid_game_ids = Vec::new();
    let split_lines = game_records.split("\n");
    
    for line in split_lines {
        let game_instance = parse_game(line);
        if game_instance.red_cubes.iter().any(|pull| *pull > max_red) ||
            game_instance.green_cubes.iter().any(|pull| *pull > max_green) ||
            game_instance.blue_cubes.iter().any(|pull| *pull > max_blue) {
        } else {
            valid_game_ids.push(game_instance.game_id);
        }        
    }

    return valid_game_ids.iter().sum();
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input, 12, 13, 14);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 12, 13, 14);
        assert_eq!(result, 8);
    }
}