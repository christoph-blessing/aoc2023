use std::env;
use std::fs;
use std::iter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let histories: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();
    let answer1 = answer(&histories, next);
    let answer2 = answer(&histories, previous);
    println!("Part 1 answer: {answer1}");
    println!("Part 2 answer: {answer2}")
}

fn answer(histories: &Vec<Vec<isize>>, direction: fn(&Vec<isize>, isize) -> isize) -> isize {
    let extrapolated: Vec<isize> = histories
        .iter()
        .zip(iter::repeat(direction))
        .map(|(history, f)| extrapolate(history, f))
        .collect();
    extrapolated.into_iter().sum()
}

fn extrapolate(history: &Vec<isize>, direction: fn(&Vec<isize>, isize) -> isize) -> isize {
    if history.iter().all(|&number| number == 0) {
        return 0;
    }
    let differences = history
        .into_iter()
        .zip(history.into_iter().skip(1))
        .map(|(number1, number2)| number2 - number1)
        .collect();
    let value = extrapolate(&differences, direction);
    direction(history, value)
}

fn next(history: &Vec<isize>, value: isize) -> isize {
    *history.into_iter().last().unwrap() + value
}

fn previous(history: &Vec<isize>, value: isize) -> isize {
    *history.into_iter().next().unwrap() - value
}
