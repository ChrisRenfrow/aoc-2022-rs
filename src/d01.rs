use std::collections::BinaryHeap;

type Food = Vec<u32>;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<Food> {
    input.split("\n\n").map(parse_elf_inv).collect()
}

fn parse_elf_inv(elf: &str) -> Food {
    elf.lines().flat_map(|s| s.parse::<u32>()).collect()
}

#[aoc(day1, part1, iter)]
pub fn solve_d01_pt1(elves: &[Food]) -> u32 {
    sum_top_n_iter(elves, 1)
}

#[aoc(day1, part2, iter)]
pub fn solve_d01_pt2(elves: &[Food]) -> u32 {
    sum_top_n_iter(elves, 3)
}

fn sum_top_n_iter(elves: &[Food], n: usize) -> u32 {
    elves.iter().map(|e| e.iter().sum::<u32>()).take(n).sum()
}

#[aoc(day1, part1, heap)]
pub fn solve_d01_pt1_heap(elves: &[Food]) -> u32 {
    sum_top_n_heap(elves, 1)
}

#[aoc(day1, part2, heap)]
pub fn solve_d01_pt2_heap(elves: &[Food]) -> u32 {
    sum_top_n_heap(elves, 3)
}

fn sum_top_n_heap(elves: &[Food], n: usize) -> u32 {
    let mut heap = BinaryHeap::<u32>::with_capacity(elves.len());
    elves.iter().for_each(|e| heap.push(e.iter().sum()));
    heap.into_iter_sorted().take(n).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    static FILE_INPUT: &str = include_str!("../input/2022/day1.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 24000;
        let actual = solve_d01_pt1(&input);
        assert_eq!(expect, actual);
        let actual = solve_d01_pt1_heap(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 69912;
        let actual = solve_d01_pt1(&input);
        assert_eq!(expect, actual);
        let actual = solve_d01_pt1_heap(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 45000;
        let actual = solve_d01_pt2(&input);
        assert_eq!(expect, actual);
        let actual = solve_d01_pt2_heap(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt2() {
        let input = parse_input(FILE_INPUT);
        let expect = 208180;
        let actual = solve_d01_pt2(&input);
        assert_eq!(expect, actual);
        let actual = solve_d01_pt2_heap(&input);
        assert_eq!(expect, actual);
    }
}
