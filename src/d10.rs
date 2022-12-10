use std::str::FromStr;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Ins> {
    input.trim().lines().map(|l| l.parse()).flatten().collect()
}

#[aoc(day10, part1)]
fn solve_d10_pt1(instructions: &Vec<Ins>) -> i32 {
    let mut state: Vec<i32> = vec![];
    let mut x: i32 = 1;
    let cycles_of_interest: &[usize] = &[20, 60, 100, 140, 180, 220];

    for (n, i) in instructions.iter().enumerate() {
        match i {
            Ins::AddX(y) => {
                state.push(x);
                state.push(x);
                x += y;
                state.push(x);
            }
            _ => state.push(x),
        }
    }

    dbg!(&state);

    state
        .iter()
        .enumerate()
        .filter(|(n, _)| cycles_of_interest.contains(n))
        .zip(cycles_of_interest)
        .fold(0, |acc: i32, ((_, v), &c)| acc + c as i32 * v)
}

#[derive(Debug)]
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
        let expect = 0;
        let actual = solve_d10_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 0;
        let actual = solve_d10_pt1(&input);
        assert_eq!(expect, actual);
    }
}
