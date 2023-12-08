use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let part1 = solve_part1(input);
    let part2 = solve_part2(input);

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn solve_part1(input: &str) -> usize {
    let mut steps = 0;

    let chunks = input.split_once("\r\n\r\n").unwrap();

    let navigation = chunks.0.trim().as_bytes();
    let nodes: HashMap<&str, (&str, &str)> = parse_nodes(chunks.1);

    let mut node = "AAA";

    while node != "ZZZ" {
        let direction = navigation[steps % navigation.len()];
        let left_right = nodes.get(node).unwrap();

        if direction == b'L' {
            node = left_right.0;
        } else {
            node = left_right.1;
        }

        steps += 1;
    }

    steps
}

fn solve_part2(input: &str) -> usize {
    let chunks = input.split_once("\r\n\r\n").unwrap();

    let navigation = chunks.0.trim().as_bytes();
    let nodes: HashMap<&str, (&str, &str)> = parse_nodes(chunks.1);

    let step_nodes: Vec<&str> = nodes
        .keys()
        .map(|key| *key)
        .filter(|key| key.ends_with('A'))
        .collect();

    let steps_of_nodes: Vec<_> = step_nodes
        .iter()
        .map(|node| traverse(navigation, node, &nodes))
        .collect();

    let max_steps = steps_of_nodes.iter().max().unwrap();

    let mut multiplier = 2;

    // good enough
    let mut steps;
    loop {
        steps = max_steps * multiplier;
        let is_good = steps_of_nodes.iter().all(|num| {
            return steps % num == 0;
        });

        if is_good {
            break;
        }

        multiplier += 1;
    }

    steps
}

fn parse_nodes(lines: &str) -> HashMap<&str, (&str, &str)> {
    lines
        .lines()
        .map(|line| {
            let parts = line.split_once(" = ").unwrap();

            let node_name = parts.0;

            let mut left_right = parts.1.chars();
            left_right.next();
            left_right.next_back();

            let left_right = left_right.as_str();
            let (left, right) = left_right.split_once(", ").unwrap();

            (node_name, (left, right))
        })
        .collect()
}

fn traverse(directions: &[u8], node: &'_ str, nodes: &HashMap<&str, (&'_ str, &'_ str)>) -> usize {
    let mut node = node;
    let mut steps = 0;
    while !node.ends_with('Z') {
        let direction = directions[steps % directions.len()];
        let left_right = nodes.get(node).unwrap();

        if direction == b'L' {
            node = left_right.0;
        } else {
            node = left_right.1;
        }
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn sample1_part1() {
        let input = include_str!("../sample1");

        let result = solve_part1(input);

        assert_eq!(2, result);
    }

    #[test]
    fn sample2_part1() {
        let input = include_str!("../sample2");

        let result = solve_part1(input);

        assert_eq!(6, result);
    }

    #[test]
    fn sample3_part2() {
        let input = include_str!("../sample3");

        let result = solve_part2(input);

        assert_eq!(6, result);
    }
}
