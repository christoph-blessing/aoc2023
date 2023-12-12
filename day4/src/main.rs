use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    for mut card in contents.lines() {
        card = card.strip_prefix("Card ").unwrap();
        let id: u32;
        (id, card) = match card.split(":").collect::<Vec<&str>>()[..] {
            [id, card] => (id.trim().parse().unwrap(), card),
            _ => panic!(),
        };
        let (winning, numbers) = match card.split("|").collect::<Vec<&str>>()[..] {
            [winning, numbers] => (parse_numbers(winning), parse_numbers(numbers)),
            _ => panic!(),
        };
        let mut card_score = 0;
        for number in numbers {
            if !winning.contains(&number) {
                continue;
            }
            if card_score == 0 {
                card_score += 1;
            } else {
                card_score *= 2;
            }
        }
        total = total + card_score
    }
    println!("{total}");
}

fn parse_numbers(input: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    for raw_number in input.trim().split(" ") {
        if raw_number == "" {
            continue;
        }
        let number: u32 = raw_number.parse().unwrap();
        numbers.push(number);
    }
    numbers
}
