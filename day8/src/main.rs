use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node<'a> {
    label: &'a str,
    left: &'a str,
    right: &'a str,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let nodes: Vec<Node> = lines.skip(1).map(parse_node).collect();
    let answer1 = part1(&nodes, &instructions);
    println!("Part 1 Answer: {answer1}");
    let answer2 = part2(&nodes, &instructions);
    println!("Part 2 Answer: {answer2}");
}

fn parse_node(line: &str) -> Node {
    let mut split = line.split(" ");
    let label = split.next().unwrap();
    let mut skip = split.skip(1);
    let left = &skip.next().unwrap()[1..4];
    let right = &skip.next().unwrap()[0..3];
    Node { label, left, right }
}

fn part1(nodes: &Vec<Node>, instructions: &Vec<char>) -> usize {
    let mut current = find_node(&nodes, "AAA");
    let mut n_steps = 0;
    let mut instructions_iter = instructions.iter().cycle();
    while current.label != "ZZZ" {
        let instruction = instructions_iter.next().unwrap();
        let label;
        if instruction == &'L' {
            label = current.left;
        } else {
            label = current.right;
        }
        current = find_node(&nodes, label);
        n_steps = n_steps + 1;
    }
    n_steps
}

fn part2(nodes: &Vec<Node>, instructions: &Vec<char>) -> usize {
    let left: HashMap<&str, &str> = HashMap::from_iter(nodes.iter().map(|n| (n.label, n.left)));
    let right: HashMap<&str, &str> = HashMap::from_iter(nodes.iter().map(|n| (n.label, n.right)));
    let mut current: Vec<&str> = nodes
        .iter()
        .filter(|n| n.label.chars().last().unwrap() == 'A')
        .map(|n| n.label)
        .collect();
    let mut n_steps = 0;
    let mut instructions_iter = instructions.iter().cycle();
    while current.iter().any(|l| l.chars().last().unwrap() != 'Z') {
        println!("{:?}", n_steps);
        let instruction = instructions_iter.next().unwrap();
        current = current
            .iter()
            .map(|old| {
                let new;
                if instruction == &'L' {
                    new = *left.get(old).unwrap()
                } else {
                    new = *right.get(old).unwrap()
                }
                new
            })
            .collect();
        n_steps = n_steps + 1;
    }
    n_steps
}

fn find_node<'a>(nodes: &'a Vec<Node>, label: &str) -> &'a Node<'a> {
    nodes.iter().find(|n| n.label == label).unwrap()
}
