use std::{collections::HashMap, error::Error, sync::OnceLock};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Pipe(Pipe),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}
use Pipe::*;

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    alt((
        value(Tile::Start, char('S')),
        value(Tile::Ground, char('.')),
        value(Tile::Pipe(Pipe::Vertical), char('|')),
        value(Tile::Pipe(Pipe::Horizontal), char('-')),
        value(Tile::Pipe(Pipe::TopRight), char('L')),
        value(Tile::Pipe(Pipe::TopLeft), char('J')),
        value(Tile::Pipe(Pipe::BottomLeft), char('7')),
        value(Tile::Pipe(Pipe::BottomRight), char('F')),
    ))(input)
}

fn parse_tile_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(parse_tile)(input)
}

fn parse_tile_map(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, parse_tile_line)(input)
}

const TOP: isize = -1;
const BOTTOM: isize = 1;
const LEFT: isize = -1;
const RIGHT: isize = 1;

const TOP_EDGE: [Pipe; 3] = [Vertical, BottomLeft, BottomRight];
const BOTTOM_EDGE: [Pipe; 3] = [Vertical, TopLeft, TopRight];
const LEFT_EDGE: [Pipe; 3] = [Horizontal, TopRight, BottomRight];
const RIGHT_EDGE: [Pipe; 3] = [Horizontal, TopLeft, BottomLeft];

static PIPES_ADJACENCY_RULES: OnceLock<HashMap<Pipe, HashMap<(isize, isize), [Pipe; 3]>>> =
    OnceLock::new();

fn pipes_adjacency_rules() -> &'static HashMap<Pipe, HashMap<(isize, isize), [Pipe; 3]>> {
    PIPES_ADJACENCY_RULES.get_or_init(|| {
        HashMap::from_iter([
            (
                Pipe::Vertical,
                HashMap::from_iter([((TOP, 0), TOP_EDGE), ((BOTTOM, 0), BOTTOM_EDGE)]),
            ),
            (
                Pipe::Horizontal,
                HashMap::from_iter([((0, LEFT), LEFT_EDGE), ((0, RIGHT), RIGHT_EDGE)]),
            ),
            (
                Pipe::TopRight,
                HashMap::from_iter([((TOP, 0), TOP_EDGE), ((0, RIGHT), RIGHT_EDGE)]),
            ),
            (
                Pipe::TopLeft,
                HashMap::from_iter([((TOP, 0), TOP_EDGE), ((0, LEFT), LEFT_EDGE)]),
            ),
            (
                Pipe::BottomLeft,
                HashMap::from_iter([((BOTTOM, 0), BOTTOM_EDGE), ((0, LEFT), LEFT_EDGE)]),
            ),
            (
                Pipe::BottomRight,
                HashMap::from_iter([((BOTTOM, 0), BOTTOM_EDGE), ((0, RIGHT), RIGHT_EDGE)]),
            ),
        ])
    })
}

static START_TILE_ADJACENCY_RULES: OnceLock<HashMap<(isize, isize), [Pipe; 3]>> = OnceLock::new();

fn start_tile_adjacency_rules() -> &'static HashMap<(isize, isize), [Pipe; 3]> {
    START_TILE_ADJACENCY_RULES.get_or_init(|| {
        HashMap::from_iter([
            ((TOP, 0), TOP_EDGE),
            ((BOTTOM, 0), BOTTOM_EDGE),
            ((0, LEFT), LEFT_EDGE),
            ((0, RIGHT), RIGHT_EDGE),
        ])
    })
}

fn vertical_horizontal_shifts(
    (i, j): (isize, isize),
    (lines, columns): (isize, isize),
) -> impl Iterator<Item = (isize, isize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter(move |(m, n)| *m + i >= 0 && *n + j >= 0 && *m + i < lines && *n + j < columns)
}

fn find_adjacent_pipes(
    (i, j): (usize, usize),
    tile_map: &Vec<Vec<Tile>>,
) -> (((usize, usize), Pipe), ((usize, usize), Pipe)) {
    let (i, j) = (i as isize, j as isize);
    let v: Vec<((usize, usize), Pipe)> = vertical_horizontal_shifts(
        (i, j),
        (tile_map.len() as isize, tile_map[0].len() as isize),
    )
    .filter_map(|(m, n)| {
        let adjacency_rules = &start_tile_adjacency_rules()[&(m, n)];
        let tile = tile_map[(m + i) as usize][(n + j) as usize].clone();
        if let Tile::Pipe(pipe) = tile {
            if adjacency_rules.contains(&pipe) {
                return Some((((m + i) as usize, (n + j) as usize), pipe));
            }
        }
        None
    })
    .collect();
    (v[0], v[1])
}

fn next_pipe(
    previous_position: (usize, usize),
    current_position: ((usize, usize), Pipe),
    tile_map: &Vec<Vec<Tile>>,
) -> ((usize, usize), Pipe) {
    let previous_position = (previous_position.0 as isize, previous_position.1 as isize);
    let (i, j) = (
        current_position.0 .0 as isize,
        current_position.0 .1 as isize,
    );
    let v: Vec<((usize, usize), Pipe)> = vertical_horizontal_shifts(
        (i, j),
        (tile_map.len() as isize, tile_map[0].len() as isize),
    )
    .filter(|(m, n)| previous_position != (*m + i, *n + j))
    .filter_map(|(m, n)| {
        let adjacency_rules = &pipes_adjacency_rules()[&current_position.1].get(&(m, n))?;
        let tile = tile_map[(m + i) as usize][(n + j) as usize].clone();
        if let Tile::Pipe(pipe) = tile {
            if adjacency_rules.contains(&pipe) {
                return Some((((m + i) as usize, (n + j) as usize), pipe));
            }
        }
        None
    })
    .collect();

    v[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let tile_map = parse_tile_map(input)?.1;

    let start_position = tile_map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            line.iter()
                .position(|tile| *tile == Tile::Start)
                .map(|j| (i, j))
        })
        .collect::<Vec<(usize, usize)>>()[0];

    let (first, second) = find_adjacent_pipes(start_position, &tile_map);
    let (mut first, mut second) = ((start_position, first), (start_position, second));

    let mut farthest_point_steps_quantity = 1;

    loop {
        farthest_point_steps_quantity += 1;
        first = (first.1 .0, next_pipe(first.0, first.1, &tile_map));
        second = (second.1 .0, next_pipe(second.0, second.1, &tile_map));
        if first.1 == second.1 {
            break;
        }
    }

    println!("{farthest_point_steps_quantity}");

    Ok(())
}
