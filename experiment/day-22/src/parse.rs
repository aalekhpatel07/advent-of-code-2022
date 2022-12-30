use nom::{
    IResult, 
    multi::many1, 
    combinator::map, 
    bytes::complete::tag, 
    branch::alt,
    character::complete::u64 as parse_u64
};

use crate::{Board, Instructions, Step, Tile};

// N = 150
// M = 200

pub fn parse_board<const N: usize, const M: usize>(s: &str) -> Board<N, M> {
    let mut board = [[Tile::default(); N]; M];

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => board[y][x] = Tile::Open,
                '#' => board[y][x] = Tile::Occupied,
                ' ' => board[y][x] = Tile::Blank,
                _ => panic!("Invalid character in board"),
            }
        }
    }

    Board { board }
}

pub fn parse_instructions(s: &str) -> IResult<&str, Instructions> {
    map(
        many1(
            alt((
                map(tag("L"), |_| Step::CounterClockwise),
                map(tag("R"), |_| Step::Clockwise),
                map(
                    parse_u64,
                    Step::Forward,
                ),
            )),
        ),
        |data: Vec<Step>| {
            Instructions {
                instructions: data,
            }
        }
    )(s)
}