fn split_individuals<'a>(elves: &'a String) -> Vec<&'a str> {
    elves.split("\n\n").collect::<Vec<&'a str>>()
}

pub fn solve_d01_pt1(input: String) -> usize {
    let elves_inventory = split_individuals(&input);
    let mut max_calories = 0;

    for elf in elves_inventory {
        let sum = elf.lines().map(|l| l.parse::<usize>().unwrap()).sum();
        max_calories = if sum > max_calories {
            sum
        } else {
            max_calories
        }
    }

    max_calories
}

pub fn solve_d01_pt2(input: String) -> usize {
    let elves_inventory = split_individuals(&input);
    let mut totals = elves_inventory
        .iter()
        .map(|e| {
            e.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    totals.sort_by(|a, b| b.cmp(a));

    totals.chunks(3).next().unwrap().iter().sum()
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

    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn example_pt1() {
        let expect: usize = 24000;
        let actual = solve_d01_pt1(EXAMPLE_INPUT.to_string());

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let expect = 69912;
        let actual = solve_d01_pt1(INPUT.to_string());

        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let expect: usize = 45000;
        let actual = solve_d01_pt2(EXAMPLE_INPUT.to_string());

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt2() {
        let expect = 208180;
        let actual = solve_d01_pt2(INPUT.to_string());

        assert_eq!(expect, actual);
    }
}
