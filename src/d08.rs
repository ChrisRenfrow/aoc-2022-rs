use std::ops::ControlFlow;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<u8> {
    line.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn get_col(trees: &Vec<Vec<u8>>, col: usize) -> Vec<u8> {
    trees.iter().map(|r| r[col]).collect()
}

fn all_visible(tree_slice: &[u8], h: u8) -> bool {
    tree_slice.iter().all(|&o| o < h)
}

fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let tree = trees[y][x];
    let height = trees.len();
    let width = trees[0].len();
    let row = &trees[y][..];
    let col = get_col(trees, x);

    let right = all_visible(&row[x + 1..width], tree);
    let left = all_visible(&row[0..x], tree);
    let up = all_visible(&col[y + 1..height], tree);
    let down = all_visible(&col[0..y], tree);

    right || left || up || down
}

#[aoc(day8, part1)]
fn solve_d08_pt1(trees: &Vec<Vec<u8>>) -> usize {
    let height = trees.len();
    let width = trees[0].len();

    let mut v_tree_ct = width * 2 + height * 2 - 4;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if is_visible(trees, x, y) {
                v_tree_ct += 1;
            }
        }
    }

    v_tree_ct
}

fn unwrap_ctrl(c: ControlFlow<usize, usize>) -> usize {
    if c.is_break() {
        c.break_value().unwrap()
    } else {
        c.continue_value().unwrap()
    }
}

fn get_score(tree_section: Vec<u8>, tree: u8) -> usize {
    unwrap_ctrl(tree_section.iter().try_fold(0, |acc, &other| {
        if other >= tree {
            ControlFlow::Break(acc + 1)
        } else {
            ControlFlow::Continue(acc + 1)
        }
    }))
}

fn visibility_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let tree = trees[y][x];
    let height = trees.len();
    let width = trees[0].len();
    let row = &trees[y][..].to_vec();
    let col = get_col(trees, x);

    let right = get_score(row[x + 1..width].to_vec(), tree);
    let left = get_score(row[0..x].iter().rev().map(|&a| a).collect(), tree);
    let up = get_score(col[y + 1..height].to_vec(), tree);
    let down = get_score(col[0..y].iter().rev().map(|&a| a).collect(), tree);

    right * left * up * down
}

#[aoc(day8, part2)]
fn solve_d08_pt2(trees: &Vec<Vec<u8>>) -> usize {
    let height = trees.len();
    let width = trees[0].len();

    let mut max_scenic_score = 1;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let score = visibility_score(trees, x, y);
            max_scenic_score = if score > max_scenic_score {
                score
            } else {
                max_scenic_score
            };
        }
    }

    max_scenic_score
}

mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";
    static FILE_INPUT: &str = include_str!("../input/2022/day8.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 21;
        let actual = solve_d08_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 1669;
        let actual = solve_d08_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 8;
        let actual = solve_d08_pt2(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt2() {
        let input = parse_input(FILE_INPUT);
        let expect = 331344;
        let actual = solve_d08_pt2(&input);
        assert_eq!(expect, actual);
    }
}
