use std::collections::BinaryHeap;

type Food = Vec<u32>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Food> {
    input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1, iter)]
pub fn solve_d01_pt1(elves: &[Food]) -> u32 {
    let mut max_calories = 0;

    for elf in elves {
        let sum = elf.iter().sum();
        max_calories = if sum > max_calories {
            sum
        } else {
            max_calories
        }
    }

    max_calories
}

#[aoc(day1, part2, iter)]
pub fn solve_d01_pt2(elves: &[Food]) -> u32 {
    let mut totals = elves.iter().map(|e| e.iter().sum()).collect::<Food>();
    totals.sort_by(|a, b| b.cmp(a));

    totals.chunks(3).next().unwrap().iter().sum()
}

#[aoc(day1, part1, heap)]
pub fn solve_d01_pt1_heap(elves: &[Food]) -> u32 {
    sum_top_n(elves, 1)
}

#[aoc(day1, part2, heap)]
pub fn solve_d01_pt2_heap(elves: &[Food]) -> u32 {
    sum_top_n(elves, 2)
}

fn sum_top_n(elves: &[Food], n: usize) -> u32 {
    let mut heap = BinaryHeap::<u32>::with_capacity(elves.len());
    elves.iter().for_each(|e| heap.push(e.iter().sum()));
    heap.into_iter_sorted()
        .take(n)
        .collect::<Food>()
        .iter()
        .sum()
}
