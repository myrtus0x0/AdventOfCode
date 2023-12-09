fn calc_history(nums: Vec<i32>) -> i32 {
    let mut next_layer = Vec::new();
    if nums.len() == 0 {
        return 0;
    }

    for i in 0..nums.len() - 1 {
        next_layer.push(nums[i + 1] - nums[i])
    }

    return nums[nums.len() - 1] + calc_history(next_layer);
}

fn part2(src_info: &str) -> i32 {
    let mut sum: i32 = 0;
    for row in src_info.split("\n") {
        let history: Vec<i32> = row.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        sum += calc_history(history.into_iter().rev().collect())
    }

    return sum;
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
        let result = part2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 2);
    }
}
