static WIN_PTS: u32 = 6;
static DRAW_PTS: u32 = 3;
static LOSE_PTS: u32 = 0;
static ROCK_PTS: u32 = 1;
static PAPER_PTS: u32 = 2;
static SCISS_PTS: u32 = 3;

#[aoc_generator(day2, part1)]
fn input_generator_pt1(input: &str) -> Vec<Round> {
    input.trim().split('\n').map(parse_round_pt1).collect()
}

fn parse_round_pt1(s: &str) -> Round {
    let plays = s.split(' ').collect::<Vec<&str>>();
    (parse_play_pt1(plays[0]), parse_play_pt1(plays[1]))
}

fn parse_play_pt1(play: &str) -> Play {
    match play {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => unreachable!(),
    }
}

#[aoc_generator(day2, part2)]
fn input_generator_pt2(input: &str) -> Vec<TargetRound> {
    input.trim().split('\n').map(parse_round_pt2).collect()
}

fn parse_round_pt2(s: &str) -> TargetRound {
    let plays = s.split(' ').collect::<Vec<&str>>();
    (parse_play_pt2(plays[0]), parse_target_result(plays[1]))
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

#[aoc(day2, part1)]
fn solve_d02_pt1(rounds: &[Round]) -> u32 {
    rounds.iter().map(points_for_round).sum()
}

#[aoc(day2, part2)]
fn solve_d02_pt2(target_rounds: &[TargetRound]) -> u32 {
    target_rounds
        .iter()
        .map(round_for_target)
        .collect::<Vec<Round>>()
        .iter()
        .map(points_for_round)
        .sum()
}

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
