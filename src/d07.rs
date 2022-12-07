use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Output {
    Command(Cmd),
    Out(Item),
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Item {
    Dir,
    File(usize),
}

// Brittle parsing for now
fn parse_line(line: &str) -> Output {
    let splt: Vec<&str> = line.trim().split(' ').collect();
    if splt[0] == "$" {
        dbg!(line);
        Output::Command(match splt[1] {
            "cd" => Cmd::Cd(splt[2].to_string()),
            "ls" => Cmd::Ls,
            &_ => unreachable!(),
        })
    } else {
        Output::Out(match splt[0] {
            "dir" => Item::Dir,
            _ => Item::File(splt[0].parse().unwrap()),
        })
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Output> {
    input.trim().lines().map(parse_line).collect()
}

fn sum_dirs_below_x(dirs: Vec<(String, usize)>, x: usize) -> usize {
    dirs.iter()
        .filter(|(_, size)| size <= &x)
        .fold(0, |acc, (_, size)| acc + size)
}

#[aoc(day7, part1)]
fn solve_d07_pt1(output: &Vec<Output>) -> usize {
    let mut dir_totals = HashMap::new();

    let mut curr_dir = "".to_string();

    for o in output {
        match o {
            Output::Command(Cmd::Cd(d)) => curr_dir = d.clone(),
            Output::Out(Item::File(s)) => {
                if let Some(d) = dir_totals.get_mut(&curr_dir) {
                    *d = *d + s;
                } else {
                    dir_totals.insert(curr_dir.clone(), *s);
                }
            }
            _ => (),
        }
    }

    sum_dirs_below_x(dir_totals.drain().collect(), 100000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    static FILE_INPUT: &str = include_str!("../input/2022/day7.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 95437;
        let actual = solve_d07_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 0;
        let actual = solve_d07_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[ignore]
    #[test]
    fn example_pt2() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 0;
        let actual = solve_d07_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[ignore]
    #[test]
    fn solve_pt2() {
        let input = parse_input(FILE_INPUT);
        let expect = 0;
        let actual = solve_d07_pt1(&input);
        assert_eq!(expect, actual);
    }
}
