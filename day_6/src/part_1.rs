use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

struct Race {
    time_ms: u64,
    distance_mm: u64,
}

impl From<(u64, u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
        Self {
            time_ms: value.0,
            distance_mm: value.1,
        }
    }
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tuple((tag("Time:"), space1))(input)?;

    separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>()))(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tuple((tag("Distance:"), space1))(input)?;

    separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>()))(input)
}

fn parse_races(input: &str) -> Vec<Race> {
    let (_, (times, distances)) =
        separated_pair(parse_times, newline, parse_distances)(input).unwrap();

    times.into_iter().zip(distances).map(Race::from).collect()
}

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(INPUT_FILE).unwrap();

    let races = parse_races(&input);

    let mut result = 1;
    for Race {
        time_ms,
        distance_mm,
    } in races
    {
        /*
            Formula: (t-h)*h > d, where
            t - race time
            h - (inner, in brackets) is button holding time
            h - (outer) is a velocity (mm per ms)
            d - record distance

            h - is what we are looking for

            It can be represented in the following way: h^2 - t*h + d < 0,
            d = t^2 - 4*d
            h1 = (t + sqrt(d))/2
            h2 = (t - sqrt(d))/2

            h1 = min(h1, h2)
            h2 = max(h1, h2)

            so, the h1..=h2 is the number of ways we can beat the previous record

               h1 ^[h1]   v[h2] h2
            ------0-------0--------
        */
        let d = time_ms * time_ms - 4 * distance_mm;
        let (h1, h2) = (
            ((time_ms as f64 + (d as f64).sqrt()) / 2.0),
            ((time_ms as f64 - (d as f64).sqrt()) / 2.0),
        );
        let (h1, h2) = (h1.min(h2), h1.max(h2));
        let (h1_ceil, h2_floor) = (h1.ceil(), h2.floor());
        let h1 = if h1_ceil == h1 {
            h1 as u64 + 1
        } else {
            h1_ceil as u64
        };

        let h2 = if h2_floor == h2 {
            h2 as u64 - 1
        } else {
            h2_floor as u64
        };

        result *= h2 - h1 + 1;
    }

    println!("{result}");

    Ok(())
}
