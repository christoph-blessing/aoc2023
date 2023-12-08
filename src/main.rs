use std::fs;

fn parse_numbers(input: &str) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<u32> = Vec::new();
    for line in &lines {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;
        for char in line.chars() {
            if !char.is_numeric() {
                continue;
            }
            if first_digit.is_none() {
                first_digit = Some(char);
            }
            last_digit = Some(char);
        }
        let first_digit = first_digit.unwrap();
        let last_digit = last_digit.unwrap();
        let number: u32 = format!("{first_digit}{last_digit}").parse().unwrap();
        numbers.push(number);
        println!("{line}, {number}")
    }
    numbers
}

fn sum_numbers(numbers: Vec<u32>) -> u32 {
    let mut total = 0;
    for number in numbers {
        total = total + number;
    }
    total
}

const NUMBERS_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUMBERS_INT: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn convert_numbers(input: &str) -> String {
    let mut converted_lines: Vec<String> = Vec::new();
    for line in input.lines() {
        let mut converted_line = line.to_owned();

        let mut min_match = None;
        for (pattern_index, number_str) in NUMBERS_STR.iter().enumerate() {
            for (match_index, _) in converted_line.match_indices(number_str) {
                match min_match {
                    Some((min_index, _)) => {
                        if match_index < min_index {
                            min_match = Some((match_index, pattern_index));
                        }
                    }
                    None => min_match = Some((match_index, pattern_index)),
                };
            }
        }
        match min_match {
            Some((min_index, pattern_index)) => {
                let pattern = NUMBERS_STR[pattern_index];
                converted_line.replace_range(
                    min_index..min_index + pattern.len(),
                    NUMBERS_INT[pattern_index],
                )
            }
            None => (),
        }

        let mut max_match = None;
        for (pattern_index, number_str) in NUMBERS_STR.iter().enumerate() {
            for (match_index, _) in converted_line.match_indices(number_str) {
                match max_match {
                    Some((max_index, _)) => {
                        if match_index > max_index {
                            max_match = Some((match_index, pattern_index))
                        }
                    }
                    None => max_match = Some((match_index, pattern_index)),
                };
            }
        }
        match max_match {
            Some((max_index, pattern_index)) => {
                let pattern = NUMBERS_STR[pattern_index];
                converted_line.replace_range(
                    max_index..max_index + pattern.len(),
                    NUMBERS_INT[pattern_index],
                )
            }
            None => (),
        }
        // println!("{line}, {converted_line}");
        converted_lines.push(converted_line);
    }
    converted_lines.join("\n")
}

fn parse_numbers2(input: &str) {
    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;
        let mut possible_matches = Vec::from(NUMBERS_STR);
        let mut match_index = 0;
        for char in line.chars() {
            if char.is_numeric() {
                if first_digit.is_none() {
                    first_digit = Some(char)
                }
                last_digit = Some(char);
                continue;
            }
            for possible_match in possible_matches {
                if match_index > possible_match.len() {}
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("1_trebuchet.txt").expect("Could not read file!");
    let total = sum_numbers(parse_numbers(&contents));
    println!("{total}");
    let converted = convert_numbers(&contents);
    let converted_total = sum_numbers(parse_numbers(&converted));
    println!("{converted_total}")
}
