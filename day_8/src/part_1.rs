use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum Step {
    Left,
    Right,
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    alt((value(Step::Left, char('L')), value(Step::Right, char('R'))))(input)
}

fn parse_steps(input: &str) -> IResult<&str, Vec<Step>> {
    let (input, steps) = many1(parse_step)(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    Ok((input, steps))
}

type NetworkNode<'a> = (&'a str, (&'a str, &'a str));

fn parse_network_node(input: &str) -> IResult<&str, NetworkNode> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
}

fn parse_network(input: &str) -> IResult<&str, Vec<NetworkNode>> {
    separated_list1(newline, parse_network_node)(input)
}

fn parse_map(input: &str) -> (Vec<Step>, Vec<NetworkNode>) {
    tuple((parse_steps, parse_network))(input).unwrap().1
}

const INPUT_FILE: &str = "input.txt";

const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let map = parse_map(&input);
    let steps = map.0;
    let network: HashMap<&str, (&str, &str)> =
        map.1.into_iter().map(|item| (item.0, item.1)).collect();

    let mut steps_quantity = 0;
    let mut current_node = (START_LABEL, network[START_LABEL]);

    'outer: loop {
        for current_step in &steps {
            steps_quantity += 1;

            let next_key = match current_step {
                Step::Left => current_node.1 .0,
                Step::Right => current_node.1 .1,
            };
            current_node = (next_key, network[next_key]);

            if next_key == END_LABEL {
                break 'outer;
            }
        }
    }

    println!("{steps_quantity}");
}
