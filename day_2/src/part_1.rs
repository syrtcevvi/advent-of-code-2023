use std::{collections::HashMap, fs::read_to_string, sync::OnceLock};

const INPUT_FILE: &str = "input.txt";

fn cube_constraints() -> &'static HashMap<&'static str, u64> {
    static CUBE_CONSTRAINTS: OnceLock<HashMap<&'static str, u64>> = OnceLock::new();
    CUBE_CONSTRAINTS.get_or_init(|| HashMap::from([("red", 12), ("green", 13), ("blue", 14)]))
}

fn main() {
    let possible_game_ids_sum =
        read_to_string(INPUT_FILE)
            .unwrap()
            .lines()
            .fold(0, |possible_game_ids_sum, line| {
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
                        // Limit is exceeded -> game is not possible
                        if quantity > cube_constraints()[color] {
                            return possible_game_ids_sum;
                        }
                    }
                }

                possible_game_ids_sum + game_id
            });
    println!("{possible_game_ids_sum}");
}
