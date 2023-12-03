use std::{path::Path, fs::File, io::{self, BufRead}, collections::HashSet};

// --------------------
// Entrypoint
// --------------------
fn main() {
    let input = read_input("input.txt").unwrap();
    let mut schematic: Vec<Vec<NodeType>> = vec![];

    for line in input {
        let schematic_row: Vec<NodeType> = line.unwrap().chars()
            .map(|c| c.into())
            .collect();
        schematic.push(schematic_row)
    }

    let engine_schematic = EngineSchematic { schematic };
    println!("{:?}", engine_schematic.sum_part_numbers());
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// --------------------
// NodeType - Represents what we have on each node of the schematic
// --------------------
#[derive(Debug)]
enum NodeType {
    Number(char),
    Symbol,
    Empty
}

impl From<char> for NodeType {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => NodeType::Number(c),
            '.' => NodeType::Empty,
            _ => NodeType::Symbol
        }
    }
}


// --------------------
// IntelligentCoordinates - Able to add neighbours while filtering based on constraints
// --------------------
#[derive(Debug)]
struct IntelligentCoordinates {
    coordinates: HashSet<Vec<usize>>
}

impl IntelligentCoordinates {
    fn new() -> Self{
        Self {
            coordinates: HashSet::new()
        }
    }

    fn add_neighbours(&mut self, coordinate: Vec<usize>, constraints: Vec<usize>) {
        let coordinates_to_insert: Vec<Vec<usize>> = vec![
            vec![coordinate[0] as i32 - 1, coordinate[1] as i32 - 1],
            vec![coordinate[0] as i32 - 1, coordinate[1] as i32],
            vec![coordinate[0] as i32 - 1, coordinate[1] as i32 + 1],
            vec![coordinate[0] as i32, coordinate[1] as i32 - 1],
            vec![coordinate[0] as i32, coordinate[1] as i32 + 1],
            vec![coordinate[0] as i32 + 1, coordinate[1] as i32 - 1],
            vec![coordinate[0] as i32 + 1, coordinate[1] as i32],
            vec![coordinate[0] as i32 + 1, coordinate[1] as i32 + 1],
        ]
        .into_iter()
        .filter(|c| c[0] >= 0 && c[0] < constraints[0] as i32 && c[1] >= 0 && c[1] < constraints[1] as i32)
        .map(|c| vec![c[0] as usize, c[1] as usize])
        .collect();

        for c in coordinates_to_insert {
            self.coordinates.insert(c);
        }
    }
}

// --------------------
// EngineSchematic - Knows how to sum the part numbers
// --------------------
#[derive(Debug)]
struct EngineSchematic {
    schematic: Vec<Vec<NodeType>>
}

impl EngineSchematic {
    fn sum_part_numbers(self) -> i32{
        let mut result: i32 = 0;

        let rows = self.schematic.len();
        let columns = self.schematic[0].len();

        let mut current_number: String = String::new();
        let mut coordinates_to_check = IntelligentCoordinates::new();

        for (row_index, row) in self.schematic.iter().enumerate() {
            for (column_index, item) in row.iter().enumerate() {
                match item {
                    NodeType::Number(num) => {
                        coordinates_to_check.add_neighbours(vec![row_index, column_index], vec![rows, columns]);
                        current_number.push(*num)
                    },
                    _ => {
                        if current_number.is_empty() {
                            continue;
                        } else {
                            for coordinate in coordinates_to_check.coordinates.iter() {
                                match self.schematic[coordinate[0]][coordinate[1]] {
                                    NodeType::Symbol => {
                                        result += current_number.parse::<i32>().unwrap();
                                        break;
                                    },
                                    _ => {
                                        continue;
                                    }
                                }
                            }
                            coordinates_to_check = IntelligentCoordinates::new();
                            current_number = String::new();
                        }
                    }
                }
            }
        }
        result
    }
}
