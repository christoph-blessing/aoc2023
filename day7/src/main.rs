use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut hands: Vec<Hand> = input.lines().map(parse).collect();
    hands.sort_by(compare_hands);
    hands.reverse();
    let answer1 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum::<usize>();
    println!("Part 1 answer: {answer1}")
}

fn parse(line: &str) -> Hand {
    let mut line = line.split(" ");
    let cards = line.next().unwrap().chars().collect();
    let bid = line.next().unwrap().parse().unwrap();
    Hand { cards, bid }
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    let type1 = determine_type(hand1);
    let type2 = determine_type(hand2);
    if type1 > type2 {
        Ordering::Greater
    } else if type1 < type2 {
        Ordering::Less
    } else {
        break_tie(hand1, hand2)
    }
}

fn determine_type(hand: &Hand) -> HandType {
    let mut counts: HashMap<&char, usize> = HashMap::new();
    for card in &hand.cards {
        match counts.get(card) {
            Some(count) => counts.insert(card, count + 1),
            None => counts.insert(card, 1),
        };
    }
    let mut counts: Vec<&usize> = counts.values().collect();
    counts.sort();
    counts.reverse();
    match counts[..] {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn break_tie(hand1: &Hand, hand2: &Hand) -> Ordering {
    for (card1, card2) in hand1.cards.iter().zip(&hand2.cards) {
        let position1 = CARDS.iter().position(|c| c == card1).unwrap();
        let position2 = CARDS.iter().position(|c| c == card2).unwrap();
        if position1 < position2 {
            return Ordering::Less;
        } else if position1 > position2 {
            return Ordering::Greater;
        }
    }
    panic!()
}
