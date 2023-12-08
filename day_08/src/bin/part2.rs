use regex::Regex;
use std::collections::HashMap;

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(x: &str) -> Node {
        let matches = Regex::new(r"^(.{3}) = \((.{3}), (.{3})\)$")
            .unwrap()
            .captures(x)
            .unwrap();

        let name = matches.get(1).unwrap().as_str().to_owned();
        let left = matches.get(2).unwrap().as_str().to_owned();
        let right = matches.get(3).unwrap().as_str().to_owned();

        return Node { name, left, right };
    }
}

fn lcm(steps_to_end: Vec<u64>) -> u64 {
    return steps_to_end
        .into_iter()
        .reduce(|a, b| a * b / gcd::binary_u64(a, b))
        .unwrap();
}

fn part2(src_info: &str) -> u64 {
    let move_set = src_info
        .split("\n\n")
        .into_iter()
        .nth(0)
        .unwrap()
        .to_string();
    let map_info = src_info
        .split("\n\n")
        .into_iter()
        .nth(1)
        .unwrap()
        .split("\n");

    let mut node_map = HashMap::new();
    for map in map_info.clone() {
        let line_node = Node::parse(map);
        node_map.insert(line_node.name.clone(), line_node);
    }

    let starts: Vec<&String> = node_map.keys().filter(|x| x.ends_with("A")).collect();
    let mut iterations: Vec<u64> = Vec::new();

    for start in starts {
        let mut i: u64 = 0;
        let mut curr_loc = start;
        for current_dir in move_set.chars().cycle() {
            let location = node_map.get(curr_loc).unwrap();
            i += 1;
            match current_dir {
                'L' => curr_loc = &location.left,
                'R' => curr_loc = &location.right,
                _ => (),
            }

            // if we reach an end, we break and push how many steps it took
            if curr_loc.ends_with("Z") {
                break;
            }
        }
        iterations.push(i);
    }

    dbg!(iterations.clone());
    return lcm(iterations);
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
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}
