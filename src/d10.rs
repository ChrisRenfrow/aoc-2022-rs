use std::str::FromStr;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Ins> {
    input.trim().lines().map(|l| l.parse()).flatten().collect()
}

#[aoc(day10, part1)]
fn solve_d10_pt1(instructions: &Vec<Ins>) -> i32 {
    let mut vm = Vm::new(instructions);
    let cycles_of_interest: &[usize] = &[20, 60, 100, 140, 180, 220];
    vm.run();
    vm.eng
        .cycles
        .iter()
        .enumerate()
        .filter(|(n, _)| cycles_of_interest.contains(&(n + 1)))
        .zip(cycles_of_interest)
        .fold(0, |acc: i32, ((_, v), &c)| acc + c as i32 * v)
}

#[derive(Debug)]
struct Vm {
    ins: Vec<Ins>,
    eng: Engine,
}

impl Vm {
    pub fn new(ins: &Vec<Ins>) -> Self {
        Self {
            ins: ins.to_vec(),
            eng: Engine::new(),
        }
    }

    pub fn run(&mut self) {
        let engine = &mut self.eng;
        self.ins.iter().for_each(|i| engine.eval(i));
    }
}

#[derive(Debug)]
struct Engine {
    cycle: usize,
    cycles: Vec<i32>,
    x_reg: i32,
}

impl Engine {
    fn new() -> Self {
        Self {
            cycle: 0,
            cycles: vec![],
            x_reg: 1,
        }
    }

    fn eval(&mut self, i: &Ins) {
        match i {
            Ins::AddX(x) => self.addx(x),
            Ins::NoOp => self.noop(),
        }
    }

    fn addx(&mut self, x: &i32) {
        self.advance_cycle(2);
        self.x_reg += x;
    }

    fn noop(&mut self) {
        self.advance_cycle(1);
    }

    fn advance_cycle(&mut self, n: usize) {
        (0..n).for_each(|n| {
            self.cycle += 1;
            self.cycles.push(self.x_reg);
        });
    }
}

#[derive(Debug, Copy, Clone)]
enum Ins {
    AddX(i32),
    NoOp,
}

#[derive(Debug)]
struct ParseInsError;

impl FromStr for Ins {
    type Err = ParseInsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        Ok(match parts[0] {
            "addx" => Ins::AddX(parts[1].parse::<i32>().unwrap()),
            "noop" => Ins::NoOp,
            _ => unreachable!(),
        })
    }
}

mod tests {
    use super::*;

    static SMALL_EXAMPLE_INPUT: &str = "noop
addx 3
addx -5";
    static EXAMPLE_INPUT: &str = include_str!("../input/2022/d10-example.txt");
    static FILE_INPUT: &str = include_str!("../input/2022/day10.txt");

    #[test]
    fn small_example_pt1() {
        let input = parse_input(SMALL_EXAMPLE_INPUT);
        let expect = 0;
        let actual = solve_d10_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 13140;
        let actual = solve_d10_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 12980;
        let actual = solve_d10_pt1(&input);
        assert_eq!(expect, actual);
    }
}
