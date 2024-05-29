use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const DIGITS: &str = "0123456789";

fn main() {
    let calibration_sum: u64 = read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let digits = line.chars().filter(|&c| DIGITS.contains(c));
            let current = format!(
                "{}{}",
                digits.clone().next().unwrap(),
                digits.last().unwrap()
            )
            .parse::<u64>()
            .unwrap();
            acc + current
        });
    println!("{calibration_sum}");
}
