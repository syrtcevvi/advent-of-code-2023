use std::{collections::HashMap, fs::read_to_string, ops::RangeInclusive};

use iter_tools::Itertools;

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut numbers: Vec<((bool, u64), (usize, RangeInclusive<usize>))> = Vec::with_capacity(256);
    let mut symbols: HashMap<(usize, usize), char> = HashMap::with_capacity(128);

    read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            let line = format!("{line}\n");
            let mut current_number: Option<u64> = None;
            for (j, c) in line.char_indices() {
                if c.is_digit(10) {
                    let current_digit = c.to_digit(10).unwrap() as u64;
                    if current_number.is_none() {
                        current_number = Some(current_digit);
                    } else {
                        current_number = current_number.map(|cn| cn * 10 + current_digit)
                    }
                } else {
                    if c != '.' && c != '\n' {
                        symbols.insert((i, j), c);
                    }

                    if let Some(number) = current_number.take() {
                        let digits_quantity = number.to_string().len();
                        numbers.push(((false, number), (i, j - digits_quantity..=j - 1)));
                    }
                }
            }
        });

    let mut part_numbers_sum = 0;

    for ((i, j), c) in symbols {
        for (line, column) in (i - 1..=i + 1).cartesian_product(j - 1..=j + 1) {
            if let Some(((is_used, number), _)) = numbers
                .iter_mut()
                .filter(|((is_used, _), _)| !is_used)
                .find(|(_, (i, js))| line == *i && js.contains(&column))
            {
                *is_used = true;
                part_numbers_sum += *number;
            }
        }
    }

    println!("{part_numbers_sum}");
}
