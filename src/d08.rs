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

fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let tree = trees[y][x];
    let height = trees.len();
    let width = trees[0].len();

    let right = trees[y][x + 1..width].iter().all(|&o| o < tree);
    let left = trees[y][0..x].iter().all(|&o| o < tree);
    let up = trees[y + 1..height].iter().all(|o| o[x] < tree);
    let down = trees[0..y].iter().all(|o| o[x] < tree);

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
        let expect = 0;
        let actual = solve_d08_pt1(&input);
        assert_eq!(expect, actual);
    }
}
