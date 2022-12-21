use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<Move>, Vec<Stack>) {
    let parts = input.trim().split("\n\n").collect::<Vec<&str>>();
    let moves = parts[1]
        .lines()
        .map(parser::parse_move)
        .flatten()
        .map(|(_, m)| m)
        .collect::<Vec<Move>>();

    dbg!(&moves);

    let stack_rows = parts[0]
        .lines()
        .map(parser::parse_stack_row)
        .flatten()
        .map(|(_, r)| r)
        .collect::<Vec<Vec<Option<Crate>>>>();

    dbg!(&stack_rows);

    // recreate rows as columns
    let stacks = todo!();

    dbg!(&stacks);

    (moves, stacks)
}

#[aoc(day5, part1)]
fn solve_d05_pt1(input: &(Vec<Move>, Vec<Stack>)) -> String {
    let (moves, stacks) = input;
    todo!()
}

pub type Stack = VecDeque<Option<Crate>>;

#[derive(Debug)]
pub struct Crate {
    label: char,
}

#[derive(Debug)]
pub struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn new(qty: usize, from: usize, to: usize) -> Self {
        Self { qty, from, to }
    }
}

mod parser {
    use super::{Crate, Move};
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, space0, space1},
        combinator::map_res,
        sequence::{delimited, preceded, terminated},
        IResult,
    };

    pub fn parse_move(input: &str) -> IResult<&str, Move> {
        let (input, qty) = field("move", input)?;
        let (input, from) = field("from", input)?;
        let (input, to) = field("to", input)?;
        Ok((input, Move::new(qty, from, to)))
    }

    fn field<'a>(field: &'a str, input: &'a str) -> IResult<&'a str, usize> {
        let (input, _) = terminated(tag(field), space1)(input)?;
        terminated(map_res(digit1, number), space0)(input)
    }

    fn number(input: &str) -> Result<usize, std::num::ParseIntError> {
        input.parse()
    }

    pub fn parse_stack_row(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
        // crates in a row are chunks of 3 bytes followed by a single space
        todo!()
    }

    fn parse_crate(input: &str) -> IResult<&str, Option<Crate>> {
        // "    " -> None
        // "[A] " -> Some(Crate { label: 'A' })
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    static FILE_INPUT: &str = include_str!("../input/2022/day5.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = " ";
        let actual = solve_d05_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[ignore]
    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = " ";
        let actual = solve_d05_pt1(&input);

        assert_eq!(expect, actual);
    }

    // #[test]
    // fn example_pt2() {
    //     let input = input_generator(EXAMPLE_INPUT);
    //     let expect = 4;
    //     let actual = solve_d04_pt2(&input);

    //     assert_eq!(expect, actual);
    // }

    // #[test]
    // fn solve_pt2() {
    //     let input = input_generator(FILE_INPUT);
    //     let expect = 893;
    //     let actual = solve_d04_pt2(&input);

    //     assert_eq!(expect, actual);
    // }
}
