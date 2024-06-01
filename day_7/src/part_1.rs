use std::{cmp::Ordering, collections::HashMap, convert::Infallible, fs::read_to_string};

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline, space1},
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    pub cards: Vec<Card>,
    pub kind: HandKind,
    pub bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Self {
        let kind = Self::determine_kind(&cards);
        Self { cards, kind, bid }
    }

    fn determine_kind(cards: &[Card]) -> HandKind {
        let mut card_kinds: HashMap<Card, usize> = HashMap::with_capacity(4);
        for card in cards {
            let entry = card_kinds.entry(card.clone()).or_default();
            *entry += 1;
        }
        let mut counters: Vec<usize> = card_kinds.into_iter().map(|pair| pair.1).collect();
        counters.sort_by(|a, b| b.cmp(a));

        match counters.as_slice() {
            [five, ..] if *five == 5 => HandKind::FiveOfAKind,
            [four, ..] if *four == 4 => HandKind::FourOfAKind,
            [three, _middle @ .., two] if *three == 3 && *two == 2 => HandKind::FullHouse,
            rest => {
                if rest == [3, 1, 1] {
                    HandKind::ThreeOfAKind
                } else if rest == [2, 2, 1] {
                    HandKind::TwoPair
                } else if rest == [2, 1, 1, 1] {
                    HandKind::OnePair
                } else {
                    HandKind::HighCard
                }
            }
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    alt((
        value(Card::Two, char('2')),
        value(Card::Three, char('3')),
        value(Card::Four, char('4')),
        value(Card::Five, char('5')),
        value(Card::Six, char('6')),
        value(Card::Seven, char('7')),
        value(Card::Eight, char('8')),
        value(Card::Nine, char('9')),
        value(Card::Ten, char('T')),
        value(Card::Jack, char('J')),
        value(Card::Queen, char('Q')),
        value(Card::King, char('K')),
        value(Card::Ace, char('A')),
    ))(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    many1(parse_card)(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map_res(
        separated_pair(parse_cards, space1, map_res(digit1, |s: &str| s.parse())),
        |(cards, bid)| Ok::<_, Infallible>(Hand::new(cards, bid)),
    )(input)
}

fn parse_hands(input: &str) -> Vec<Hand> {
    separated_list1(newline, parse_hand)(input).unwrap().1
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut hands = parse_hands(&read_to_string(INPUT_FILE).unwrap());

    hands.sort();

    let total_winnings = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1, hand))
        .fold(0, |acc, curr| acc + curr.0 * curr.1.bid);

    println!("{total_winnings}");
}
