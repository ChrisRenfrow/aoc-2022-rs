static WIN_PTS: u32 = 6;
static DRAW_PTS: u32 = 3;
static LOSE_PTS: u32 = 0;
static ROCK_PTS: u32 = 1;
static PAPER_PTS: u32 = 2;
static SCISS_PTS: u32 = 3;

#[derive(Debug, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

type Round = (Play, Play);
type TargetRound = (Play, Results);

#[derive(Debug, Copy, Clone)]
enum Results {
    Win,
    Lose,
    Draw,
}

fn parse_play_pt1(play: &str) -> Play {
    match play {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => unreachable!(),
    }
}

fn round_result_pt1(round: &Round) -> Results {
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

fn round_for_target(target: &TargetRound) -> Round {
    let (opp_play, target) = target;
    (
        *opp_play,
        match opp_play {
            Play::Rock => match target {
                Results::Win => Play::Paper,
                Results::Draw => Play::Rock,
                Results::Lose => Play::Scissors,
            },
            Play::Paper => match target {
                Results::Win => Play::Scissors,
                Results::Draw => Play::Paper,
                Results::Lose => Play::Rock,
            },
            Play::Scissors => match target {
                Results::Win => Play::Rock,
                Results::Draw => Play::Scissors,
                Results::Lose => Play::Paper,
            },
        },
    )
}

fn parse_play_pt2(play: &str) -> Play {
    match play {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => unimplemented!(),
    }
}

fn parse_target_result(target: &str) -> Results {
    match target {
        "X" => Results::Lose,
        "Y" => Results::Draw,
        "Z" => Results::Win,
        _ => unimplemented!(),
    }
}

fn points_for_round(round: &Round) -> u32 {
    (match round_result_pt1(round) {
        Results::Win => WIN_PTS,
        Results::Draw => DRAW_PTS,
        Results::Lose => LOSE_PTS,
    }) + (match round.1 {
        Play::Rock => ROCK_PTS,
        Play::Paper => PAPER_PTS,
        Play::Scissors => SCISS_PTS,
    })
}

#[aoc_generator(day2, part1)]
fn input_generator_pt1(input: &str) -> Vec<Round> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            let mut round = l.split(' ');
            let p1 = parse_play_pt1(round.next().unwrap());
            let p2 = parse_play_pt1(round.next().unwrap());
            (p1, p2)
        })
        .collect()
}

#[aoc_generator(day2, part2)]
fn input_generator_pt2(input: &str) -> Vec<TargetRound> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            let mut round = l.split(' ');
            let p1 = parse_play_pt2(round.next().unwrap());
            let p2 = parse_target_result(round.next().unwrap());
            (p1, p2)
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_d02_pt1(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(points_for_round).sum()
}

#[aoc(day2, part2)]
fn solve_d02_pt2(target_rounds: &Vec<TargetRound>) -> u32 {
    target_rounds
        .iter()
        .map(round_for_target)
        .collect::<Vec<Round>>()
        .iter()
        .map(points_for_round)
        .sum()
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
        let input = input_generator_pt1(EXAMPLE_INPUT);
        let expect = 15;
        let actual = solve_d02_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_input_pt1() {
        let input = input_generator_pt1(FILE_INPUT);
        let expect = 10624;
        let actual = solve_d02_pt1(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn example_pt2() {
        let input = input_generator_pt2(EXAMPLE_INPUT);
        let expect = 12;
        let actual = solve_d02_pt2(&input);

        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_input_pt2() {
        let input = input_generator_pt2(FILE_INPUT);
        let expect = 14060;
        let actual = solve_d02_pt2(&input);

        assert_eq!(expect, actual);
    }
}
