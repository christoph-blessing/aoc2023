use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let seeds = parse_seeds(&input);
    let maps = parse_maps(&input);
    let lowest_location = part_1(&seeds, &maps);
    println!("Lowest location number: {lowest_location}")
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, number: usize) -> usize {
        match self.ranges.iter().find_map(|r| r.map(number)) {
            Some(mapped) => mapped,
            None => number,
        }
    }
}

#[derive(Debug)]
struct Range {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl Range {
    fn can_map(&self, number: usize) -> bool {
        self.source_start <= number && number < self.source_start + self.length
    }
    fn map(&self, number: usize) -> Option<usize> {
        if !self.can_map(number) {
            return None;
        }
        let offset = number - self.source_start;
        Some(self.destination_start + offset)
    }
}

fn parse_seeds(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_maps(input: &str) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut ranges = Vec::new();
    for line in input.lines().skip(2) {
        if line.ends_with(":") {
            continue;
        }
        if line == "" {
            maps.push(Map { ranges });
            ranges = Vec::new();
            continue;
        }
        let mut iter = line.split(" ").map(|n| n.parse().unwrap());
        ranges.push(Range {
            destination_start: iter.next().unwrap(),
            source_start: iter.next().unwrap(),
            length: iter.next().unwrap(),
        });
    }
    maps
}

fn part_1(seeds: &Vec<usize>, maps: &Vec<Map>) -> usize {
    let mut lowest_location = None;
    for current_seed in seeds {
        let mut number = current_seed.clone();
        for map in maps {
            number = map.map(number);
        }
        match lowest_location {
            Some(location) => {
                if number < location {
                    lowest_location = Some(number);
                }
            }
            None => lowest_location = Some(number),
        }
    }
    lowest_location.unwrap()
}
