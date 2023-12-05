use std::{fs, u32};
use std::collections::HashMap;

fn find_star_neighbor(i: isize, j: isize, schematic_rep: [[i8; 140]; 140]) -> (isize, isize) {
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
    
    for map_pair in locality_map.iter() {
        // bound check x axis
        if map_pair[0] >= schematic_rep.len() as isize || map_pair[0] < 0 {
            continue
        }

        // bound check y axis
        if map_pair[1] >= schematic_rep.len() as isize || map_pair[1] < 0 {
            continue
        }
        let neighbor = schematic_rep[map_pair[0] as usize][map_pair[1] as usize] as u8;
        if neighbor == 42 {
            return (map_pair[0], map_pair[1]);
        }
    }

    return (0, 0);
}

fn part2(schematic_input:&str) -> u32 {
    let mut schematic_rep = [[0i8; 140]; 140];
    let mut result_hashmap: HashMap<(u32, u32), Vec::<(u32, u32)>> = HashMap::new();
    
    // create the 2d array of the map
    for (i, line) in schematic_input.split("\n").enumerate() {
        for (j, schematic_char) in line.as_bytes().iter().enumerate() {
            schematic_rep[i][j] = *schematic_char as i8;
        }
    }

    for (i, _line) in schematic_rep.iter().enumerate() {
        let mut string_num: String = "".to_owned();
        
        let mut j:usize = 0;
        while j < schematic_rep.len() {
            let ascii_char = schematic_rep[i][j] as u8;
            // println!("{}", ascii_char as char);
            if ascii_char.is_ascii_digit() {
                let mut is_star_neighbor = false;
                let mut star_neighbor_i = 0;
                let mut star_neighbor_j = 0;

                while j < schematic_rep.len() {
                    let ascii_char = schematic_rep[i][j] as u8;
                    if !ascii_char.is_ascii_digit() {
                        break
                    }
                    
                    string_num.push(ascii_char as char);
                    // compute the current chars validity and
                    // check if current char of the previous digits were valid
                    let (star_i, star_j) = find_star_neighbor(i as isize, j as isize, schematic_rep);
                    if star_i != 0 && star_j != 0 {
                        is_star_neighbor = true;
                        star_neighbor_i = star_i;
                        star_neighbor_j = star_j;

                    }

                    j += 1;
                }
                
                // if we found a star neighbor add to set
                if is_star_neighbor {
                    // println!("star neighbor:{}, i:{}, j:{}", string_num, star_neighbor_i, star_neighbor_j);
                    let combined = i+j;
                    let parsed_int = string_num.parse::<u32>().unwrap();
                    
                    if !result_hashmap.contains_key(&((combined as u32), parsed_int)) {
                        result_hashmap.insert((combined as u32, parsed_int), Vec::new());
                    }
                    result_hashmap.get_mut(&(combined as u32, parsed_int)).unwrap().push((star_neighbor_i as u32, star_neighbor_j as u32));
                    string_num = "".to_owned();
                }
            
            } else {
                // reset state and increment
                string_num = "".to_owned();
                j += 1;
            }
        }
    }
    
    let mut parsed_indexes: Vec<(u32, u32)> = Vec::new();
    // dbg!(&result_hashmap);
    let mut to_sum_vals = Vec::new();
    for (key, value) in result_hashmap.clone() {
        for (key_2, value_2) in result_hashmap.clone() {
            if parsed_indexes.contains(&(key.0, key_2.0)){
                continue;
            }

            if value.len() > 2 || value_2.len() > 2 {
                // println!("too long");
            }
            
            if key.0 != key_2.0 && 
                value[0].0 == value_2[0].0 && 
                value[0].1 == value_2[0].1 {
                    // println!("{} {}, {} {} -> {:?}, {:?}", key.1, key.0, key_2.1, key_2.0, value, value_2);
                    // println!("mult: {} * {}: {}", key.1, key_2.1, key.1 * key_2.1);
                    to_sum_vals.push(key.1 * key_2.1);
                    parsed_indexes.push((key.0, key_2.0));
            }
        }
    }

    return to_sum_vals.iter().sum();
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer/2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result/2, 467835);
    }
}