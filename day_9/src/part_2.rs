use std::error::Error;

fn parse_history_line(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

fn parse_history_lines(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| parse_history_line(line)).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let history_lines = parse_history_lines(include_str!("../input.txt"));

    let extrapolated_numbers_sum: i64 = history_lines
        .into_iter()
        .map(|mut history_line| {
            let mut first_elements = Vec::with_capacity(4);
            while history_line.iter().filter(|&&item| item != 0).count() != 0 {
                first_elements.push(history_line.first().cloned().unwrap());
                let differences: Vec<i64> = history_line.windows(2).map(|w| w[1] - w[0]).collect();
                history_line = differences;
            }
            first_elements.reverse();
            first_elements.into_iter().fold(0_i64, |acc, i| i - acc)
        })
        .sum();

    println!("{extrapolated_numbers_sum}");

    Ok(())
}
