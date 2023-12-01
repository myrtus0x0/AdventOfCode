use std::fs;

fn parse_int_from_line(line:&str) -> (u32, u32) {
    let mut int_first: u32 = 0;
    let mut int_second: u32 = 0;
    
    for c in line.chars() { 
        if c.is_numeric() {
            int_first = c.to_string().parse::<u32>().unwrap();
            break;
        }
    }

    for c in line.chars().rev() { 
        if c.is_numeric() {
            int_second = c.to_string().parse::<u32>().unwrap();
            break;
        }
    }

    return (int_first, int_second);
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