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
    let starts: Vec<&Node> = nodes
        .iter()
        .filter(|n| n.label.chars().last().unwrap() == 'A')
        .collect();
    let factors = starts
        .into_iter()
        .map(|start| {
            let mut current = start;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .map(move |(index, instruction)| {
                    let label = if instruction == &'L' {
                        current.left
                    } else {
                        current.right
                    };
                    current = find_node(nodes, label);
                    (index, current)
                })
                .find(|(_, node)| node.label.chars().last() == Some('Z'))
                .map(|(index, _)| index + 1)
        })
        .flatten();
    factors
        .reduce(|acc, f| least_common_multiple(acc, f))
        .unwrap()
}

fn find_node<'a>(nodes: &'a Vec<Node>, label: &str) -> &'a Node<'a> {
    nodes.iter().find(|n| n.label == label).unwrap()
}

fn least_common_multiple(number1: usize, number2: usize) -> usize {
    if number1 == 0 || number2 == 0 {
        return 0;
    }
    (number1 * number2) / greatest_common_divisor(number1, number2)
}

fn greatest_common_divisor(mut number1: usize, mut number2: usize) -> usize {
    while number2 != 0 {
        let temp = number2;
        number2 = number1 % number2;
        number1 = temp;
    }
    number1
}
