use hashbrown::HashSet;

type Sack = (Vec<Item>, Vec<Item>);
type Item = char;

#[aoc_generator(day3, part1)]
fn parse_input_pt1(input: &str) -> Vec<Sack> {
    input.trim().lines().map(line_to_sack).collect()
}

fn line_to_sack(line: &str) -> Sack {
    let length = line.len();
    let chars = line.chars().collect::<Vec<Item>>();
    (chars[0..length / 2].to_vec(), chars[length / 2..].to_vec())
}

#[aoc(day3, part1)]
fn solve_d03_pt1(sacks: &[Sack]) -> u32 {
    sacks.iter().map(|s| item_priority(common_item(s))).sum()
}

fn common_item(sack: &Sack) -> Item {
    let (comp_a, comp_b) = sack;
    for item in comp_a {
        if comp_b.contains(item) {
            return *item;
        }
    }

    unreachable!()
}

fn item_priority(item: Item) -> u32 {
    if item.is_uppercase() {
        item as u32 - ('A' as u32) + 27
    } else {
        item as u32 - ('a' as u32) + 1
    }
}

#[aoc_generator(day3, part2)]
fn parse_input_pt2(input: &str) -> Vec<String> {
    input.trim().lines().map(str::to_string).collect()
}

#[aoc(day3, part2)]
fn solve_d03_pt2(sacks: &[String]) -> u32 {
    sacks.chunks(3).map(group_value).sum()
}

fn group_value(sacks: &[String]) -> u32 {
    item_priority(group_badge(sacks))
}

fn group_badge(sacks: &[String]) -> Item {
    let sets: Vec<HashSet<char>> = sacks.iter().map(|s| s.chars().collect()).collect();

    sets.iter()
        .fold(HashSet::new(), |common, s| {
            if common.is_empty() {
                s.iter().copied().collect()
            } else {
                common.intersection(s).copied().collect()
            }
        })
        .drain()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    static FILE_INPUT: &str = include_str!("../input/2022/day3.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input_pt1(EXAMPLE_INPUT);
        let expect = 157;
        let actual = solve_d03_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_input_pt1() {
        let input = parse_input_pt1(FILE_INPUT);
        let expect = 7848;
        let actual = solve_d03_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let input = parse_input_pt2(EXAMPLE_INPUT);
        let expect = 70;
        let actual = solve_d03_pt2(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_input_pt2() {
        let input = parse_input_pt2(FILE_INPUT);
        let expect = 2616;
        let actual = solve_d03_pt2(&input);

        assert_eq!(expect, actual);
    }
}
