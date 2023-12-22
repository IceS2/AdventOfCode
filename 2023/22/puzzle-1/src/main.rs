use std::{fs, cell::RefCell, collections::{HashMap, HashSet}, ops::Range};

fn main() {
    let input: Vec<Block> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line
            .split('~')
            .map(|part| part
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
            )
            .collect::<Vec<Vec<usize>>>()
            .into_iter()
            .map(|values| (values[0], values[1], values[2]))
            .collect::<Vec<(usize, usize, usize)>>()
        )
        .collect::<Vec<Vec<(usize, usize, usize)>>>()
        .into_iter()
        .map(|v| (v[0], v[1]).into())
        .collect();

    let sand_blocks: SandBlocks = SandBlocks::new(input);

    sand_blocks.sort();
    sand_blocks.name_blocks();

    // println!("{:?}", sand_blocks);

    // println!("{:?}", sand_blocks.get_boundary_coordinates());

    sand_blocks.fall();

    // println!("{:?}", sand_blocks);

    let supports = sand_blocks.blocks_supports();
    let is_supported_by = sand_blocks.blocks_supported_by();

    println!("Supports: {:?}", supports.get(&477).unwrap());
    println!("{}", "-".repeat(40));
    println!("Is Supported: {:?}", is_supported_by.get(&477).unwrap());

    let mut safe_to_remove: HashSet<usize> = HashSet::new();

    for (block, supported_blocks) in supports {
        if supported_blocks.is_empty() {
            safe_to_remove.insert(block);
        }

        if supported_blocks.iter().all(|b| is_supported_by.get(b).unwrap().len() > 1) {
            safe_to_remove.insert(block);
        }
    }

    // println!("{:?}", safe_to_remove);
    println!("{:?}", safe_to_remove.len());
}

#[derive(Debug)]
enum Coordinate {
    X,
    Y,
    Z
}

impl From<char> for Coordinate {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::X,
            'y' => Self::Y,
            'z' => Self::Z,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl From<(usize, usize, usize)> for Position {
    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Vertical,
    HorizontalX,
    HorizontalY,
}

impl From<(&Position, &Position)> for Orientation {
    fn from((start, end): (&Position, &Position)) -> Self {
        if start.x == end.x && start.y == end.y {
            Self::Vertical
        } else if start.z == end.z && start.y == end.y {
            Self::HorizontalX
        } else if start.z == end.z && start.x == end.x {
            Self::HorizontalY
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    start: Position,
    end: Position,
    orientation: Orientation
}

impl From<((usize, usize, usize), (usize, usize, usize))> for Block {
    fn from((start, end): ((usize, usize, usize), (usize, usize, usize))) -> Self {
        let start: Position = start.into();
        let end: Position = end.into();
        let orientation: Orientation = (&start, &end).into();

        Self {
            id: 0,
            start,
            end,
            orientation,
        }
    }
}

impl Block {
    fn min(&self, coordinate: &Coordinate) -> usize {
        match coordinate {
            Coordinate::X => self.start.x.min(self.end.x),
            Coordinate::Y => self.start.y.min(self.end.y),
            Coordinate::Z => self.start.z.min(self.end.z),
        }
    }

    fn max(&self, coordinate: &Coordinate) -> usize {
        match coordinate {
            Coordinate::X => self.start.x.max(self.end.x),
            Coordinate::Y => self.start.y.max(self.end.y),
            Coordinate::Z => self.start.z.max(self.end.z),
        }
    }

    fn range(&self, coordinate: &Coordinate) -> Range<usize> {
        match coordinate {
            Coordinate::X => self.min(coordinate)..self.max(coordinate) + 1,
            Coordinate::Y => self.min(coordinate)..self.max(coordinate) + 1,
            Coordinate::Z => self.min(coordinate)..self.max(coordinate) + 1,
        }
    }

    fn positions_xy(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![];
        for x in self.range(&Coordinate::X) {
            for y in self.range(&Coordinate::Y) {
            positions.push(Position { x, y, z: 0 });
            }
        }
        positions
    }
}

#[derive(Debug)]
struct SandBlocks {
    blocks: RefCell<Vec<Block>>,
    blocks_after_falling: RefCell<HashMap<usize, Vec<Vec<usize>>>>,
}

impl SandBlocks {
    fn new(blocks: Vec<Block>) -> Self {
        Self {
            blocks: RefCell::new(blocks),
            blocks_after_falling: RefCell::new(HashMap::new()),
        }
    }

    fn sort(&self) {
        self.blocks.borrow_mut().sort_by(|a, b| {
            let a_z = a.start.z.min(a.end.z);
            let b_z = b.start.z.min(b.end.z);

            a_z.cmp(&b_z)
        })
    }

    fn name_blocks(&self) {
        let mut start: usize = 1;

        let mut named_blocks: Vec<Block> = vec![];

        for block in self.blocks.borrow().iter() {
            named_blocks.push(Block { id: start, start: block.start, end: block.end, orientation: block.orientation });
            start += 1;
        }

        *self.blocks.borrow_mut() = named_blocks;
    }

    fn get_boundary_coordinates(&self) -> (Position, Position) {
        let start_x = self.blocks.borrow().iter().map(|block| block.min(&Coordinate::X)).min().unwrap();
        let start_y = self.blocks.borrow().iter().map(|block| block.min(&Coordinate::Y)).min().unwrap();

        let end_x = self.blocks.borrow().iter().map(|block| block.max(&Coordinate::X)).max().unwrap();
        let end_y = self.blocks.borrow().iter().map(|block| block.max(&Coordinate::Y)).max().unwrap();
        let end_z = self.blocks.borrow().iter().map(|block| block.max(&Coordinate::Z)).max().unwrap();

        (Position::from((start_x, start_y, 1)), Position::from((end_x, end_y, end_z)))
    }

    fn fall(&self) {
        let (start, end) = self.get_boundary_coordinates();

        let x_range = start.x..end.x + 1;
        let y_range = start.y..end.y + 1;
        let z_range = start.z..end.z + 1;

        let mut map = self.blocks_after_falling.borrow_mut();
        let mut new_blocks: Vec<Block> = vec![];

        for z in z_range.clone() {
            map.insert(z, vec![vec![0; y_range.len()]; x_range.len()]);
        }

        for block in self.blocks.borrow().iter() {
            // println!("{:?}", block);
            let mut z_with_space = z_range.end - 1;

            for z in z_range.clone().rev() {
                // println!("Z: {:?}", z);
                // println!("{:?}", map.get(&z).unwrap());
                let mut has_space = true;

                for x in block.range(&Coordinate::X) {
                    for y in block.range(&Coordinate::Y) {
                        // println!("{} {}", x, y);
                        // println!("{:?}", map.get(&(z)).unwrap()[x][y]);
                        if map.get(&(z)).unwrap()[x][y] != 0 {
                            has_space = false;
                            break;
                        };
                    }
                    if !has_space {
                        break;
                    }
                }

                if has_space {
                    // println!("Has Space: {:?}", z);
                    z_with_space = z;
                } else {
                    break;
                }
            }

            new_blocks.push(Block {
                id: block.id,
                start: Position {
                    x: block.start.x,
                    y: block.start.y,
                    z: z_with_space,
                },
                end: Position {
                    x: block.end.x,
                    y: block.end.y,
                    z: z_with_space + block.range(&Coordinate::Z).len() - 1
                },
                orientation: block.orientation
            });

            // println!("Block: {:?}", block);
            for z in z_with_space..(block.range(&Coordinate::Z).len() + z_with_space) {
                // println!("Z: {:?}", z);
                for x in block.range(&Coordinate::X) {
                    for y in block.range(&Coordinate::Y) {
                        map.get_mut(&z).unwrap()[x][y] = block.id;
                    }
                }
            }
        }

        *self.blocks.borrow_mut() = new_blocks;


        for z in z_range {
            let row = map.get(&z).unwrap();

            if row.iter().any(|col| col.iter().any(|c| *c != 0)) {
                println!("Z: {:?}", z);
                println!("{}", "-".repeat(40));
                for col in row.iter() {
                    for c in col {
                        let w: usize = 4;
                        print!("{c:>w$} ");
                    }
                    println!();
                }
                println!();
            }
        }
    }

    fn blocks_supports(&self) -> HashMap<usize, Vec<usize>> {
        let mut blocks: HashMap<usize, Vec<usize>> = HashMap::new();

        for block in self.blocks.borrow().iter() {
            let supports: Vec<usize> = self.blocks.borrow()
                .iter()
                .filter(|other| block.max(&Coordinate::Z) == other.min(&Coordinate::Z) - 1)
                .filter(|other|
                    block.positions_xy().iter().any(|p| other.positions_xy().contains(p))
                )
                .map(|other| other.id)
                .collect();

            blocks.insert(block.id, supports);
        }

        blocks
    }

    fn blocks_supported_by(&self) -> HashMap<usize, Vec<usize>> {
        let mut blocks: HashMap<usize, Vec<usize>> = HashMap::new();

        for block in self.blocks.borrow().iter() {
            let supports: Vec<usize> = self.blocks.borrow()
                .iter()
                .filter(|other| other.max(&Coordinate::Z) == block.min(&Coordinate::Z) - 1)
                .filter(|other|
                    other.positions_xy().iter().any(|p| block.positions_xy().contains(p))
                )
                .map(|other| other.id)
                .collect();

            blocks.insert(block.id, supports);
        }

        blocks
    }
}
