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
            let mut last_elements = Vec::with_capacity(4);
            while history_line.iter().filter(|&&item| item != 0).count() != 0 {
                last_elements.push(history_line.last().cloned().unwrap());
                let differences: Vec<i64> = history_line.windows(2).map(|w| w[1] - w[0]).collect();
                history_line = differences;
            }
            last_elements.into_iter().sum::<i64>()
        })
        .sum();

    println!("{extrapolated_numbers_sum}");

    Ok(())
}
