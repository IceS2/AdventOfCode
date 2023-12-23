use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::ops::Range;

fn main() {
    let map: Map = fs::read_to_string("test.txt")
        .unwrap()
        .as_str()
        .into();

    let (start, end) = map.get_start_and_end();

    let graph: Graph = map.into();
    let result = graph.walk_longest_path(start, end);
    println!("{:?}", result);
}

// Slope
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Slope {
    Up,
    Left,
    Down,
    Right,
}

impl From<char> for Slope {
    fn from(c: char) -> Self {
        match c {
            '^' => Slope::Up,
            '<' => Slope::Left,
            'v' => Slope::Down,
            '>' => Slope::Right,
            _ => unreachable!()
        }
    }
}

// TileType
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TileType {
    Path,
    Forest,
    Slope(Slope)
}

impl From<char> for TileType {
    fn from(c: char) -> Self {
        match c {
            '.' => TileType::Path,
            '#' => TileType::Forest,
            '>' | '<' | '^' | 'v' => TileType::Slope(c.into()),
            _ => unreachable!()
        }
    }
}

// Position
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position { x, y }
    }
}

impl Position {
    fn neighbours(&self, boundary: &(Range<usize>, Range<usize>)) -> Vec<Position> {
        let coordinates = (self.x as isize, self.y as isize);

        [
            (coordinates.0 - 1, coordinates.1),
            (coordinates.0 + 1, coordinates.1),
            (coordinates.0, coordinates.1 - 1),
            (coordinates.0, coordinates.1 + 1),
        ]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| boundary.0.contains(x) && boundary.1.contains(y))
            .map(|(x, y)| Position { x, y })
            .collect()
    }
}

// Tile
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Tile {
    position: Position,
    tile_type: TileType,
}

impl From<((usize, usize), char)> for Tile {
    fn from(((x, y), c): ((usize, usize), char)) -> Self {
        Tile {
            position: (x, y).into(),
            tile_type: c.into(),
        }
    }
}

// Map
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    boundary: (Range<usize>, Range<usize>),
}

impl Map {
    fn get_start_and_end(&self) -> (Position, Position) {
        let start = self.map[0]
            .iter()
            .find(|t| t.tile_type == TileType::Path)
            .unwrap()
            .position;
        let end = self.map[self.boundary.0.end - 1]
            .iter()
            .find(|t| t.tile_type == TileType::Path)
            .unwrap()
            .position;

        (start, end)
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let map: Vec<Vec<Tile>> = s
            .lines()
            .enumerate()
            .map(|(r_index, line)|
                line
                    .chars()
                    .enumerate()
                    .map(|(c_index, c)|
                        ((r_index, c_index), c)
                            .into())
                    .collect()
            )
            .collect();

        let boundary: (Range<usize>, Range<usize>) = (0..map.len(), 0..map[0].len());

        Map { map , boundary }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for tile in row {
                match &tile.tile_type {
                    TileType::Path => write!(f, ".")?,
                    TileType::Forest => write!(f, "#")?,
                    TileType::Slope(slope) => match slope {
                        Slope::Up => write!(f, "^")?,
                        Slope::Left => write!(f, "<")?,
                        Slope::Down => write!(f, "v")?,
                        Slope::Right => write!(f, ">")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Edge
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Edge {
    p1: Position,
    p2: Position,
    distance: usize
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

// Node
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Node {
    position: Position,
    edges: HashMap<Position, usize>
}

// State
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    position: Position,
    seen: Vec<Position>,
    distance: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}


// Graph
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Graph {
    nodes: HashMap<Position, Node>
}

impl From<Vec<Edge>> for Graph {
    fn from(v: Vec<Edge>) -> Self {
        let mut nodes: HashMap<Position, Node> = HashMap::new();

        for edge in v {
            if edge.distance == 0 { continue; }
            nodes
                .entry(edge.p1)
                .and_modify(|e| { e.edges.insert(edge.p2, edge.distance); })
                .or_insert(Node { position: edge.p1, edges: HashMap::from([(edge.p2, edge.distance)]) });
        }

        Graph { nodes }
    }
}

impl From<Map> for Graph {
    fn from(m: Map) -> Self {
        let start = m.map[0]
            .iter()
            .find(|t| t.tile_type == TileType::Path)
            .unwrap()
            .position;

        let mut edges: Vec<Edge> = Vec::new();
        let mut visited: Vec<(Position, Position)> = vec![];
        let mut heap: BinaryHeap<Edge> = BinaryHeap::new();

        edges.push(Edge { p1: start, p2: start, distance: 0 });
        heap.push(Edge { p1: start, p2: start, distance: 0 });

        while let Some(Edge { p1, p2, distance }) = heap.pop() {
            let mut neighbours: Vec<Position> = p2.neighbours(&m.boundary)
                .iter()
                .filter(|p| m.map[p.x][p.y].tile_type != TileType::Forest)
                .filter(|p| !visited.contains(&(**p, p2)))
                .cloned()
                .collect();

            let mut next_edge: Edge = Edge { p1, p2, distance };

            while neighbours.len() == 1 {
                let last_position = next_edge.p2;
                next_edge.p2 = neighbours[0];
                next_edge.distance += 1;
                neighbours = next_edge.p2.neighbours(&m.boundary)
                    .iter()
                    .filter(|p| m.map[p.x][p.y].tile_type != TileType::Forest)
                    .filter(|p| **p != last_position)
                    .cloned()
                    .collect();
            }

            edges.push(next_edge);


            for neighbour in neighbours {
                if !visited.contains(&(next_edge.p2, neighbour)) {
                    let next = Edge {
                        p1: next_edge.p2,
                        p2: neighbour,
                        distance: 1,
                    };
                    heap.push(next);
                    visited.push((next_edge.p2, neighbour));
                }
            }
        }
        edges.into()
    }
}

impl Graph {
    fn walk_longest_path(&self, start: Position, end: Position) -> (Vec<Position>, usize) {
        let mut distance_map: HashMap<Position, (usize, Option<Position>)> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut result: (Vec<Position>, usize) = (vec![], 0);

        distance_map.insert(start, (0, None));
        heap.push(State { position: start, seen: vec![], distance: 0 });

        while let Some(State { position, seen, distance }) = heap.pop() {
            if position == end {
                if distance > result.1 {
                    let mut new_seen = seen.clone();
                    new_seen.push(position);

                    result = (new_seen, distance);
                }
                continue;
            }

            for (neighbour, neighbour_distance) in &self.nodes.get(&position).unwrap().edges {
                let mut next_seen = seen.clone();
                next_seen.push(position);

                let next = State {
                    position: *neighbour,
                    seen: next_seen,
                    distance: distance + neighbour_distance,
                };


                if !seen.contains(neighbour) {
                    heap.push(next);
                }
            }
        }
        result
        }
}
