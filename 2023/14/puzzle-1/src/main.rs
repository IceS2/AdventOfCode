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


    let moved_input: Vec<Vec<Node>> = transpose(transpose(input)
        .into_iter()
        .map(|row| {
            let fixed_positions: Vec<usize> = row
                .iter()
                .enumerate()
                .filter(|(_, &node)| node == Node::Fixed)
                .map(|(index, _)| index)
                .collect();

            let splitted_row = row
                .split(|node| node == &Node::Fixed)
                .map(|node_slice| node_slice.to_vec())
                .collect::<Vec<Vec<Node>>>();

            let mut moved_splits: Vec<Vec<Node>> = vec![];

            for split in splitted_row {
                let mut moved_split = split.clone();
                moved_split.sort();
                moved_splits.push(moved_split);
            }

            let moved_rocks: Vec<Node> = moved_splits
                .join(&Node::Fixed);

            moved_rocks
        })
        .collect());

    let input_len: usize = moved_input.len();
    for (row_index, row) in moved_input.iter().enumerate() {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
