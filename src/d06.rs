use std::collections::HashSet;
use std::ops::ControlFlow;

#[aoc(day6, part1)]
fn solve_d06_pt1(stream: &[u8]) -> usize {
    let mut i = 0;

    stream
        .windows(4)
        .try_for_each(|q| {
            let mut h = HashSet::with_capacity(4);
            i += 1;
            if q.iter().all(|&b| h.insert(b)) {
                ControlFlow::Break(i + 3)
            } else {
                ControlFlow::Continue(())
            }
        })
        .break_value()
        .unwrap()
}

#[aoc(day6, part2)]
fn solve_d06_pt2(stream: &[u8]) -> usize {
    let mut i = 0;

    stream
        .windows(14)
        .try_for_each(|q| {
            let mut h = HashSet::with_capacity(14);
            i += 1;
            if q.iter().all(|&b| h.insert(b)) {
                ControlFlow::Break(i + 13)
            } else {
                ControlFlow::Continue(())
            }
        })
        .break_value()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT_1: &[u8] = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();
    static EXAMPLE_INPUT_2: &[u8] = "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes();
    static FILE_INPUT: &[u8] = include_str!("../input/2022/day6.txt").as_bytes();

    #[test]
    fn example_pt1() {
        let expect = 7;
        let actual = solve_d06_pt1(EXAMPLE_INPUT_1);
        assert_eq!(expect, actual);
        let expect = 5;
        let actual = solve_d06_pt1(EXAMPLE_INPUT_2);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let expect = 1794;
        let actual = solve_d06_pt1(FILE_INPUT);

        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let expect = 19;
        let actual = solve_d06_pt2(EXAMPLE_INPUT_1);
        assert_eq!(expect, actual);
        let expect = 23;
        let actual = solve_d06_pt2(EXAMPLE_INPUT_2);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt2() {
        let expect = 2851;
        let actual = solve_d06_pt2(FILE_INPUT);

        assert_eq!(expect, actual);
    }
}
