use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::ops::Range;

fn main() {
    let map: Map = fs::read_to_string("input.txt")
        .unwrap()
        .as_str()
        .into();

    println!("{:#}", map);

    let result = map.walk_longest_path();


    for row in &map.map {
        for tile in row {
            if result.0.contains(&tile.position) {
                print!("O");
            } else {
                match &tile.tile_type {
                    TileType::Path => print!("."),
                    TileType::Forest => print!("#"),
                    TileType::Slope(slope) => match slope {
                        Slope::Up => print!("^"),
                        Slope::Left => print!("<"),
                        Slope::Down => print!("v"),
                        Slope::Right => print!(">"),
                    }
                }
            }
        }
        println!();
    }

    println!();
    println!("Steps: {:?}", result.1);
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

impl Slope {
    fn neighbours(&self, position: &Position, boundary: &(Range<usize>, Range<usize>)) -> Vec<Position> {
        match self {
            Slope::Up => {
                let neighbour = Position { x: position.x - 1, y: position.y };

                if  boundary.0.contains(&neighbour.x) &&
                    boundary.1.contains(&neighbour.y) {
                        vec![neighbour]
                } else {
                    vec![]
                }
            },
            Slope::Left => {
                let neighbour = Position { x: position.x, y: position.y - 1 };

                if  boundary.0.contains(&neighbour.x) &&
                    boundary.1.contains(&neighbour.y) {
                        vec![neighbour]
                } else {
                    vec![]
                }
            },
            Slope::Down => {
                let neighbour = Position { x: position.x + 1, y: position.y };

                if  boundary.0.contains(&neighbour.x) &&
                    boundary.1.contains(&neighbour.y) {
                        vec![neighbour]
                } else {
                    vec![]
                }
            },
            Slope::Right => {
                let neighbour = Position { x: position.x, y: position.y + 1 };

                if  boundary.0.contains(&neighbour.x) &&
                    boundary.1.contains(&neighbour.y) {
                        vec![neighbour]
                } else {
                    vec![]
                }
            }
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

// State
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq)]
struct State {
    position: Position,
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

// Map
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    boundary: (Range<usize>, Range<usize>),
}

impl Map {
    fn walk_longest_path(&self) -> (Vec<Position>, usize) {
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

        let mut distance_map: HashMap<Position, (usize, Option<Position>)> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut result: (Vec<Position>, usize) = (vec![], 0);

        distance_map.insert(start, (0, None));
        heap.push(State { position: start, distance: 0 });

        while let Some(State { position, distance }) = heap.pop() {
            if position == end {
                let mut path = vec![];
                let mut current = position;

                while let Some(previous) = distance_map[&current].1 {
                    path.push(previous);
                    current = previous;
                }

                path = path.into_iter().rev().collect();
                path.push(position);

                if distance > result.1 {
                    result = (path, distance);
                }
            }

            if distance_map.contains_key(&position) && distance < distance_map[&position].0 {
                continue;
            }

            let neighbours: Vec<Position> = match self.map[position.x][position.y].tile_type {
                    TileType::Slope(slope) => {
                        slope.neighbours(&position, &self.boundary)
                    },
                    _ => {
                        position.neighbours(&self.boundary)
                    }
                }
                .iter()
                .filter(|p| self.map[p.x][p.y].tile_type != TileType::Forest)
                .cloned()
                .collect();


            for neighbour in neighbours {
                let mut path = vec![];
                let mut current = position;

                while let Some(previous) = distance_map[&current].1 {
                    path.push(previous);
                    current = previous;
                }

                if path.contains(&neighbour) {
                    continue;
                };

                let next = State {
                    position: neighbour,
                    distance: distance + 1,
                };

                if !distance_map.contains_key(&neighbour) || next.distance > distance_map[&neighbour].0 {
                    distance_map.insert(neighbour, (next.distance, Some(position)));
                    heap.push(next);
                }
            }
        }
        result
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
