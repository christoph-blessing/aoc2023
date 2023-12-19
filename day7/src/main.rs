use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
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
const CARDS_JOKER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut hands: Vec<Hand> = input.lines().map(parse).collect();
    hands.sort_by(create_comparer(&CARDS.to_vec(), determine_type));
    hands.reverse();
    let answer1 = compute_winnings(&hands);
    hands.sort_by(create_comparer(&CARDS_JOKER.to_vec(), determine_type_joker));
    hands.reverse();
    let answer2 = compute_winnings(&hands);
    println!("Part 1 answer: {answer1}");
    println!("Part 2 answer: {answer2}");
}

fn parse(line: &str) -> Hand {
    let mut line = line.split(" ");
    let cards = line.next().unwrap().chars().collect();
    let bid = line.next().unwrap().parse().unwrap();
    Hand { cards, bid }
}

fn create_comparer<'a>(
    ranks: &'a Vec<char>,
    type_determiner: impl Fn(&Hand) -> HandType + 'a,
) -> impl Fn(&Hand, &Hand) -> Ordering + '_ {
    let compare = move |hand1: &Hand, hand2: &Hand| {
        let type1 = type_determiner(hand1);
        let type2 = type_determiner(hand2);
        match type1.partial_cmp(&type2).unwrap() {
            Ordering::Equal => break_tie(hand1, hand2, ranks),
            order => order,
        }
    };
    compare
}

fn determine_type_joker(hand: &Hand) -> HandType {
    determine_type(&replace_jokers(hand))
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

fn break_tie(hand1: &Hand, hand2: &Hand, ranks: &Vec<char>) -> Ordering {
    for (card1, card2) in hand1.cards.iter().zip(&hand2.cards) {
        let position1 = ranks.iter().position(|c| c == card1).unwrap();
        let position2 = ranks.iter().position(|c| c == card2).unwrap();
        if position1 < position2 {
            return Ordering::Less;
        } else if position1 > position2 {
            return Ordering::Greater;
        }
    }
    panic!()
}

fn replace_jokers(hand: &Hand) -> Hand {
    let mut counts: HashMap<&char, usize> = HashMap::new();
    for card in &hand.cards {
        match counts.get(card) {
            Some(count) => counts.insert(card, count + 1),
            None => counts.insert(card, 1),
        };
    }
    if !counts.contains_key(&'J') {
        return hand.clone();
    }
    let n_jokers = counts.get(&'J').unwrap();
    if n_jokers == &hand.cards.len() {
        return hand.clone();
    }
    let mut counts: Vec<(&char, usize)> = counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| *count);
    counts.reverse();
    let most_common = counts
        .into_iter()
        .map(|(c, _)| c)
        .filter(|&c| c != &'J')
        .next()
        .unwrap();
    let replaced: Vec<char> = hand
        .cards
        .iter()
        .map(|c| {
            if c == &'J' {
                most_common.clone()
            } else {
                c.clone()
            }
        })
        .collect();
    Hand {
        cards: replaced,
        bid: hand.bid,
    }
}

fn compute_winnings(hands: &Vec<Hand>) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum::<usize>()
}
