use std::{fs, ops::Range, collections::{HashSet, HashMap, VecDeque}};

fn main() {
    let input: Vec<Vec<Node>> = fs::read_to_string("test.txt").unwrap()
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
    // for step in [6, 10, 50, 100, 500, 1000, 5000] {
    //     let result = gardens.walk(step);
    //     println!("{:?}", result.len());
    //
    //     let new_result = gardens.count_reachable_garden_plots(step);
    //     println!("{:?}", new_result - new_result2);
    // }
    let mut result = 1;
    let steps = 100;
    for i in (0..steps).step_by(2) {
        result += gardens.count_reachable_garden_plots(steps - i);
    }
    for i in (0..steps).step_by(2) {
        result -= gardens.count_reachable_garden_plots(steps - (i + 1));
    }
    println!("{:#?}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    quadrant: (isize, isize)
}

impl From<(usize, usize)> for Position {
    fn from(p: (usize, usize)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            quadrant: (0, 0)
        }
    }
}

impl Position {
    fn neighbours(&self, boundary: &(Range<isize>, Range<isize>)) -> Vec<Position> {
        let quadrant = self.quadrant;
        vec![
            (self.x as isize - 1, self.y as isize),
            (self.x as isize + 1, self.y as isize),
            (self.x as isize, self.y as isize - 1),
            (self.x as isize, self.y as isize + 1),
        ]
            .into_iter()
            .map(|(x, y)| {
                    if x == -1 {
                        Position {
                            x: (boundary.0.end - 1) as usize,
                            y: y as usize,
                            quadrant: (quadrant.0 - 1, quadrant.1)
                        }
                    } else if x == boundary.0.end {
                        Position {
                            x: 0,
                            y: y as usize,
                            quadrant: (quadrant.0 + 1, quadrant.1)
                        }
                    } else if y == -1 {
                        Position {
                            x: x as usize,
                            y: (boundary.1.end - 1) as usize,
                            quadrant: (quadrant.0, quadrant.1 - 1)
                        }
                    } else if y == boundary.1.end {
                        Position {
                            x: x as usize,
                            y: 0,
                            quadrant: (quadrant.0, quadrant.1 + 1)
                        }
                    } else {
                        Position {
                            x: x as usize,
                            y: y as usize,
                            quadrant
                        }
                    }
            })
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
            position: Position { x, y , quadrant: (0, 0)},
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

    fn walk(&self, steps: usize) -> HashSet<Position> {
        let mut current_positions: HashSet<Position> =  HashSet::from([self.start]);

        for _ in 0..steps {
            let mut new_positions: HashSet<Position> = HashSet::new();
            for cur_pos in &current_positions {
                let neighbours: Vec<Position> = cur_pos.neighbours(&self.boundary)
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

    fn count_reachable_garden_plots(&self, steps: usize) -> usize {
        let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
        queue.push_back((self.start, steps));

        let mut visited = HashSet::new();

        while let Some((position, remaining_steps)) = queue.pop_front() {
            if visited.contains(&position) {
                continue;
            }

            visited.insert(position);

            if remaining_steps == 0 {
                continue;
            }

            let neighbours: Vec<Position> = position.neighbours(&self.boundary)
                .iter()
                .filter(|pos| self.gardens[pos.x][pos.y].node_type != NodeType::Rock)
                .copied()
                .collect();

            for neighbour in neighbours {
                queue.push_back((neighbour, remaining_steps - 1));
            }
        }

        visited.len()
    }
}
