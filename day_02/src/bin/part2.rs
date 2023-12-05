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
        println!("pull:\"{}\"", pull);
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

fn part2(game_records:&str) -> u32 {
    let mut power_sets = Vec::new();
    let split_lines = game_records.split("\n");
    
    for line in split_lines {
        let game_instance = parse_game(line);
        power_sets.push(
            game_instance.red_cubes.iter().max().unwrap() * 
            game_instance.green_cubes.iter().max().unwrap() * 
            game_instance.blue_cubes.iter().max().unwrap() 
        )
    }

    return power_sets.iter().sum();
}

fn main() {
    let input = include_str!("../puzzle");
    let sum_ids = part2(input);
    dbg!(sum_ids);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}