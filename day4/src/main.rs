use std::collections::VecDeque;
use std::fs;

struct Card {
    id: usize,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let cards = parse_cards(&contents);
    let points_total = part_1(&cards);
    let cards_total = part_2(&cards);
    println!("Total points: {points_total}");
    println!("Total cards: {cards_total}");
}

fn part_1(cards: &Vec<Card>) -> u32 {
    let mut total = 0;
    for card in cards {
        let mut card_score = 0;
        for number in &card.numbers {
            if !card.winning.contains(&number) {
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
    total
}

fn part_2(cards: &Vec<Card>) -> u32 {
    fn get_copies<'a>(cards: &'a Vec<Card>, card: &Card) -> Vec<&'a Card> {
        let mut winning_total = 0;
        for number in &card.numbers {
            if !card.winning.contains(&number) {
                continue;
            }
            winning_total = winning_total + 1;
        }
        let mut copies = Vec::new();
        for offset in 1..winning_total + 1 {
            let copy_id = card.id + offset;
            let copy_card = cards.iter().find(|&card| card.id == copy_id).unwrap();
            copies.push(copy_card);
        }
        copies
    }
    let mut copies = Vec::new();
    for card in cards {
        copies.push(get_copies(cards, card))
    }
    let mut cards_total = 0;
    let mut queue = VecDeque::from_iter(cards.iter());
    while queue.len() > 0 {
        cards_total = cards_total + 1;
        let card = queue.pop_front().unwrap();
        for copy in &copies[card.id - 1] {
            queue.push_back(copy);
        }
    }
    cards_total
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for mut card in input.lines() {
        card = card.strip_prefix("Card ").unwrap();
        let id: usize;
        (id, card) = match card.split(":").collect::<Vec<&str>>()[..] {
            [id, card] => (id.trim().parse().unwrap(), card),
            _ => panic!(),
        };
        let (winning, numbers) = match card.split("|").collect::<Vec<&str>>()[..] {
            [winning, numbers] => (parse_numbers(winning), parse_numbers(numbers)),
            _ => panic!(),
        };
        cards.push(Card {
            id,
            winning,
            numbers,
        });
    }
    cards
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
