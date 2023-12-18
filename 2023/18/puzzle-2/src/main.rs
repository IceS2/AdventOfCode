use std::fs;

fn main() {
    let input: Vec<TrenchInstruction> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row|
            row
                .split(' ')
                .collect::<Vec<&str>>()
        )
        .map(|row| row.into())
        .collect();

    let lagoon: Lagoon = input.into();

    println!("Area: {:?}", lagoon.area());

    let lagoon_perimeter = lagoon.perimeter();
    let lagoon_internal_points = lagoon.internal_points();

    println!("Perimeter: {:?}", lagoon_perimeter);
    println!("Internal points: {:?}", lagoon_internal_points);

    println!("Lagoon Size: {:?}", lagoon_perimeter + lagoon_internal_points);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct TrenchInstruction {
    direction: Direction,
    meters: i64,
}

impl From<Vec<&str>> for TrenchInstruction {
    fn from(row: Vec<&str>) -> Self {
        let code: String = row[2][2..row[2].len() - 1].to_string();

        let meters: i64 = i64::from_str_radix(&code[..code.len() - 1], 16).unwrap();

        let direction: Direction = match code.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!()
        };

        TrenchInstruction {
            direction,
            meters
        }
    }
}

impl TrenchInstruction {
    fn end(&self, start: (i64, i64)) -> (i64, i64) {
        match self.direction {
            Direction::Up => (start.0 - self.meters, start.1),
            Direction::Down => (start.0 + self.meters, start.1),
            Direction::Left => (start.0, start.1 - self.meters),
            Direction::Right => (start.0, start.1 + self.meters),
        }
    }
}

#[derive(Debug)]
struct Trench {
    start: (i64, i64),
    end: (i64, i64)
}

#[derive(Debug)]
struct Lagoon {
    trenches: Vec<Trench>
}

impl From<Vec<TrenchInstruction>> for Lagoon {
    fn from(v: Vec<TrenchInstruction>) -> Self {
        let mut trenches: Vec<Trench> = vec![];
        let mut current_coordinates: (i64, i64) = (0, 0);

        for instruction in &v {
            let start = current_coordinates;
            let end = instruction.end(start);

            trenches.push(Trench { start, end });

            current_coordinates = end;
        }

        Self { trenches }
    }
}

impl Lagoon {

    fn area(&self) -> usize {
        // Area of Polygons in Coordinate Geometry
        (self.trenches.iter().fold(0, |acc, trench| {
            acc + (trench.start.0 * trench.end.1) - (trench.end.0 * trench.start.1)
        }).abs() / 2) as usize
    }

    fn perimeter(&self) -> usize {
        // Perimeter as usual (sum of all sides)
        self.trenches.iter().fold(0, |acc, trench| {
            let start = trench.start.min(trench.end);
            let end = trench.start.max(trench.end);

            acc + (end.0 - start.0) + (end.1 - start.1)
        }) as usize
    }

    fn internal_points(&self) -> usize {
        // Calculates internal Points based on Pick's theorem
        self.area() + 1 - self.perimeter() / 2
    }
}
