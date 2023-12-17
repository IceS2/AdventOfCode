use std::{fs, collections::{HashMap, BinaryHeap}, ops::Range, cmp::Ordering};

fn main() {
    let input: Vec<Vec<Node>> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .enumerate()
        .map(|(row_index, row)| {
            row
                .chars()
                .enumerate()
                .map(|(c_index, c)| (row_index, c_index, c).into())
                .collect::<Vec<Node>>()
        })
        .filter(|row| !row.is_empty())
        .collect();

    let number_of_rows: usize = input.len();
    let number_of_cols: usize = input[0].len();

    let mut map: Map = Map::new(HashMap::new(), (0..number_of_rows, 0..number_of_cols));

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let id = (row_index * number_of_rows) + col_index;
            map.insert_node(id, *col);
        }
    }

    let shortest_path = map.find_path(0, (number_of_rows * number_of_cols) - 1);

    // println!("Shortest Path: {:?}", shortest_path);

    let mut heat_lost_per_node: Vec<(usize, usize)> = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let id = (row_index * number_of_rows) + col_index;

            if shortest_path.0.contains(&id) {
                let id_position = shortest_path.0.iter().position(|&s| s == id).unwrap();
                heat_lost_per_node.push((id, col.heat_loss));

                if id_position > 0 {
                    let current = shortest_path.0[id_position] as isize;
                    let previous = shortest_path.0[id_position - 1] as isize;

                    if current - previous == 1 {
                        print!(">");
                    } else if current - previous == -1 {
                        print!("<");
                    } else if current - previous == number_of_rows as isize {
                        print!("v");
                    } else if current - previous == -(number_of_rows as isize) {
                        print!("^");
                    }
                } else {
                    print!("#");
                }
            } else {
                print!("{}", col.heat_loss);
            }
        }
        println!();
    }

    println!("Total Heat Lost: {:?}", shortest_path.1);
}

// Direction
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

// Node
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
struct Node {
    position: (usize, usize),
    heat_loss: usize,
}

impl From<(usize, usize, char)> for Node {
    fn from(c: (usize, usize, char)) -> Self {
        let position = (c.0, c.1);
        let heat_loss = c.2.to_digit(10).unwrap() as usize;

        Self {
            position,
            heat_loss
        }
    }
}

impl Node {
    fn neighbours_with_boundaries(
        &self,
        grid_boundaries: &(Range<usize>, Range<usize>),
        current_direction_tracker: &(Direction, usize),
        direction_boundary: &Range<usize>
    ) -> Vec<(Direction, (usize, usize))> {
        let mut neighbours: Vec<(Direction, (usize, usize))> = vec![];

        let x = self.position.0;
        let y = self.position.1;

        if x != 0 && current_direction_tracker.0.opposite() != Direction::Up {
            if direction_boundary.contains(&current_direction_tracker.1) || (current_direction_tracker.0 == Direction::Up && current_direction_tracker.1 < direction_boundary.start) {
                neighbours.push((Direction::Up, (x - 1, y)));
            }
        }

        if x != grid_boundaries.0.end - 1 && current_direction_tracker.0.opposite() != Direction::Down {
            if direction_boundary.contains(&current_direction_tracker.1) || (current_direction_tracker.0 == Direction::Down && current_direction_tracker.1 < direction_boundary.start) {
                neighbours.push((Direction::Down, (x + 1, y)));
            }
        }

        if y != 0 && current_direction_tracker.0.opposite() != Direction::Left {
            if direction_boundary.contains(&current_direction_tracker.1) || (current_direction_tracker.0 == Direction::Left && current_direction_tracker.1 < direction_boundary.start) {
                neighbours.push((Direction::Left, (x, y - 1)));
            }
        }

        if y != grid_boundaries.1.end - 1 && current_direction_tracker.0.opposite() != Direction::Right {
            if direction_boundary.contains(&current_direction_tracker.1) || (current_direction_tracker.0 == Direction::Right && current_direction_tracker.1 < direction_boundary.start) {
                neighbours.push((Direction::Right, (x, y + 1)));
            }
        }

        neighbours
    }
}

// State
// -----------------------------------------------------------------------------
#[derive(Debug, Eq, PartialEq)]
struct State {
    node_id: usize,
    heat_lost: usize,
    direction_tracker: (Direction, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_lost.cmp(&self.heat_lost)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// DistKey
// -----------------------------------------------------------------------------
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct DistKey {
    node_id: usize,
    direction_tracker: (Direction, usize),
}

// Map
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Map {
    nodes: HashMap<usize, Node>,
    boundaries: (Range<usize>, Range<usize>)
}

impl Map {
    fn new(nodes: HashMap<usize, Node>, boundaries: (Range<usize>, Range<usize>)) -> Self {
        Self { nodes, boundaries }
    }

    fn insert_node(&mut self, id: usize, node: Node) {
        self.nodes.insert(id, node);
    }

    fn find_path(&self, start: usize, end: usize) -> (Vec<usize>, usize) {
        let mut dist: HashMap<DistKey, (usize, Option<DistKey>)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(DistKey {
            node_id: start,
            direction_tracker: (Direction::Right, 0)
        }, (0, None));
        dist.insert(DistKey {
            node_id: start,
            direction_tracker: (Direction::Down, 0)
        }, (0, None));

        heap.push(State {
            node_id: start,
            heat_lost: 0,
            direction_tracker: (Direction::Right, 0)
        });

        heap.push(State {
            node_id: start,
            heat_lost: 0,
            direction_tracker: (Direction::Down, 0)
        });

        while let Some(State { node_id, heat_lost, direction_tracker }) = heap.pop() {
            let dist_key = DistKey { node_id, direction_tracker };
            // println!("Dist Key: {:?}", dist_key);

            if node_id == end && direction_tracker.1 >= 4 {
                let mut path = vec![];
                let mut current = dist_key;

                while let Some(previous) = dist[&current].1 {
                    path.push(previous.node_id);
                    current = previous;
                }

                path = path.into_iter().rev().collect();
                path.push(node_id);

                return (path, heat_lost);
            }


            if dist.contains_key(&dist_key) && heat_lost > dist[&dist_key].0 {
                // println!("\tSkipped - Heat Lost Higher than Already Found");
                // println!("{}", "-".repeat(80));
                continue;
            }

            for (direction, neighbour) in self.nodes.get(&node_id).unwrap().neighbours_with_boundaries(&self.boundaries, &direction_tracker, &(4..10)) {
                // println!("\tNeighbour: {:?}", neighbour);

                let neighbour_id = (neighbour.0 * self.boundaries.0.end) + neighbour.1;

                let neighbour_direction_tracker = if direction_tracker.0 == direction {
                    (direction_tracker.0, direction_tracker.1 + 1)
                } else {
                    (direction, 1)
                };

                let next = State {
                    node_id: neighbour_id,
                    heat_lost: heat_lost + self.nodes.get(&neighbour_id).unwrap().heat_loss,
                    direction_tracker: neighbour_direction_tracker
                };

                let next_dist_key = DistKey {
                    node_id: next.node_id,
                    direction_tracker: next.direction_tracker
                };

                if !dist.contains_key(&next_dist_key) || next.heat_lost < dist[&next_dist_key].0 {
                    // println!("\tPushing Next: {:?}", next);
                    dist.insert(next_dist_key, (next.heat_lost, Some(dist_key)));
                    heap.push(next);
                }
                // println!("{}", "-".repeat(80));
            }
        }
        (vec![], 0)
    }
}
