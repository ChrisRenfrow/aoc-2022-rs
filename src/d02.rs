static WIN_PTS: u32 = 6;
static DRAW_PTS: u32 = 3;
static LOSE_PTS: u32 = 0;
static ROCK_PTS: u32 = 1;
static PAPER_PTS: u32 = 2;
static SCISS_PTS: u32 = 3;

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

type Round = (Play, Play);

#[derive(Debug)]
enum Results {
    Win,
    Lose,
    Draw,
}

fn parse_play(play: &str) -> Play {
    match play {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => unreachable!(),
    }
}

fn round_result(round: &Round) -> Results {
    let (p1, p2) = round;
    match p2 {
        Play::Rock => match p1 {
            Play::Rock => Results::Draw,
            Play::Paper => Results::Lose,
            Play::Scissors => Results::Win,
        },
        Play::Paper => match p1 {
            Play::Rock => Results::Win,
            Play::Paper => Results::Draw,
            Play::Scissors => Results::Lose,
        },
        Play::Scissors => match p1 {
            Play::Rock => Results::Lose,
            Play::Paper => Results::Win,
            Play::Scissors => Results::Draw,
        },
    }
}

fn points_for_round(round: &Round) -> u32 {
    (match round_result(round) {
        Results::Win => WIN_PTS,
        Results::Draw => DRAW_PTS,
        Results::Lose => LOSE_PTS,
    }) + (match round.1 {
        Play::Rock => ROCK_PTS,
        Play::Paper => PAPER_PTS,
        Play::Scissors => SCISS_PTS,
    })
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Round> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            let mut round = l.split(' ');
            let p1 = parse_play(round.next().unwrap());
            let p2 = parse_play(round.next().unwrap());
            (p1, p2)
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_d02_pt1(rounds: &Vec<Round>) -> u32 {
    rounds.iter().fold(0, |acc, r| points_for_round(r) + acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "A Y
B X
C Z";
    static FILE_INPUT: &str = include_str!("../input/2022/day2.txt");

    #[test]
    fn example_pt1() {
        let input = input_generator(EXAMPLE_INPUT);
        let expect = 15;
        let actual = solve_d02_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_input() {
        let input = input_generator(FILE_INPUT);
        let expect = 10624;
        let actual = solve_d02_pt1(&input);

        assert_eq!(expect, actual);
    }
}
