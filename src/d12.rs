use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use petgraph::algo::dijkstra;
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::UnGraphMap;

const HEIGHT_DISPLAY_CHARS: &[char] = &[
    '_', '.', ',', '-', '=', '+', '*', '\'', '\"', '^', ':', ';', '!', '|', '[', ']', '(', ')',
    '{', '}', '$', '#', '@', '&', '%', '0',
];

#[aoc_generator(day12)]
fn parse_input(input: &str) -> HeightMap {
    let mut start = Point2d::new(0, 0);
    let mut end = Point2d::new(0, 0);

    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            if let Some(x) = l.chars().position(|c| c == 'S') {
                start = Point2d::new(x, y);
            }
            if let Some(x) = l.chars().position(|c| c == 'E') {
                end = Point2d::new(x, y);
            }

            parse_line(l)
        })
        .collect::<Vec<Vec<Height>>>();

    HeightMap { start, end, map }
}

fn parse_line(line: &str) -> Vec<Height> {
    line.chars().map(parse_height).collect()
}

fn parse_height(c: char) -> Height {
    match c {
        'E' => 25,
        c if c.is_lowercase() => c as usize - 'a' as usize,
        _ => 0,
    }
}

#[aoc(day12, part1)]
fn solve_d12_pt1(heightmap: &HeightMap) -> usize {
    println!("{:?}", heightmap);
    let g = map_to_graph(heightmap);
    let res = dijkstra(&g, heightmap.start, None, |_| 1);
    // dbg!(&res);
    // graph_to_dot(&g);
    *res.get(&heightmap.end).unwrap()
}

struct HeightMap {
    start: Point2d,
    end: Point2d,
    map: Vec<Vec<Height>>,
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Start: {}, {}\nEnd: {}, {}\n\n",
            self.start.x, self.start.y, self.end.x, self.end.y
        )?;

        for (y, r) in self.map.iter().enumerate() {
            for (x, v) in r.iter().enumerate() {
                if self.start.x == x && self.start.y == y {
                    write!(f, "S")?;
                } else if self.end.x == x && self.end.y == y {
                    write!(f, "E")?;
                } else {
                    write!(
                        f,
                        "{}",
                        match v {
                            0..=25 => HEIGHT_DISPLAY_CHARS[*v],
                            _ => '?',
                        }
                    )?;
                }
            }

            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Debug for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

type Height = usize;

#[derive(Debug, Hash, Copy, Clone, PartialOrd, Eq)]
struct Point2d {
    x: usize,
    y: usize,
}

impl Display for Point2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Point2d {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x * self.y).cmp(&(other.x * other.y))
    }
}

impl Point2d {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type Point = (usize, usize);

impl HeightMap {
    fn get_traversable_neighbors(&self, x: usize, y: usize) -> Neighbors {
        let curr = self.map[y][x];
        vec![
            self.traversable(curr, self.up(x, y)),
            self.traversable(curr, self.right(x, y)),
            self.traversable(curr, self.down(x, y)),
            self.traversable(curr, self.left(x, y)),
        ]
    }

    fn traversable(&self, origin: usize, neighbor: Option<Point>) -> Option<Point> {
        if !neighbor.is_none() {
            let (x, y) = neighbor.unwrap();
            let n = self.map[y][x];
            if n <= origin + 1 && n >= origin.checked_sub(1).unwrap_or(0) {
                return neighbor;
            }
        }
        None
    }

    fn up(&self, x: usize, y: usize) -> Option<Point> {
        if y != 0 {
            Some((x, y - 1))
        } else {
            None
        }
    }

    fn down(&self, x: usize, y: usize) -> Option<Point> {
        if y != self.map.len() - 1 {
            Some((x, y + 1))
        } else {
            None
        }
    }

    fn left(&self, x: usize, y: usize) -> Option<Point> {
        if x != 0 {
            Some((x - 1, y))
        } else {
            None
        }
    }

    fn right(&self, x: usize, y: usize) -> Option<Point> {
        if x != self.map[0].len() - 1 {
            Some((x + 1, y))
        } else {
            None
        }
    }
}

fn map_to_graph(m: &HeightMap) -> UnGraphMap<Point2d, usize> {
    let mut graph = UnGraphMap::new();

    for (y, r) in m.map.iter().enumerate() {
        for (x, v) in r.iter().enumerate() {
            let curr = Point2d::new(x, y);
            let nbrs = m.get_traversable_neighbors(x, y);

            graph.add_node(curr);

            for n in nbrs {
                if let Some(n) = n {
                    graph.add_edge(curr, Point2d::new(n.0, n.1), *v);
                }
            }
        }
    }

    graph
}

fn graph_to_dot(g: &UnGraphMap<Point2d, usize>) -> std::io::Result<()> {
    std::fs::write("./output.dot", Dot::new(&g).to_string())
}

/// Clock-wise from top: (Up, Right, Down, Left)
type Neighbors = Vec<Option<(usize, usize)>>;

mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    static FILE_INPUT: &str = include_str!("../input/2022/day12.txt");

    #[test]
    fn example_pt1() {
        let input = parse_input(EXAMPLE_INPUT);
        let expect = 31;
        let actual = solve_d12_pt1(&input);
        panic!();
        assert_eq!(expect, actual);
    }

    #[test]
    fn solve_pt1() {
        let input = parse_input(FILE_INPUT);
        let expect = 42;
        let actual = solve_d12_pt1(&input);
        assert_eq!(expect, actual);
    }
}
