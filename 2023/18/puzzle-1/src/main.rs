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


    lagoon.draw();

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
    meters: usize,
    color: String
}

impl From<Vec<&str>> for TrenchInstruction {
    fn from(row: Vec<&str>) -> Self {
        TrenchInstruction {
            direction: row[0].chars().next().unwrap().into(),
            meters: row[1].parse().unwrap(),
            color: row[2][1..row[2].len() - 1].to_string()
        }
    }
}

impl TrenchInstruction {
    fn end(&self, start: (isize, isize)) -> (isize, isize) {
        let meters = self.meters as isize;

        match self.direction {
            Direction::Up => (start.0 - meters, start.1),
            Direction::Down => (start.0 + meters, start.1),
            Direction::Left => (start.0, start.1 - meters),
            Direction::Right => (start.0, start.1 + meters),
        }
    }
}

#[derive(Debug)]
struct Trench {
    start: (isize, isize),
    end: (isize, isize)
}

#[derive(Debug)]
struct Lagoon {
    trenches: Vec<Trench>
}

impl From<Vec<TrenchInstruction>> for Lagoon {
    fn from(v: Vec<TrenchInstruction>) -> Self {
        let mut trenches: Vec<Trench> = vec![];
        let mut current_coordinates: (isize, isize) = (0, 0);

        for instruction in &v {
            let start = current_coordinates;
            let end = instruction.end(start);

            trenches.push(Trench { start, end });

            current_coordinates = end;
        }

        let normalized_parameter_for_x = trenches.iter().map(|trench| trench.start.0).min().unwrap();
        let normalized_parameter_for_y = trenches.iter().map(|trench| trench.start.1).min().unwrap();

        for trench in &mut trenches {
            trench.start.0 += -normalized_parameter_for_x;
            trench.start.1 += -normalized_parameter_for_y;
            trench.end.0 += -normalized_parameter_for_x;
            trench.end.1 += -normalized_parameter_for_y;
        }

        Self { trenches }
    }
}

impl Lagoon {
    fn draw(&self) {
        let ncols: usize = self.trenches.iter().map(|trench| trench.end.1).max().unwrap() as usize + 1;
        let nrows: usize = self.trenches.iter().map(|trench| trench.end.0).max().unwrap() as usize + 1;

        let mut matrix: Vec<Vec<char>> = vec![vec!['.'; ncols]; nrows];

        for trench in &self.trenches {
            let start = trench.start.min(trench.end);
            let end = trench.start.max(trench.end);
            (start.0..=end.0).for_each(|x| {
                (start.1..=end.1).for_each(|y| {
                    matrix[x as usize][y as usize] = '#';
                })
            });
        }

        for  row in matrix {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }

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
