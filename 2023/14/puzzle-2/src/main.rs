use std::collections::HashMap;
use std::fs;
use std::cmp::Ordering;

fn main() {
    let input: Vec<Vec<Node>> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .map(|row|
            row
                .chars()
                .map(|node| node.into())
                .collect::<Vec<Node>>()
        )
        .filter(|row| !row.is_empty())
        .collect();

    let mut sum: usize = 0;

    let mut reflector: ParabolicReflector = ParabolicReflector {
        nodes: input,
        memo: HashMap::new(),
        visited_states: HashMap::new(),
        terminal: false
    };

    for i in 0..1000000000 {
    // for i in 0..3 {
        if reflector.terminal {
            println!("Iterations: {:?}", i);
            break;
        }
        reflector.run_cycle();
        // println!("{:?}", reflector.nodes);
    }

    // let moved_input: Vec<Vec<Node>> = transpose(ParabolicReflector::sort_nodes(transpose(input), SortKind::Ascending));

    let input_len: usize = reflector.nodes.len();
    for (row_index, row) in reflector.nodes.iter().enumerate() {
        sum += row.iter().filter(|&node| node == &Node::Rock).count() * (input_len - row_index)
    }

    println!("Sum {:?}", sum);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let col_len = v[0].len();

    let mut inner_vec_as_iter: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..col_len)
        .map(|_| {
            inner_vec_as_iter
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Node {
    Rock,
    Fixed,
    Empty
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Fixed,
            'O' => Self::Rock,
            _ => unreachable!()
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Empty) => Ordering::Less,
            (Self::Empty, Self::Rock) => Ordering::Greater,
            _ => Ordering::Equal
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum SortKind {
    Ascending,
    Descending
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
struct ParabolicReflector {
    nodes: Vec<Vec<Node>>,
    memo: HashMap<(Vec<Vec<Node>>, Direction), Vec<Vec<Node>>>,
    visited_states: HashMap<Vec<Vec<Node>>, bool>,
    terminal: bool
}

impl ParabolicReflector {
    fn run_cycle(&mut self) {
        if self.visited_states.get(&self.nodes.clone()).is_some() {
            self.terminal = true;
            return;
        } else {
            self.visited_states.insert(self.nodes.clone(), true);
        }

        self.move_to(Direction::North);
        self.move_to(Direction::West);
        self.move_to(Direction::South);
        self.move_to(Direction::East);
    }

    fn move_to(&mut self, direction: Direction) {
        let initial_nodes = self.nodes.clone();

        if let Some(result) = self.memo.get(&(initial_nodes.clone(), direction)) {
            self.nodes = result.clone();
        } else {
            self.nodes = match &direction {
                    Direction::North => {
                        transpose(
                            ParabolicReflector::sort_nodes(
                                transpose(self.nodes.clone()),
                                SortKind::Ascending
                            )
                        )
                    },
                    Direction::South => {
                        transpose(
                            ParabolicReflector::sort_nodes(
                                transpose(self.nodes.clone()),
                                SortKind::Descending
                            )
                        )
                    },
                    Direction::East => {
                        ParabolicReflector::sort_nodes(self.nodes.clone(), SortKind::Descending)
                    },
                    Direction::West => {
                        ParabolicReflector::sort_nodes(self.nodes.clone(), SortKind::Ascending)
                    },
                };

            self.memo.insert((initial_nodes, direction), self.nodes.clone());
        }
    }

    fn sort_nodes(nodes: Vec<Vec<Node>>, kind: SortKind) -> Vec<Vec<Node>> {
        nodes
            .into_iter()
            .map(|row| {
                let splitted_row = row
                    .split(|node| node == &Node::Fixed)
                    .map(|node_slice| node_slice.to_vec())
                    .collect::<Vec<Vec<Node>>>();

                let mut moved_splits: Vec<Vec<Node>> = vec![];

                for split in splitted_row {
                    let mut moved_split = split.clone();

                    match kind {
                        SortKind::Ascending => moved_split.sort(),
                        SortKind::Descending => moved_split.sort_by(|a, b| b.cmp(a))
                    }

                    moved_splits.push(moved_split);
                }

                let moved_rocks: Vec<Node> = moved_splits
                    .join(&Node::Fixed);

                moved_rocks
            })
            .collect()
    }
}
