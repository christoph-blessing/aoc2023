use std::fs;

const COLORS: [&str; 3] = ["red", "green", "blue"];

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Could not read file!");
    let mut sum = 0;
    for line in lines.lines() {
        let mut min_counts = vec![0, 0, 0];
        let draws = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        for draw in draws {
            let balls = draw.split(",");
            for ball in balls {
                let count = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap();
                let color = ball.trim().split(" ").collect::<Vec<&str>>()[1];
                let index = COLORS.iter().position(|x| x == &color).unwrap();
                if min_counts[index] < count {
                    min_counts[index] = count;
                }
            }
        }
        let power = min_counts[0] * min_counts[1] * min_counts[2];
        sum = sum + power;
    }
    println!("{sum}")
}
