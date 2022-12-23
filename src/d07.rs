use parser::Line;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .flat_map(parser::output_line)
        .map(|(_, l)| l)
        .collect()
}

#[aoc(day7, part1)]
fn solve_d07_pt1(output: &[Line]) -> usize {
    dbg!(&output);
    todo!()
}

mod parser {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, space1},
        combinator::{rest, value},
        sequence::{pair, preceded, terminated},
        IResult,
    };

    pub fn output_line(i: &str) -> IResult<&str, Line> {
        alt((cmd, file))(i)
    }

    fn cmd(i: &str) -> IResult<&str, Line> {
        preceded(pair(char('$'), space1), alt((ls, cd)))(i)
    }

    fn ls(i: &str) -> IResult<&str, Line> {
        value(Line::CmdLs, tag("ls"))(i)
    }

    fn cd(i: &str) -> IResult<&str, Line> {
        let (i, arg) = preceded(pair(tag("cd"), space1), rest)(i)?;
        Ok((i, Line::CmdCd(arg.to_string())))
    }

    fn file(i: &str) -> IResult<&str, Line> {
        alt((plain, dir))(i)
    }

    fn plain(i: &str) -> IResult<&str, Line> {
        let (i, size_str) = terminated(digit1, pair(space1, rest))(i)?;
        Ok((i, Line::File(size_str.parse::<usize>().unwrap())))
    }

    fn dir(i: &str) -> IResult<&str, Line> {
        let (i, label) = preceded(pair(tag("dir"), space1), alpha1)(i)?;
        Ok((i, Line::Dir(label.to_string())))
    }

    #[derive(Debug, Clone)]
    pub enum Line {
        CmdLs,
        CmdCd(String),
        File(usize),
        Dir(String),
    }
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

    // #[ignore]
    // #[test]
    // fn example_pt2() {
    //     let input = parse_input(EXAMPLE_INPUT);
    //     let expect = 0;
    //     let actual = solve_d07_pt1(&input);
    //     assert_eq!(expect, actual);
    // }

    // #[ignore]
    // #[test]
    // fn solve_pt2() {
    //     let input = parse_input(FILE_INPUT);
    //     let expect = 0;
    //     let actual = solve_d07_pt1(&input);
    //     assert_eq!(expect, actual);
    // }
}
