use std::env;
use std::fs;

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
    let extrapolated: Vec<isize> = histories.iter().map(extrapolate).collect();
    let answer1: isize = extrapolated.into_iter().sum();
    println!("Part 1 answer: {answer1}")
}

fn extrapolate(history: &Vec<isize>) -> isize {
    if history.iter().all(|&number| number == 0) {
        return 0;
    }
    let differences: Vec<isize> = history
        .into_iter()
        .zip(history.into_iter().skip(1))
        .map(|(number1, number2)| number2 - number1)
        .collect();
    let value = extrapolate(&differences);
    history.into_iter().last().unwrap() + value
}
