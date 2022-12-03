type Sack = (Vec<Item>, Vec<Item>);
type Item = char;

fn line_to_sack(line: &str) -> Sack {
    let length = line.len();
    let chars = line.chars().collect::<Vec<Item>>();
    (chars[0..length / 2].to_vec(), chars[length / 2..].to_vec())
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Sack> {
    input.trim().lines().map(line_to_sack).collect()
}

fn common_item(sack: &Sack) -> Item {
    let (comp_a, comp_b) = sack;
    for item in comp_a {
        if comp_b.contains(&item) {
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

#[aoc(day3, part1)]
fn solve_d03_pt1<'a>(sacks: &'a Vec<Sack>) -> u32 {
    sacks.iter().map(|s| item_priority(common_item(s))).sum()
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
        let input = input_generator(EXAMPLE_INPUT);
        let expect = 157;
        let actual = solve_d03_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_input_pt1() {
        let input = input_generator(FILE_INPUT);
        let expect = 7848;
        let actual = solve_d03_pt1(&input);

        assert_eq!(expect, actual);
    }
}
