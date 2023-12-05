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

fn part1(coordinate_info:&str) -> u32 {
    let mut converted_ints = Vec::new();
    let split_lines = coordinate_info.split("\n");
    
    for line in split_lines {
        let (part1, part2) = parse_int_from_line(line);
        
        let joined = (part1*10)+part2;        
        converted_ints.push(joined);
    }

    return converted_ints.iter().sum();
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}