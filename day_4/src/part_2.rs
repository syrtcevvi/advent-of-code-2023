use std::{cell::Cell, cmp::min, collections::HashSet, fs::read_to_string};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

const INPUT_FILE: &str = "input.txt";

fn parse_card_header(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((tag("Card"), space1, digit1, tag(":")))(input)?;
    Ok((input.trim(), ()))
}

fn parse_card_lists(input: &str) -> IResult<&str, usize> {
    separated_pair(
        terminated(
            separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>())),
            space1,
        ),
        tag("|"),
        preceded(
            space1,
            separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>())),
        ),
    )(input)
    .map(|(s, (lhs, rhs))| {
        let lhs: HashSet<u64> = HashSet::from_iter(lhs);
        let rhs = HashSet::from_iter(rhs);
        (s, lhs.intersection(&rhs).count())
    })
}

fn parse_cards(input: &str) -> Vec<usize> {
    separated_list1(newline, tuple((parse_card_header, parse_card_lists)))(input)
        .unwrap()
        .1
        .into_iter()
        .map(|i| i.1)
        .collect()
}

fn main() {
    let cards: Vec<(usize, Cell<usize>)> = parse_cards(&read_to_string(INPUT_FILE).unwrap())
        .into_iter()
        .map(|i| (i, Cell::new(1)))
        .collect();

    let mut total_cards = 0;

    for (i, (winning_numbers, quantity)) in cards.iter().enumerate() {
        total_cards += quantity.get();

        for j in (i + 1)..=min(i + winning_numbers, cards.len()) {
            let cell = cards[j].1.get();
            cards[j].1.set(cell + quantity.get())
        }
    }

    println!("{total_cards}");
}
