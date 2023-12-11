use std::{path::Path, fs::File, io::{self, BufRead}, iter};

fn main() {
    let input = read_input("input.txt").unwrap();
    let expanded_input: ExpandedInput = input.into();
    let universe: Universe = expanded_input.into();

    println!("{:?}", universe.find_sum_of_distance_between_all_galaxies());
    // println!("{:?}", universe.find_sum_of_shortest_path_between_every_galaxy());

}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let col_len = v[0].len();

    let mut inner_vec_as_iter: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..col_len)
        .map(|_| {
            inner_vec_as_iter
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// ExpandedInput
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct ExpandedInput(Vec<(Vec<(char,  u32)>, u32)>);

impl From<io::Lines<io::BufReader<File>>> for ExpandedInput {
    fn from(l: io::Lines<io::BufReader<File>>) -> Self {
        let mut contents: Vec<Vec<char>> = vec![];

        for line in l {
            contents.push(line.unwrap().chars().collect());
        }

        let row_weights: Vec<u32> = contents.iter().map(|row| {
                if row.iter().any(|c| c == &'#') {
                    1
                } else {
                    1000000
                }
            })
            .collect();

        let transposed: Vec<Vec<char>> = transpose(contents.clone());

        let col_weights: Vec<u32> = transposed.iter().map(|col| {
            if col.iter().any(|c| c == &'#') {
                1
            } else {
                1000000
            }
        })
        .collect();

        Self(
            contents
                .iter()
                .zip(row_weights)
                .map(|(r, rw)| {
                    let row: Vec<(char, u32)> = r.iter().zip(col_weights.clone()).map(|(c, cw)| (*c, cw)).collect();
                    (row, rw)
                })
                .collect()
        )
    }
}

// Galaxy
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Galaxy {
    x: u32,
    y: u32
}

// Universe
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>
}

impl From<ExpandedInput> for Universe {
    fn from(e: ExpandedInput) -> Self {
        let ExpandedInput(expanded_input) = e;
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut galaxies: Vec<Galaxy> = vec![];

        for row in expanded_input.iter() {
            for col in row.0.iter() {
                if col.0 == '#' {
                    galaxies.push(Galaxy { x, y });
                }
                y += col.1;
            }
            y = 0;
            x += row.1
        }
        Self { galaxies }
    }
}

impl Universe {
    fn get_all_galaxy_pairs(&self) -> Vec<(&Galaxy, &Galaxy)> {
        self.galaxies.iter()
            .enumerate()
            .flat_map(|(i, g)| iter::repeat(g).zip(self.galaxies.iter().skip(i + 1)))
            .map(|(g1, g2)| (g1, g2)).collect()
    }

    fn find_sum_of_distance_between_all_galaxies(&self) -> u64 {
        self.get_all_galaxy_pairs()
            .iter()
            .fold(0, |acc, (g1, g2)| {
                acc + ((g1.x as i32 - g2.x as i32).abs() + (g1.y as i32 - g2.y as i32).abs()) as u64
            })
    }
}
