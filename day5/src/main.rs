use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let seeds = parse_seeds(&input);
    let maps = parse_maps(&input);
    let answer_1 = part_1(&seeds, &maps);
    let answer_2 = part_2(&seeds, &maps);
    println!("Part 1 answer: {answer_1}");
    println!("Part 2 answer: {answer_2}")
}

#[derive(Debug)]
struct Map {
    functions: Vec<Function>,
}

impl Map {
    fn map(&self, number: isize) -> isize {
        for range in &self.functions {
            if !(range.src <= number && number < range.src + range.length) {
                continue;
            }
            return range.dest + number - range.src;
        }
        return number;
    }
    fn map_range(&self, range: Range) -> Vec<Range> {
        let mut to_be_processed = vec![range];
        let mut processed = Vec::new();
        while to_be_processed.len() > 0 {
            let range = to_be_processed.pop().unwrap();
            let mut has_overlap = false;
            for function in &self.functions {
                let overlap_start = range.start.max(function.src);
                let overlap_end = (range.start + range.length).min(function.src + function.length);
                if overlap_end <= overlap_start {
                    continue;
                }
                let overlap = Range {
                    start: function.dest + overlap_start - function.src,
                    length: overlap_end - overlap_start,
                };
                processed.push(overlap);
                let before = Range {
                    start: range.start,
                    length: overlap_start - range.start,
                };
                if before.length > 0 {
                    to_be_processed.push(before);
                }
                let after = Range {
                    start: overlap_end,
                    length: range.start + range.length - overlap_end,
                };
                if after.length > 0 {
                    to_be_processed.push(after)
                }
                has_overlap = true;
                break;
            }
            if !has_overlap {
                processed.push(range);
            }
        }
        processed
    }
}

#[derive(Debug)]
struct Function {
    src: isize,
    dest: isize,
    length: isize,
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: isize,
    length: isize,
}

fn parse_seeds(input: &str) -> Vec<isize> {
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
            maps.push(Map { functions: ranges });
            ranges = Vec::new();
            continue;
        }
        let mut iter = line.split(" ").map(|n| n.parse().unwrap());
        ranges.push(Function {
            dest: iter.next().unwrap(),
            src: iter.next().unwrap(),
            length: iter.next().unwrap(),
        });
    }
    maps
}

fn find_location<I>(seeds: I, maps: &Vec<Map>) -> isize
where
    I: Iterator<Item = isize>,
{
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

fn part_1(seeds: &Vec<isize>, maps: &Vec<Map>) -> isize {
    find_location(seeds.to_owned().into_iter(), maps)
}

fn part_2(raw_seed_ranges: &Vec<isize>, maps: &Vec<Map>) -> isize {
    let mut seed_ranges = Vec::new();
    let mut seed_iter = raw_seed_ranges.iter();
    loop {
        let start = match seed_iter.next() {
            Some(start) => start,
            None => break,
        };
        let length = seed_iter.next().unwrap();
        seed_ranges.push(Range {
            start: *start,
            length: *length,
        });
    }
    let mut to_be_processed = seed_ranges;
    let mut processed = Vec::new();
    for map in maps {
        while to_be_processed.len() > 0 {
            processed.extend(map.map_range(to_be_processed.pop().unwrap()))
        }
        to_be_processed = processed.clone();
        processed.clear();
    }
    to_be_processed.iter().map(|r| r.start).min().unwrap()
}
