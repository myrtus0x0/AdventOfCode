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

fn part1(src_info: &str) -> u32 {
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

    let mut node_map: HashMap<String, Node> = HashMap::new();

    for map in map_info.clone() {
        let line_node = Node::parse(map);
        node_map.insert(line_node.name.clone(), line_node);
    }

    let mut node = "AAA";
    let mut i = 0;
    loop {
        let c_node = node_map.get(node).unwrap();
        if c_node.name == "ZZZ" {
            break;
        }
        match move_set.chars().nth(i % move_set.len()).unwrap() {
            'L' => node = c_node.right.as_str(),
            'R' => node = c_node.left.as_str(),
            _ => (),
        };

        i += 1;
    }

    return i.try_into().unwrap();
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
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2);
    }
}
