

fn part1(src_info:&str) -> u32 {
    let times = src_info.split("\n").nth(0).unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = src_info.split("\n").nth(1).unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let num_races = distances.clone().len();
    let mut race_wins: Vec<u32> = Vec::new();
    for i in 0..num_races {
        let mut num_wins = 0;
        for j in 1..times[i] {
            let total_distance = j * (times[i] - j);
            if total_distance > distances[i] {
                num_wins += 1;
            }
        }
        race_wins.push(num_wins);
    }
    
    return race_wins.iter().product();
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
        let result = part1("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 288);
    }
}