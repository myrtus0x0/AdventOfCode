use std::fs;


fn convert_word_to_int(token:&str) -> u32 {
    match token {
        "one" => return 1,
        "two" => return 2,
        "three" => return 3,
        "four" => return 4,
        "five" => return 5,
        "six" => return 6,
        "seven" => return 7,
        "eight" => return 8,
        "nine" => return 9,
        _ => return 1337
    }
}

fn backward_parse(line:&str, number_tokens:[&str; 9]) -> u32 {
    let mut res: u32 = 0;
    
    let line_size = line.len();
    let mut i: usize = 1;
    while i <= line_size {
        let index = line_size-i;
        let c = line.chars().nth(index).unwrap();
        // case to check if its a digit
        if c.is_numeric() {
            res = c.to_string().parse::<u32>().unwrap();
            break;
        } else {
            // iterate over all the tokens and see if our index with the 
            // substring is a match, if so return the int val and break out 
            // of parsing
            for token in number_tokens {
                if index+token.len() > line_size {
                    continue;
                }
                
                if line[index..index+token.len()].contains(token) {
                    println!("backward match found: {} - {}", line, token);
                    res = convert_word_to_int(token);
                    break
                }
            }
        }

        if res != 0 {
            break
        }

        i += 1;
    }

    return res;
}

fn forward_parse(line:&str, number_tokens:[&str; 9]) -> u32 {
    let mut res: u32 = 0;
    
    let line_size = line.len();
    let mut i: usize = 0;
    while i < line_size {
        let c = line.chars().nth(i).unwrap();
        // case to check if its a digit
        if c.is_numeric() {
            res = c.to_string().parse::<u32>().unwrap();
            break;
        } else {
            // iterate over all the tokens and see if our index with the 
            // substring is a match, if so return the int val and break out 
            // of parsing
            for token in number_tokens {
                if i+token.len() > line_size {
                    continue;
                }
                // println!("{}", line[i..i+token.len()]);
                if line[i..i+token.len()].contains(token) {
                    println!("forward match found: {} - {}", line, token);
                    res = convert_word_to_int(token);
                    break
                }
            }
        }

        if res != 0 {
            break
        }

        i += 1;
    }

    return res;
}

fn parse_int_from_line(line:&str) -> (u32, u32) {
    let number_tokens = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    return (forward_parse(line, number_tokens), backward_parse(line, number_tokens));
}

fn itemize_lines(coordinate_info:String) -> Vec<u32> {
    let mut converted_ints = Vec::new();
    let split_lines = coordinate_info.split("\n");
    
    for line in split_lines {
        let (part1, part2) = parse_int_from_line(line);
        
        let joined = (part1*10)+part2;
        println!("converted: {}", joined);
        
        converted_ints.push(joined);
        println!("line:{} - {}{}",line, part1, part2);
    }

    return converted_ints;
}

fn main() {
    let file_path = "./input_coordinates";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let converted_lines = itemize_lines(contents);
    
    let mut sum = 0;
    for converted_int in converted_lines {
        println!("{} += {}", sum, converted_int);
        sum += converted_int;
    }

    println!("final sum: {}", sum);
}