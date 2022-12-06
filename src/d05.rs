use std::collections::VecDeque;
use std::num::ParseIntError;
use std::result::Result;
use std::str::FromStr;

type Stack = VecDeque<Crate>;
type Crate = char;

#[derive(Debug)]
struct Depot {
    stacks: Vec<Stack>,
    orders: Vec<Order>,
}

#[derive(Debug, Copy, Clone)]
struct Order {
    qty: usize,
    from: usize,
    to: usize,
}

struct ParseOrderError;

fn parse_order(s: &str) -> Result<Order, ParseIntError> {
    let mut order_iter = s.trim().split(' ');
    Ok(Order {
        qty: order_iter.nth(1).unwrap().parse()?,
        from: order_iter.nth(1).unwrap().parse()?,
        to: order_iter.nth(1).unwrap().parse()?,
    })
}

impl FromStr for Order {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_order(&s)
    }
}

impl Depot {
    pub fn take_qty_from(mut self, qty: usize, stack: usize) -> Stack {
        let mut from = VecDeque::new();
        for n in 0..qty {
            let c = self.stacks[stack].pop_front().unwrap();
            from.push_front(c);
        }

        from
    }

    pub fn add_crates_to(mut self, stack: usize, crates: Stack) {
        for c in crates {
            self.stacks[stack].push_front(c);
        }
    }

    pub fn top_crates(&self) -> Vec<Crate> {
        self.stacks.iter().map(|s| s[0]).collect::<Vec<Crate>>()
    }
}

/// Particularly brittle parsing solution but good enough for now
fn parse_stacks(s: &str) -> Vec<Stack> {
    let stack_rows: Vec<Vec<char>> = s
        .lines()
        .map(|l| l[1..].chars().step_by(4).collect())
        .collect();

    // Turn rows into columns
    let num_stacks = stack_rows[0].len();
    let mut stacks: Vec<Stack> = vec![];

    for s in 0..num_stacks {
        stacks.push(VecDeque::new());
        for r in &stack_rows {
            if r[s].is_numeric() {
                break;
            } else if r[s].is_alphabetic() {
                stacks[s].push_front(r[s]);
            }
        }
    }

    stacks
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Depot {
    let (stack_str, orders_str) = input.split_once("\n\n").unwrap();
    let orders = orders_str
        .lines()
        .map(Order::from_str)
        .flatten()
        .collect::<Vec<Order>>();
    let stacks = parse_stacks(stack_str);

    // dbg!(&orders, &stacks);
    Depot { orders, stacks }
}

#[aoc(day5, part1)]
fn solve_d05_pt1(d: &Depot) -> String {
    dbg!(&d);
    for o in &d.orders {
        let from = &d.take_qty_from(o.qty, o.from);
        d.add_crates_to(o.to, *from);
    }
    d.top_crates().iter().cloned().collect::<String>()
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
