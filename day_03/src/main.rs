use std::{fs, u32};

fn is_part_number_valid(i: isize, j: isize, schematic_rep: [[i8; 140]; 140]) -> bool {
    let locality_map = [
        [i, j-1],
        [i, j+1],
        [i-1, j],
        [i+1, j],
        [i-1, j-1],
        [i-1, j+1],
        [i+1, j-1],
        [i+1, j+1],
    ];
    let mut is_valid_number: bool = false;
    
    for map_pair in locality_map.iter() {
        // bound check x axis
        if map_pair[0] >= schematic_rep.len() as isize || map_pair[0] < 0 {
            continue
        }

        // bound check x axis
        if map_pair[1] >= schematic_rep.len() as isize || map_pair[1] < 0 {
            continue
        }
        let neighbor = schematic_rep[map_pair[0] as usize][map_pair[1] as usize] as u8;
        if !neighbor.is_ascii_digit() && neighbor != 46 {
            is_valid_number = true;
            break
        }
    }

    return is_valid_number;
}

fn parse_schematic(schematic_input:String) -> Vec<u32> {
    let mut schematic_rep = [[0i8; 140]; 140];
    let mut valid_part_ids = Vec::new();
    
    // create the 2d array of the map
    for (i, line) in schematic_input.split("\n").enumerate() {
        for (j, schematic_char) in line.as_bytes().iter().enumerate() {
            schematic_rep[i][j] = *schematic_char as i8;
        }
        println!("{:?}", schematic_rep[i]);
    }

    for (i, _line) in schematic_rep.iter().enumerate() {
        let mut string_num: String = "".to_owned();
        
        let mut j:usize = 0;
        while j < schematic_rep.len() {
            let ascii_char = schematic_rep[i][j] as u8;
            // println!("{}", ascii_char as char);
            if ascii_char.is_ascii_digit() {
                let mut part_id_is_valid = false;

                while j < schematic_rep.len() {
                    let ascii_char = schematic_rep[i][j] as u8;
                    if !ascii_char.is_ascii_digit() {
                        break
                    }
                    
                    string_num.push(ascii_char as char);
                    // compute the current chars validity and
                    // check if current char of the previous digits were valid
                    if !part_id_is_valid {
                        if is_part_number_valid(i as isize, j as isize, schematic_rep) {
                            part_id_is_valid = true;
                        }
                    }

                    j += 1;
                }
                
                // if our parsing led to a valid part
                if part_id_is_valid {
                    println!("pushing: {}", string_num);
                    valid_part_ids.push(string_num.parse::<u32>().unwrap());

                    string_num = "".to_owned();
                }
            
            } else {
                // reset state and increment
                string_num = "".to_owned();
                j += 1;
            }
        }
        println!("------------------");
    }
    return valid_part_ids;
}

fn main() {
    let file_path = "./puzzle";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let valid_part_ids = parse_schematic(contents);
    let sum_game_ids: u32 = valid_part_ids.iter().sum();
    println!("final sum: {}", sum_game_ids);
}