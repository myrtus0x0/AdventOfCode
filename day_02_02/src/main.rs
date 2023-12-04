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

fn parse_game_info(game_records:String) -> Vec<u32> {
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

    return power_sets;
}

fn main() {
    let file_path = "./puzzle";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let valid_game_ids = parse_game_info(contents);
    let sum_game_ids: u32 = valid_game_ids.iter().sum();
    println!("final sum: {}", sum_game_ids);
}