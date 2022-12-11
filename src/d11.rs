use std::str::FromStr;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| m.parse::<Monkey>().unwrap())
        .collect()
}

#[aoc(day11, part1)]
fn solve_d11_pt1(monkeys: &Vec<Monkey>) -> u32 {
    todo!()
}

type Item = u32;

/// Monkeh
// #[derive(Debug)]
struct Monkey {
    /// The id of the monkey as defined by the input.
    id: usize,
    /// A list of items the monkey starts with.
    start: Vec<Item>,
    /// The operation to apply to the currently held item.
    op: Box<dyn Fn(Option<Item>) -> u32>,
    /// The function used to decide which monkey to throw to.
    test: Box<dyn Fn(Item) -> bool>,
    /// The IDs of the monkeys the item will be thrown to if test evaluates true or false, respectively.
    throw: (usize, usize),
}

impl Monkey {
    fn new(
        id: usize,
        start: Vec<u32>,
        op: Box<dyn Fn(Option<u32>) -> u32>,
        test: Box<dyn Fn(u32) -> bool>,
        throw: (usize, usize),
    ) -> Self {
        Self {
            id,
            start,
            op,
            test,
            throw,
        }
    }
}

#[derive(Debug)]
enum Op {
    Mul,
    Add,
}

#[derive(Debug)]
struct MonkeyParseError;

impl FromStr for Monkey {
    type Err = MonkeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = parse_id(lines.next().unwrap());
        let start = parse_start(lines.next().unwrap());
        let op = parse_op(lines.next().unwrap());
        let test = parse_test(lines.next().unwrap());
        let throw = parse_throw(lines.next().unwrap());

        Ok(Monkey::new(id, start, op, test, throw))
    }
}

fn parse_id(s: &str) -> usize {
    s.split_whitespace().collect::<Vec<&str>>()[1]
        .chars()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap()
}

fn parse_start(s: &str) -> Vec<u32> {
    s.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(", ")
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

fn parse_op(s: &str) -> Box<dyn Fn(Option<u32>) -> u32> {
    let mut op_str = s
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" = ")
        .nth(1)
        .unwrap()
        .split_whitespace();
    let op = match op_str.nth(1).unwrap() {
        "*" => Op::Mul,
        "+" => Op::Add,
        _ => unreachable!(),
    };
    let b = op_str.next().unwrap().parse::<u32>().unwrap();

    match op {
        Op::Mul => Box::new(move |a| a.unwrap_or(b) * b),
        Op::Add => Box::new(move |a| a.unwrap_or(b) + b),
    }
}

fn parse_test(s: &str) -> Box<dyn Fn(u32) -> bool> {
    todo!()
}

// returns (if_true_id, if_false_id)
fn parse_throw(s: &str) -> (usize, usize) {
    todo!()
}

mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input/2022/d11-example.txt");
    static FILE_INPUT: &str = include_str!("../input/2022/day11.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 42;
        let actual = solve_d11_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[ignore]
    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 42;
        let actual = solve_d11_pt1(&input);
        assert_eq!(expect, actual);
    }
}
