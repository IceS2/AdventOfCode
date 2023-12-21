use std::{fs, ops::Range, collections::HashSet};

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
    let result = gardens.walk(64);
    println!("{:?}", result.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize
}

impl From<(usize, usize)> for Position {
    fn from(p: (usize, usize)) -> Self {
        Self {
            x: p.0,
            y: p.1
        }
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

    fn neighbours(&self, boundary: &(Range<isize>, Range<isize>)) -> Vec<Position> {
        vec![
            (self.position.x as isize - 1, self.position.y as isize),
            (self.position.x as isize + 1, self.position.y as isize),
            (self.position.x as isize, self.position.y as isize - 1),
            (self.position.x as isize, self.position.y as isize + 1),
        ]
            .into_iter()
            .filter(|(x, y)| boundary.0.contains(x) && boundary.1.contains(y))
            .map(|(x, y)| Position { x: x as usize, y: y as usize })
            .collect()
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

    fn walk(&self, steps: usize) -> HashSet<Position> {
        let mut current_positions: HashSet<Position> =  HashSet::from([self.start]);

        for _ in 0..steps {
            let mut new_positions: HashSet<Position> = HashSet::new();
            for cur_pos in &current_positions {
                let neighbours: Vec<Position> = self.gardens[cur_pos.x][cur_pos.y].neighbours(&self.boundary)
                    .iter()
                    .filter(|pos| self.gardens[pos.x][pos.y].node_type != NodeType::Rock)
                    .copied()
                    .collect();
                for neighbour in neighbours {
                    new_positions.insert(neighbour);
                }
            }
            current_positions = new_positions;

        }
        current_positions
    }
}
