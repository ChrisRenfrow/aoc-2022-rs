use itertools::Itertools;

type Pair = (Assignment, Assignment);
type Assignment = (u32, u32);

fn parse_assignment(s: &str) -> Assignment {
    s.split('-')
        .map(|a| a.parse().unwrap())
        .tuple_windows()
        .next()
        .unwrap()
}

fn parse_pair(s: &str) -> Pair {
    s.split(',')
        .map(parse_assignment)
        .tuple_windows()
        .next()
        .unwrap()
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Pair> {
    input.trim().lines().map(parse_pair).collect()
}

fn either_contains_other(left: &Assignment, right: &Assignment) -> bool {
    let (l_low, l_high) = left;
    let (r_low, r_high) = right;
    (l_low <= r_low && l_high >= r_high) || (r_low <= l_low && r_high >= l_high)
}

fn is_overlapping(left: &Assignment, right: &Assignment) -> bool {
    let (l_low, l_high) = left;
    let (r_low, r_high) = right;
    (l_low <= r_high && l_high >= r_low) || (r_low <= l_high && r_high >= l_low)
}

#[aoc(day4, part1)]
fn solve_d04_pt1(pairs: &Vec<Pair>) -> u32 {
    pairs.iter().fold(0, |acc, p| {
        let (left, right) = p;

        acc + if either_contains_other(left, right) {
            1
        } else {
            0
        }
    })
}

#[aoc(day4, part2)]
fn solve_d04_pt2(pairs: &Vec<Pair>) -> u32 {
    pairs.iter().fold(0, |acc, p| {
        let (left, right) = p;

        acc + if is_overlapping(left, right) { 1 } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    static FILE_INPUT: &str = include_str!("../input/2022/day4.txt");

    #[test]
    fn example_pt1() {
        let input = input_generator(EXAMPLE_INPUT);
        let expect = 2;
        let actual = solve_d04_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = input_generator(FILE_INPUT);
        let expect = 582;
        let actual = solve_d04_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let input = input_generator(EXAMPLE_INPUT);
        let expect = 4;
        let actual = solve_d04_pt2(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt2() {
        let input = input_generator(FILE_INPUT);
        let expect = 893;
        let actual = solve_d04_pt2(&input);

        assert_eq!(expect, actual);
    }
}
