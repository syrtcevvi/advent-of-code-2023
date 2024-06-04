use std::{
    cmp::{min, Ordering},
    collections::HashMap,
    fs,
    ops::Range,
    time::Instant,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

const INPUT_FILE: &str = "input.txt";

type SrcKey<'a> = &'a str;
type DstKey<'a> = &'a str;
type SrcRange = Range<i64>;
type DstRange = Range<i64>;

#[derive(Debug)]
struct Input<'a> {
    pub initial_seeds: Vec<Range<i64>>,
    pub transformation_sequence: Vec<(SrcKey<'a>, DstKey<'a>)>,
    pub transformations: HashMap<(SrcKey<'a>, DstKey<'a>), Vec<(SrcRange, DstRange)>>,
}

fn parse_initial_seed_ranges(input: &str) -> IResult<&str, Vec<Range<i64>>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, raw_ranges) =
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<i64>()))(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        raw_ranges
            .chunks(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect(),
    ))
}

fn parse_transformation_step(
    input: &str,
) -> IResult<&str, ((SrcKey, DstKey), Vec<(SrcRange, DstRange)>)> {
    let (input, _) = newline(input)?;

    let (input, (src_key, dst_key)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((tag(" map:"), newline)),
    )(input)?;

    let (input, lists) = separated_list1(
        newline,
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<i64>())),
    )(input)?;

    Ok((
        input,
        (
            (src_key, dst_key),
            lists
                .into_iter()
                .map(|list| (list[1]..list[1] + list[2], list[0]..list[0] + list[2]))
                .collect(),
        ),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, initial_seeds) = parse_initial_seed_ranges(input)?;
    let (input, transformations) = separated_list1(newline, parse_transformation_step)(input)?;

    let transformation_sequence = transformations.iter().map(|item| item.0).collect();

    Ok((
        input,
        Input {
            initial_seeds,
            transformation_sequence,
            transformations: transformations.into_iter().collect(),
        },
    ))
}

fn transform_range(
    range: Range<i64>,
    related_transformations: &[(SrcRange, DstRange)],
) -> Vec<Range<i64>> {
    use Ordering::{Equal, Greater, Less};

    if related_transformations.is_empty() {
        return vec![range];
    }

    let mut result: Vec<Range<i64>> = Vec::with_capacity(16);

    let mut pending_ranges = Vec::with_capacity(4);
    pending_ranges.push(range);

    'outer: while let Some(init) = pending_ranges.pop() {
        for (from, to) in related_transformations {
            let bias = to.start - from.start;

            /*
               Initial range (init): A..B,
               Transformation range (from): C..D

               Attention: Ranges are half-open
            */
            match (
                // A and C
                init.start.cmp(&from.start),
                // A and D
                init.start.cmp(&from.end),
                // B and C
                init.end.cmp(&from.start),
                // B and D
                init.end.cmp(&from.end),
            ) {
                /*
                    --Avv___B------- init
                    -----C----D----- from

                    or

                    --Avv_____B----- init
                    -----C----D----- from
                */
                (Less, _, Greater, Less) | (Less, _, Greater, Equal) => {
                    // A..C
                    pending_ranges.push(init.start..from.start);
                    // C..B
                    result.push(from.start + bias..init.end + bias);
                    continue 'outer;
                }
                /*
                    -------A__vvvB-- init
                    -----C----D----- from

                    or

                    -----A____vvvB-- init
                    -----C----D----- from
                */
                (Equal, Less, _, Greater) | (Greater, Less, _, Greater) => {
                    // A..D
                    result.push(init.start + bias..from.end + bias);
                    //D..B
                    pending_ranges.push(from.end..init.end);
                    continue 'outer;
                }
                /*
                    -----A____B------- init
                    -----C----D----- from

                    or

                    ------A__B------ init
                    -----C----D----- from

                    or

                    -------A__B----- init
                    -----C----D----- from

                    or

                    -----A__B------- init
                    -----C----D----- from
                */
                (Equal, _, _, Equal)
                | (Greater, _, _, Less)
                | (Greater, Less, _, Equal)
                | (Equal, _, Greater, Less) => {
                    result.push(init.start + bias..init.end + bias);
                    continue 'outer;
                }
                /*
                    -------AvvvvB--------
                    C--D
                    or
                    -------AvvvvB--------
                    ----------------C---D

                    and others?
                */
                _ => {}
            }
        }

        result.push(init);
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_content = fs::read_to_string(INPUT_FILE).unwrap();

    let mut input = parse_input(&input_file_content)
        .expect("Unable to parse the input")
        .1;
    input.initial_seeds.sort_by(|a, b| a.start.cmp(&b.start));

    let mut ranges = input.initial_seeds.clone();

    for key in input.transformation_sequence {
        let mut next_ranges_generation = Vec::with_capacity(ranges.len() * 2);

        for current_range in ranges {
            next_ranges_generation
                .extend(transform_range(current_range, &input.transformations[&key]));
        }

        ranges = next_ranges_generation;
    }

    let min = ranges.into_iter().map(|item| item.start).min().unwrap();

    println!("{min}");

    Ok(())
}
