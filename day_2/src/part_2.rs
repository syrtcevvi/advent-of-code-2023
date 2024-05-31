use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";
fn main() {
    let power_sum = read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .fold(0, |power_sum, line| {
            let mut max_cubes_quantity: HashMap<&str, u64> = HashMap::new();
            // Get rid of word "Game"
            let line = &line[5..];
            let (game_id, subsets) = line
                .split_once(": ")
                .map(|(game_id, line)| (game_id.parse::<u64>().unwrap(), line.split("; ")))
                .unwrap();
            for subset in subsets {
                let cube_infos = subset
                    .split(", ")
                    .map(|info| info.split_once(' ').unwrap())
                    .map(|(quantity, color)| (quantity.parse::<u64>().unwrap(), color));
                for (quantity, color) in cube_infos {
                    let max_cube_quantity = max_cubes_quantity.entry(color).or_default();

                    if quantity > *max_cube_quantity {
                        *max_cube_quantity = quantity;
                    }
                }
            }
            let power = max_cubes_quantity
                .values()
                .product::<u64>();

            power_sum + power
        });
    println!("{power_sum}");
}
