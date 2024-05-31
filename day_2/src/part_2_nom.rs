use std::{collections::HashMap, fs::read_to_string};

use nom::{bytes::complete::tag, character::complete::{alpha1, digit1, space1}, combinator::map_res, multi::separated_list0, sequence::{separated_pair, tuple}, IResult};

const INPUT_FILE: &str = "input.txt";

fn parse_game_header(input: &str) -> IResult<&str, &str> {
    let (remaining, _) = tuple((
        tag("Game"),
        space1,
        digit1,
        tag(":"),
        space1
    ))(input)?;
    Ok((remaining, ""))
}

fn parse_cube(input: &str) -> IResult<&str, (u8, &str)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<u8>()),
        space1,
        alpha1
    )(input)
}

fn parse_cubes_subset(input: &str) -> IResult<&str, Vec<(u8, &str)>> {
    separated_list0(tag(", "), parse_cube)(input)
}


fn parse_cubes_subsets(input: &str) -> IResult<&str, Vec<Vec<(u8, &str)>>> {
    let (input, _) = parse_game_header(input)?;
    separated_list0(tag("; "), parse_cubes_subset)(input)
}

fn main() {
    let power_sum = read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .fold(0, |power_sum, line| {
            let mut max_cubes_quantity: HashMap<&str, u64> = HashMap::new();
            
            let cubes_subsets = parse_cubes_subsets(line).unwrap().1;
            
            for cube_subset in cubes_subsets {
                for (quantity, color) in cube_subset {
                    let max_cube_quantity = max_cubes_quantity.entry(color).or_default();

                    if quantity as u64 > *max_cube_quantity {
                        *max_cube_quantity = quantity as u64;
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
