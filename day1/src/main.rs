use std::fs;

const PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn parse_numbers(text: &str) -> u32 {
    fn parse_line(line: &str) -> u32 {
        let mut patterns = Vec::new();
        for pattern in PATTERNS {
            let pattern = pattern.to_owned();
            patterns.push(pattern)
        }

        let mut reversed_patterns = Vec::new();
        for pattern in PATTERNS {
            let reversed_pattern = pattern.chars().rev().collect::<String>();
            reversed_patterns.push(reversed_pattern)
        }

        let first_digit = match find(line, patterns) {
            Some((_, pattern)) => pattern,
            None => panic!(),
        };
        let reversed_line = line.chars().rev().collect::<String>();
        let last_digit = match find(&reversed_line, reversed_patterns) {
            Some((_, pattern)) => pattern.chars().rev().collect::<String>(),
            None => panic!(),
        };
        format!(
            "{}{}",
            convert_digit(&first_digit),
            convert_digit(&last_digit)
        )
        .parse::<u32>()
        .unwrap()
    }
    fn find(line: &str, patterns: Vec<String>) -> Option<(usize, String)> {
        let mut first_match = None;
        for pattern in patterns {
            match line.find(&pattern) {
                Some(current_index) => match first_match {
                    Some((first_index, _)) => {
                        if current_index < first_index {
                            first_match = Some((current_index, pattern.to_owned()));
                        }
                    }
                    None => first_match = Some((current_index, pattern.to_owned())),
                },
                None => (),
            }
        }
        first_match
    }
    fn convert_digit(digit: &str) -> &str {
        let mut index = PATTERNS.iter().position(|d| d.to_owned() == digit).unwrap();
        if index > 8 {
            index = index - 9;
        }
        match PATTERNS.get(index) {
            Some(index) => index,
            None => panic!(),
        }
    }
    let mut total = 0;
    for line in text.lines() {
        total = total + parse_line(line);
    }
    total
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file!");
    let total = parse_numbers(&contents);
    println!("{total}")
}
