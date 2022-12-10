use std::cmp;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::str::FromStr;

/**
 * Approach:
 * - Parse moves
 * - Use moves to calculate size of grid to allocate/initialize
 * - Simulate tail movements from head movements, tracking number of unvisited cells as they are visited
 * - Return number of unique visits
 */

struct RopeBoard {
    /// The 2D Vec used to track tail visitations
    pub grid: Vec<Vec<bool>>,
    /// List of rope head moves
    pub moves: Vec<Move>,
    /// Convenience variable for storing and accessing the location of the rope head
    head: Point,
    /// Convenience variable for storing and accessing the location of the rope end
    tail: Point,
}

impl Debug for RopeBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let c = self.grid[y][x];
                if self.head.x == x as i32 && self.tail.y == y as i32 {
                    write!(f, "(H");
                } else if self.tail.x == x as i32 && self.tail.y == y as i32 {
                    write!(f, "T)");
                } else if c {
                    write!(f, "##");
                } else {
                    write!(f, "__");
                }
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// # Panics
    /// If move would overflow usize.
    fn move_direction(&mut self, dir: Dir, d: i32) {
        match dir {
            Dir::U => self.y = self.y.checked_add(d).unwrap(),
            Dir::D => self.y = self.y.checked_sub(d).unwrap(),
            Dir::R => self.x = self.x.checked_add(d).unwrap(),
            Dir::L => self.x = self.x.checked_sub(d).unwrap(),
        }
    }

    fn diff(&self, lhs: &Point) -> usize {
        (((self.x as isize - lhs.x as isize).pow(2) + (self.y as isize - lhs.y as isize).pow(2))
            as f32)
            .sqrt() as usize
    }
}

impl RopeBoard {
    fn new(moves: &Vec<Move>) -> Self {
        let ((w, h), origin) = RopeBoard::calc_dimensions_and_origin(moves);
        Self {
            moves: moves.to_vec(),
            grid: vec![vec![false; w + 1]; h + 1],
            head: origin,
            tail: origin,
        }
    }

    /// Given a move, move head and tail according to the following rules:
    /// 1. Head cannot be farther than one cell away in any direction (up, down, left, right, diagonally)
    /// 2. If tail is diagonal to head and head would move out of range,
    ///    tail should end up in the space head was formerly occupying
    fn move_rope(&mut self, m: &Move) {
        for _ in 0..m.dist {
            self.head.move_direction(m.dir, 1);
            let diff = self.tail.diff(&self.head);
            if diff > 1 {
                self.tail.move_direction(m.dir, 1);
                match m.dir {
                    Dir::U | Dir::D => self.tail.x = self.head.x,
                    Dir::R | Dir::L => self.tail.y = self.head.y,
                }
            }

            self.tail_visit();
        }

        dbg!(&self);
    }

    fn tail_visit(&mut self) {
        let (x, y): (usize, usize) = (
            self.tail.x.try_into().unwrap(),
            self.tail.y.try_into().unwrap(),
        );
        if !self.grid[y][x] {
            self.grid[y][x] = true;
        }
    }

    /// Return the dimensions of the grid and point of origin as indicated by the moves and their distances.
    fn calc_dimensions_and_origin(moves: &Vec<Move>) -> ((usize, usize), Point) {
        let (mut x, mut y): (i32, i32) = (0, 0);
        let (mut max_x, mut max_y): (i32, i32) = (0, 0);
        let (mut min_x, mut min_y): (i32, i32) = (0, 0);

        for m in moves {
            match m.dir {
                Dir::U => y += m.dist as i32,
                Dir::D => y -= m.dist as i32,
                Dir::R => x += m.dist as i32,
                Dir::L => x -= m.dist as i32,
            }
            max_x = cmp::max(x, max_x);
            min_x = cmp::max(x, min_x);
            max_y = cmp::max(y, max_y);
            min_y = cmp::max(y, min_y);
        }

        (
            (
                (max_x + min_x.abs() + 1) as usize,
                (max_y + min_y.abs() + 1) as usize,
            ),
            Point::new(min_x.abs() + 1, min_y.abs() + 1),
        )
    }

    /// Move head and simulate tail movements, tracking tail visitations in grid
    pub fn simulate_moves(&mut self) {
        self.moves.to_vec().iter().for_each(|m| {
            self.move_rope(m);
        })
    }

    /// Sum total of visited cells
    pub fn get_visited_cell_ct(&self) -> usize {
        self.grid
            .iter()
            .map(|r| r.iter().filter(|c| **c).count())
            .sum::<usize>()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    U,
    D,
    R,
    L,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Move {
    dist: usize,
    dir: Dir,
}

impl Move {
    fn new(dist: usize, dir: Dir) -> Self {
        Move { dist, dir }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<&str> = s.split_whitespace().collect();
        let n = p[1].parse::<usize>().unwrap();
        Ok(match p[0] {
            "U" => Move::new(n, Dir::U),
            "D" => Move::new(n, Dir::D),
            "L" => Move::new(n, Dir::L),
            "R" => Move::new(n, Dir::R),
            _ => unreachable!(),
        })
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Move> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Move>().unwrap())
        .collect::<Vec<Move>>()
}

#[aoc(day9, part1)]
fn solve_d09_pt1(moves: &Vec<Move>) -> usize {
    let mut board = RopeBoard::new(moves);
    board.simulate_moves();
    board.get_visited_cell_ct()
}

mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    static FILE_INPUT: &str = include_str!("../input/2022/day9.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 13;
        let actual = solve_d09_pt1(&input);
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 42;
        let actual = solve_d09_pt1(&input);
        assert_eq!(expect, actual);
    }
}
