use std::collections::HashMap;
use std::fs;
use std::fmt;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let pipe_map: PipeMap = contents.into();

    println!("{}", pipe_map);

    println!("{:?}", pipe_map.get_number_of_enclosed_tiles());
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Node {
    NorthSouthPipe, // │
    EastWestPipe,   // ─

    NorthEastPipe,  // └
    NorthWestPipe,  // ┘

    SouthEastPipe,  // ┌
    SouthWestPipe,  // ┐

    Ground,
    Start,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_to_write = match self {
            Node::NorthSouthPipe => "│",
            Node::EastWestPipe => "─",
            Node::NorthEastPipe => "└",
            Node::NorthWestPipe => "┘",
            Node::SouthEastPipe => "┌",
            Node::SouthWestPipe => "┐",
            Node::Ground => ".",
            Node::Start => "S"
        };
        write!(f, "{}", string_to_write)
    }
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            '|' => Node::NorthSouthPipe,
            '-' => Node::EastWestPipe,
            'L' => Node::NorthEastPipe,
            'J' => Node::NorthWestPipe,
            'F' => Node::SouthEastPipe,
            '7' => Node::SouthWestPipe,
            '.' => Node::Ground,
            'S' => Node::Start,
            _ => unreachable!()
        }
    }
}

impl Node {
    fn can_connect(&self, direction: Direction) -> bool {
        match (self, direction) {
            (Node::NorthSouthPipe, Direction::North) => true,
            (Node::NorthSouthPipe, Direction::South) => true,
            (Node::EastWestPipe, Direction::East) => true,
            (Node::EastWestPipe, Direction::West) => true,
            (Node::NorthEastPipe, Direction::North) => true,
            (Node::NorthEastPipe, Direction::East) => true,
            (Node::NorthWestPipe, Direction::North) => true,
            (Node::NorthWestPipe, Direction::West) => true,
            (Node::SouthEastPipe, Direction::South) => true,
            (Node::SouthEastPipe, Direction::East) => true,
            (Node::SouthWestPipe, Direction::South) => true,
            (Node::SouthWestPipe, Direction::West) => true,
            (Node::Start, _) => true,
            (Node::Ground, _) => false,
            _ => false
        }
    }

    fn get_next_step(&self, from: &Direction) -> (isize, isize, Direction) {
        match (self, from) {
            (Node::NorthSouthPipe, Direction::North) => (1, 0, Direction::North),
            (Node::NorthSouthPipe, Direction::South) => (-1, 0, Direction::South),
            (Node::EastWestPipe, Direction::East) => (0, -1, Direction::East),
            (Node::EastWestPipe, Direction::West) => (0, 1, Direction::West),
            (Node::NorthEastPipe, Direction::North) => (0, 1, Direction::West),
            (Node::NorthEastPipe, Direction::East) => (-1, 0, Direction::South),
            (Node::NorthWestPipe, Direction::North) => (0, -1, Direction::East),
            (Node::NorthWestPipe, Direction::West) => (-1, 0, Direction::South),
            (Node::SouthEastPipe, Direction::South) => (0, 1, Direction::West),
            (Node::SouthEastPipe, Direction::East) => (1, 0, Direction::North),
            (Node::SouthWestPipe, Direction::South) => (0, -1, Direction::East),
            (Node::SouthWestPipe, Direction::West) => (1, 0, Direction::North),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct PipeMap {
    map: Vec<Vec<Node>>,
    start: (usize, usize),
    start_node: Node,
}

impl fmt::Display for PipeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map.iter() {
            for item in row.iter() {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl From<String> for PipeMap {
    fn from(s: String) -> Self {
        let mut map: Vec<Vec<Node>> = Vec::new();
        let mut start: (usize, usize) = (0, 0);

        let lines: Vec<&str> = s.split('\n').collect();

        for (row_index, line) in lines.iter().enumerate() {
            map.push(line.chars().map(|c| c.into()).collect());

            if line.contains('S') {
                start = (row_index, line.find('S').unwrap());
            }
        }

        let row_index_limit = lines.len() - 1;
        let column_index_limit = map[0].len() - 1;

        let start_node: Node = match start {
            // On the Edges
            (0, 0) => Node::SouthEastPipe,
            (0, _c) if _c == column_index_limit => Node::SouthWestPipe,
            (_r, 0) if _r == row_index_limit => Node::NorthEastPipe,
            (_r, _c) if _r == row_index_limit && _c == column_index_limit => Node::NorthWestPipe,

            // On the first or last Row
            (0, column) => {
                let west_node = map[0][column - 1].can_connect(Direction::East);
                let east_node = map[0][column + 1].can_connect(Direction::West);
                let south_node = map[1][column].can_connect(Direction::North);

                match (west_node, east_node, south_node) {
                    (true, true, false) => Node::EastWestPipe,
                    (true, false, true) => Node::SouthWestPipe,
                    (false, true, true) => Node::SouthEastPipe,
                    _ => unreachable!()
                }
            },
            (r, column) if r == row_index_limit => {
                let west_node = map[r][column - 1].can_connect(Direction::East);
                let east_node = map[r][column + 1].can_connect(Direction::West);
                let north_node = map[r - 1][column].can_connect(Direction::South);

                match (west_node, east_node, north_node) {
                    (true, true, false) => Node::EastWestPipe,
                    (true, false, true) => Node::NorthWestPipe,
                    (false, true, true) => Node::NorthEastPipe,
                    _ => unreachable!()
                }
            },

            // On the first or last Column
            (row, 0) => {
                let east_node = map[row][1].can_connect(Direction::West);
                let north_node = map[row - 1][0].can_connect(Direction::South);
                let south_node = map[row + 1][0].can_connect(Direction::North);

                match (east_node, south_node, north_node) {
                    (true, true, false) => Node::SouthEastPipe,
                    (true, false, true) => Node::NorthEastPipe,
                    (false, true, true) => Node::NorthSouthPipe,
                    _ => unreachable!()
                }
            },
            (row, c) if c == column_index_limit => {
                let west_node = map[row][c - 1].can_connect(Direction::East);
                let north_node = map[row - 1][c].can_connect(Direction::South);
                let south_node = map[row + 1][c].can_connect(Direction::North);

                match (west_node, south_node, north_node) {
                    (true, true, false) => Node::SouthWestPipe,
                    (true, false, true) => Node::NorthWestPipe,
                    (false, true, true) => Node::NorthSouthPipe,
                    _ => unreachable!()
                }
            },

            // Elsewhere
            (row, column) => {
                let west_node = map[row][column - 1].can_connect(Direction::East);
                let east_node = map[row][column + 1].can_connect(Direction::West);
                let north_node = map[row - 1][column].can_connect(Direction::South);
                let south_node = map[row + 1][column].can_connect(Direction::North);

                match (west_node, east_node, north_node, south_node) {
                    (true, true, false, false) => Node::EastWestPipe,
                    (true, false, true, false) => Node::NorthWestPipe,
                    (true, false, false, true) => Node::SouthWestPipe,
                    (false, true, true, false) => Node::NorthEastPipe,
                    (false, true, false, true) => Node::SouthEastPipe,
                    (false, false, true, true) => Node::NorthSouthPipe,
                    _ => unreachable!()
                }
            }
        };

        Self {
            map,
            start,
            start_node,
        }
    }
}

impl PipeMap {
    fn walk(&self, start: (usize, usize), from: &Direction) -> (usize, usize, Direction) {
        let next_step = match &self.map[start.0][start.1] {
            Node::Ground => unreachable!(),
            Node::Start => self.start_node.get_next_step(from),
            node => node.get_next_step(from)
        };
        ((start.0 as isize + next_step.0) as usize, (start.1 as isize + next_step.1) as usize, next_step.2)
    }

    fn get_loop(&self) -> HashMap<(usize, usize), &Node> {
        let (mut row, mut col) = self.start;
        let mut from = match &self.start_node {
            Node::NorthSouthPipe => Direction::North,
            Node::EastWestPipe => Direction::West,
            Node::NorthEastPipe => Direction::North,
            Node::NorthWestPipe => Direction::North,
            Node::SouthEastPipe => Direction::South,
            Node::SouthWestPipe => Direction::South,
            _ => unreachable!()
        };

        let mut visited: HashMap<(usize, usize), &Node> = HashMap::from([
            ((row, col), &self.map[row][col])
        ]);

        loop {
            (row, col, from) = self.walk((row, col), &from);

            if let std::collections::hash_map::Entry::Vacant(e) = visited.entry((row, col)) {
                e.insert(&self.map[row][col]);
            } else {
                break;
            }
        }
        visited
    }

    fn get_number_of_enclosed_tiles(&self) -> u32 {
        let mut count: u32 = 0;

        let pipe_loop: HashMap<(usize, usize), &Node> = self.get_loop();

        for (row_index, row) in self.map.iter().enumerate() {
            let mut is_enclosed = false;
            let mut last_bend: Option<Node> = None;

            for (col_index, _) in row.iter().enumerate() {
                if let Some(node) = pipe_loop.get(&(row_index, col_index)) {
                    let node = if node != &&Node::Start { **node } else { self.start_node };

                    if node == Node::NorthSouthPipe ||
                       node == Node::SouthWestPipe && last_bend == Some(Node::NorthEastPipe) ||
                       node == Node::NorthWestPipe && last_bend == Some(Node::SouthEastPipe) {
                        is_enclosed = !is_enclosed;
                    }

                    if [Node::NorthEastPipe, Node::NorthWestPipe, Node::SouthEastPipe, Node::SouthWestPipe].contains(&node) {
                        last_bend = Some(node);
                    }

                    continue;
                }
                if is_enclosed {
                    count += 1;
                }
            }
        }

        count
    }
}
