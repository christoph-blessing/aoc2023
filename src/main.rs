use std::fs;

fn main() {
    let contents = fs::read_to_string("1_trebuchet.txt").expect("Could not read file!");
    let lines: Vec<&str> = contents.lines().collect();
    let mut total = 0;
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
        total = total + number;
    }
    println!("{total}")
}
