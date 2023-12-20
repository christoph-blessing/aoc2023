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
    let mut instructions = lines.next().unwrap().chars().cycle();
    let nodes: Vec<Node> = lines.skip(1).map(parse_node).collect();
    let mut current = find_node(&nodes, "AAA");
    let mut n_steps = 0;
    while current.label != "ZZZ" {
        let instruction = instructions.next().unwrap();
        let label;
        if instruction == 'L' {
            label = current.left;
        } else {
            label = current.right;
        }
        current = find_node(&nodes, label);
        n_steps = n_steps + 1;
    }
    println!("Part 1 Answer: {n_steps}")
}

fn parse_node(line: &str) -> Node {
    let mut split = line.split(" ");
    let label = split.next().unwrap();
    let mut skip = split.skip(1);
    let left = &skip.next().unwrap()[1..4];
    let right = &skip.next().unwrap()[0..3];
    Node { label, left, right }
}

fn find_node<'a>(nodes: &'a Vec<Node>, label: &str) -> &'a Node<'a> {
    nodes.iter().find(|n| n.label == label).unwrap()
}
