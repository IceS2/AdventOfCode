// Thanks villuna for this explanation
// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
use std::{fs, ops::Range, collections::{HashMap, VecDeque}};

fn main() {
    let input: Vec<Vec<Node>> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .filter(|row| !row.is_empty())
        .enumerate()
        .map(|(row_index, row)|
            row
                .chars()
                .enumerate()
                .map(|(c_index, c)| Node::new(row_index, c_index, c))
                .collect::<Vec<Node>>()
        )
        .collect();

    let start: Position = input
        .iter()
        .enumerate()
        .map(|(r_index, row)|
            (r_index, row
                .iter()
                .position(|c| c.node_type == NodeType::Start)))
        .collect::<Vec<(usize, Option<usize>)>>()
        .iter()
        .filter(|(_, c)| c.is_some())
        .map(|(r, c)| (*r, c.unwrap()))
        .collect::<Vec<(usize, usize)>>()[0]
        .into();

    let gardens: Gardens = Gardens::new(input, start);

    let result = gardens.walk(10);

    println!("{:#?}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from(p: (usize, usize)) -> Self {
        Self {
            x: p.0,
            y: p.1,
        }
    }
}

impl Position {
    fn neighbours(&self, boundary: &(Range<isize>, Range<isize>)) -> Vec<Position> {
        vec![
            (self.x as isize - 1, self.y as isize),
            (self.x as isize + 1, self.y as isize),
            (self.x as isize, self.y as isize - 1),
            (self.x as isize, self.y as isize + 1),
        ]
            .into_iter()
            .filter(|(x, y)| boundary.0.contains(x) && boundary.1.contains(y))
            .map(|(x, y)| Position { x: x as usize, y: y as usize })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Rock,
    Garden,
    Start,
}

impl From<char> for NodeType {
    fn from(c: char) -> Self {
        match c {
            '#' => NodeType::Rock,
            '.' => NodeType::Garden,
            'S' => NodeType::Start,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Node {
    position: Position,
    node_type: NodeType,
}

impl Node {
    fn new(x: usize, y: usize, c: char) -> Self {
        Self {
            position: Position { x, y },
            node_type: c.into()
        }
    }

}

#[derive(Debug)]
struct Gardens {
    gardens: Vec<Vec<Node>>,
    boundary: (Range<isize>, Range<isize>),
    start: Position,
}

impl Gardens {
    fn new(gardens: Vec<Vec<Node>>, start: Position) -> Self {
        let boundary = (0..gardens.len() as isize, 0..gardens[0].len() as isize);
        Self {
            gardens,
            boundary,
            start,
        }
    }

    fn walk(&self, steps: usize) -> usize{
        let mut to_check: VecDeque::<(Position, usize)> = VecDeque::new();
        let mut visited: HashMap<Position, usize> = HashMap::new();

        to_check.push_back((self.start, 0));

        while let Some((pos, dist)) = to_check.pop_front() {
            if visited.contains_key(&pos) {
                continue;
            }

            visited.insert(pos, dist);

            let neighbours: Vec<Position> = pos.neighbours(&self.boundary)
                .iter()
                .filter(|pos| self.gardens[pos.x][pos.y].node_type != NodeType::Rock)
                .copied()
                .collect();

            for neighbour in neighbours {
                if !visited.contains_key(&neighbour) {
                    to_check.push_back((neighbour, dist + 1));
                }
            }
        }

        if steps > 64 {
            let even_corners = visited
                .values()
                .filter(|v| **v % 2 == 0 && **v > 65)
                .count();
            let odd_corners = visited
                .values()
                .filter(|v| **v % 2 == 1 && **v > 65)
                .count();

            let n = (steps - ( self.boundary.0.len() / 2)) / self.boundary.0.len();

            let even = if n % 2 == 0 { n * n } else { (n + 1) * (n + 1) };
            let odd = if n % 2 == 0 { (n + 1) * (n + 1) } else { n * n };

            odd * visited.values().filter(|v| **v % 2 == 1).count()
                + even * visited.values().filter(|v| **v % 2 == 0).count()
                - ((n + 1) * odd_corners)
                + (n * even_corners)
        } else {
            match steps % 2 {
                0 => {
                    visited
                        .values()
                        .filter(|v| **v <= steps && **v % 2 == 0)
                        .count()
                }
                _ => {
                    visited
                        .values()
                        .filter(|v| **v <= steps && **v % 2 != 0)
                        .count()
                }
            }
        }
    }
}
