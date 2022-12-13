use std::cell::Cell;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::rc::Rc;
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
    dbg!(&monkeys);
    todo!()
}

struct Game {
    monkeys: Vec<Rc<Cell<Monkey>>>,
}

impl Game {
    fn run_for(&mut self, rounds: usize) {
        for r in 0..rounds {
            self.round();
        }
    }

    fn round(&self) {
        self.monkeys.iter().map(|m| {
            let thrown = *m.into_inner().turn().to_vec();
        });
    }

    fn top_n_monkey_biz(&self, n: usize) -> usize {
        let mut sorted = self
            .monkeys
            .iter()
            .map(|m| m.into_inner().activity)
            .collect::<Vec<usize>>();
        sorted.sort();
        sorted.iter().take(n).fold(0, |acc, a| acc + a)
    }
}

type Item = u32;

/// Monkeh
struct Monkey {
    /// The id of the monkey as defined by the input.
    id: usize,
    /// A list of items the monkey starts with.
    items: VecDeque<Item>,
    /// The operation to apply to the currently held item.
    op: Box<dyn Fn(Item) -> u32>,
    /// The function used to decide which monkey to throw to.
    test: Box<dyn Fn(Item) -> bool>,
    /// The IDs of the monkeys the item will be thrown to if test evaluates true or false, respectively.
    throw: (usize, usize),
    /// Activity count, how many times the monkey has inspected an item over all rounds
    activity: usize,
}

impl Monkey {
    fn new(
        id: usize,
        items: VecDeque<u32>,
        op: Box<dyn Fn(u32) -> u32>,
        test: Box<dyn Fn(u32) -> bool>,
        throw: (usize, usize),
    ) -> Self {
        Self {
            id,
            items,
            op,
            test,
            throw,
            activity: 0,
        }
    }

    fn turn(&mut self) -> Vec<(usize, Item)> {
        let mut thrown_items: Vec<(usize, Item)> = vec![];

        while let Some(item) = self.inspect_next() {
            thrown_items.push(item);
        }

        thrown_items
    }

    fn inspect_next(&mut self) -> Option<(usize, Item)> {
        let mut items = &mut self.items;
        let item = items.pop_front();
        if item.is_none() {
            return None;
        }
        let item = (self.op)(item.unwrap());
        self.activity += 1;
        match (self.test)(item) {
            true => Some((self.throw.0, item)),
            false => Some((self.throw.1, item)),
        }
    }

    fn catch_item(&self, incoming: Item) {
        let mut items = &mut self.items;
        items.push_back(incoming);
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Monkey")
            .field("id", &self.id)
            .field("items", &self.items)
            .field("op", &"Box<dyn Fn(Item) -> u32>")
            .field("test", &"Box<dyn Fn(Item) -> bool>")
            .field("throw", &self.throw)
            .finish()
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
        let throw = parse_throw(lines.next().unwrap(), lines.next().unwrap());

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

fn parse_start(s: &str) -> VecDeque<Item> {
    s.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(", ")
        .map(|n| n.parse::<Item>().unwrap())
        .collect()
}

fn parse_op(s: &str) -> Box<dyn Fn(u32) -> u32> {
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
    // dbg!(&op_str);
    let b = op_str.next().unwrap().parse::<u32>();

    match op {
        Op::Mul => Box::new(move |a| a * b.as_ref().unwrap_or(&a)),
        Op::Add => Box::new(move |a| a + b.as_ref().unwrap_or(&a)),
    }
}

fn parse_test(s: &str) -> Box<dyn Fn(u32) -> bool> {
    let b = s
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    Box::new(move |a| a % b == 0)
}

// returns (if_true_id, if_false_id)
fn parse_throw(s1: &str, s2: &str) -> (usize, usize) {
    let t = s1
        .trim()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let f = s2
        .trim()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    (t, f)
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
