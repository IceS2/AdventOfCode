use std::{fs, collections::HashMap, ops::Range};

fn main() {
    let input: Vec<Vec<Tile>> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .map(|row|
            row
                .chars()
                .filter(|c| c != &'\n')
                .map(|c| c.into())
                .collect::<Vec<Tile>>()
        )
        .filter(|row| !row.is_empty())
        .collect();

    // println!("Input: {:?}", input);

    let mut grid: Grid = input.into();
    // println!("Grid Limits {:?}", grid.grid_limits);
    // println!();

    grid.run_beam();

    println!("{:?}", grid.count_energized_tiles());
}

// PassableByBeam Trait
// -----------------------------------------------------------------------------
trait PassableByBeam {
    fn pass(&self, beam: &Beam, from: &Direction) -> Vec<Beam>;
}

// Direction Enum
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone)]
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

// Splitter Enum
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum Splitter {
    Vertical,    // |
    Horizontal,  // -
}

impl PassableByBeam for Splitter {
    fn pass(&self, beam: &Beam, from: &Direction) -> Vec<Beam> {
        match self {
            Self::Vertical => {
                if [Direction::Up, Direction::Down].contains(from) {
                    vec![beam.go(beam.from.opposite())]
                } else {
                    vec![beam.go(Direction::Up), beam.go(Direction::Down)]
                }
            },
            Self:: Horizontal => {
                if [Direction::Left, Direction::Right].contains(from) {
                    vec![beam.go(beam.from.opposite())]
                } else {
                    vec![beam.go(Direction::Left), beam.go(Direction::Right)]
                }
            }
        }
    }
}

// Mirror Enum
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum Mirror {
    Left, // /
    Right // \
}

impl PassableByBeam for Mirror {
    fn pass(&self, beam: &Beam, from: &Direction) -> Vec<Beam> {
        match self {
            Self::Left => {
                match from {
                    Direction::Up => vec![beam.go(Direction::Left)],
                    Direction::Right => vec![beam.go(Direction::Down)],
                    Direction::Down => vec![beam.go(Direction::Right)],
                    Direction::Left => vec![beam.go(Direction::Up)],
                }
            },
            Self::Right => {
                match from {
                    Direction::Up => vec![beam.go(Direction::Right)],
                    Direction::Right => vec![beam.go(Direction::Up)],
                    Direction::Left => vec![beam.go(Direction::Down)],
                    Direction::Down => vec![beam.go(Direction::Left)],
                }
            }
        }
    }
}

// Tile Enum
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum Tile {
    Empty,     // .
    Splitter(Splitter),
    Mirror(Mirror),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '-' => Self::Splitter(Splitter::Horizontal),
            '|' => Self::Splitter(Splitter::Vertical),
            '/' => Self::Mirror(Mirror::Left),
            '\\' => Self::Mirror(Mirror::Right),
            _ => unreachable!()
        }
    }
}

impl PassableByBeam for Tile {
    fn pass(&self, beam: &Beam, from: &Direction) -> Vec<Beam> {
        match self {
            Self::Empty => vec![beam.go(beam.from.opposite())],
            Self::Splitter(splitter) => splitter.pass(beam, from),
            Self::Mirror(mirror) => mirror.pass(beam, from),
        }
    }
}

// Beam Struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Beam {
    position: (isize, isize),
    from: Direction
}

impl Beam {
    fn new(position: (isize, isize), from: Direction) -> Self {
        Self { position, from }
    }

    fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new((self.position.0 - 1, self.position.1), direction.opposite()),
            Direction::Left => Self::new((self.position.0, self.position.1 - 1), direction.opposite()),
            Direction::Down => Self::new((self.position.0 + 1, self.position.1), direction.opposite()),
            Direction::Right => Self::new((self.position.0, self.position.1 + 1), direction.opposite()),
        }
    }

    fn move_through(&self, tile: &Tile) -> Vec<Self> {
        tile.pass(self, &self.from)
    }
}

impl Default for Beam {
    fn default() -> Self {
        Self::new((0, 0), Direction::Left)
    }
}

// Grid Struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    grid_limits: (Range<isize>, Range<isize>),
    energized_tiles: HashMap<(usize, usize), Vec<Direction>>
}

impl From<Vec<Vec<Tile>>> for Grid {
    fn from(v: Vec<Vec<Tile>>) -> Self {
        let grid_limits = (0..v.len() as isize, 0..v[0].len() as isize);

        Self {
            tiles: v,
            grid_limits,
            energized_tiles: HashMap::new()
        }
    }
}

impl Grid {
    fn run_beam(&mut self) {
        let mut beams = vec![Beam::default()];

        while let Some(beam) = beams.pop() {
            // println!("Energized Tiles: {:?}", self.energized_tiles);
            // println!("Beam: {:?}", beam);

            let beam_position = (beam.position.0 as usize, beam.position.1 as usize);

            if self.energized_tiles.get(&beam_position).is_some_and(|b| b.contains(&beam.from)) {
                continue;
            }

            self.energized_tiles.entry(beam_position).and_modify(|e|  e.push(beam.from.clone())).or_insert(vec![beam.from.clone()]);
            let next_beams = beam.move_through(&self.tiles[beam_position.0][beam_position.1]);
            for next_beam in next_beams {
                let next_beam_position = (&next_beam.position.0, &next_beam.position.1);

                if self.is_valid_position(next_beam_position) {
                    // println!("Next Beam: {:?}", next_beam);
                    beams.push(next_beam);
                }
            }
            // println!();
        }
    }

    fn count_energized_tiles(&self) -> usize {
        self.energized_tiles.len()
    }

    fn is_valid_position(&self, position: (&isize, &isize)) -> bool {
        self.grid_limits.0.contains(position.0) && self.grid_limits.1.contains(position.1)
    }
}
