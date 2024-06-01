use std::{collections::HashSet, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let points_sum = read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(|line| {
            let (winning_numbers, numbers_we_have) = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split_once('|')
                .map(|(lhs, rhs)| {
                    (
                        lhs.trim()
                            .split(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect::<HashSet<u64>>(),
                        rhs.trim()
                            .split(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect::<HashSet<u64>>(),
                    )
                })
                .unwrap();
            winning_numbers.intersection(&numbers_we_have).count() as u32
        })
        .filter(|matched_numbers| *matched_numbers > 0)
        .fold(0, |res, matched_numbers| {
            res + 2_u64.pow(matched_numbers - 1)
        });
    println!("{points_sum}");
}
