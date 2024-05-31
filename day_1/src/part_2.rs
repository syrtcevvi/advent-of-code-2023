use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const DIGITS: &str = "0123456789";
const LETTER_DIGITS: [(u8, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn main() {
    let calibration_sum: u64 = read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            // Convert all letter-digits to simply digits
            let mut digits: Vec<u8> = vec![];
            let mut cursor = 0;
            let last_i = line.len() - 1;

            'outer: while cursor <= last_i {
                let tail = &line[last_i - cursor..];
                for (digit, letter_digit) in LETTER_DIGITS {
                    if tail.starts_with(letter_digit) {
                        digits.insert(0, digit);
                        cursor += 1;
                        continue 'outer;
                    }
                }
                if let Some(digit) = tail.chars().next().and_then(|c| c.to_digit(10)) {
                    digits.insert(0, digit as u8);
                }
                cursor += 1;
            }

            let current = format!("{}{}", digits.first().unwrap(), digits.last().unwrap(),)
                .parse::<u64>()
                .unwrap();
            acc + current
        });
    println!("{calibration_sum}");
}
