

fn part2(src_info:&str) -> u64 {
    let time = src_info.split("\n").nth(0).unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = src_info.split("\n").nth(1).unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    
    let mut num_wins = 0;
    for j in 1..time {
        let total_distance = j * (time - j);
        if total_distance > distance {
            num_wins += 1;
        }
    }
    
    return num_wins;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2("Time:      71530
Distance:  940200");
        assert_eq!(result, 71503);
    }
}