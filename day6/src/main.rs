use std::fs;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let times = parse_numbers(lines.next().unwrap());
    let distances = parse_numbers(lines.next().unwrap());
    let races = times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance });
    let answer1: usize = races.map(|r| compute_margin(&r)).product();

    lines = input.lines();
    let answer2 = compute_margin(&Race {
        time: parse_number(lines.next().unwrap()),
        distance: parse_number(lines.next().unwrap()),
    });
    println!("Part 1 answer: {answer1}");
    println!("Part 2 answer: {answer2}");
}

fn parse(line: &str) -> impl Iterator<Item = &str> {
    line.split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|s| s != &"")
}

fn parse_numbers(line: &str) -> impl Iterator<Item = usize> + '_ {
    parse(line).map(|s| s.parse().unwrap())
}

fn parse_number(line: &str) -> usize {
    parse(line).collect::<Vec<&str>>().join("").parse().unwrap()
}

fn compute_margin(race: &Race) -> usize {
    let mut margin = 0;
    for t_hold in 1..race.time + 1 {
        if (race.time - t_hold) * t_hold <= race.distance {
            continue;
        }
        margin = margin + 1;
    }
    margin
}
