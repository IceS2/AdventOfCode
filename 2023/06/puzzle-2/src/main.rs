use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    let input: Vec<String> = read_input("input.txt").unwrap().map(|l| l.unwrap()).collect();

    let time: u64 = input[0]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .split("Time:")
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let distance: u64 = input[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .split("Distance:")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let race: Race = Race { time, distance };

    println!("{:?}", race.determine_number_of_ways_to_break_the_record());
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn determine_number_of_ways_to_break_the_record(&self) -> usize {
        // The function that defines the distance is given by:
        // d = ct * (t - ct) // Where d = distance, ct = charging time, t = time
        // 0 = - ct^2 + ct * t - d
        // ----
        // a = -1, b = t, c = -d
        //
        // delta = b^2 - 4ac
        // bhaskara = (-b +- sqrt(delta)) / 2a

        let a: i64 = -1;
        let b: i64 = self.time as i64;
        let c: i64 = -(self.distance as i64);

        let delta: f64 = (b.pow(2) - 4 * a * c) as f64;

        if delta < 0.0 {
            0
        } else {
            let root_1: f64 = ((-b) as f64 + delta.sqrt()) / (2 * a) as f64;
            let root_2: f64 = ((-b) as f64 - delta.sqrt()) / (2 * a) as f64;
            if root_1 == root_2 {
                (0..=self.time).map(|t| t as f64).filter(|t| t >= &root_1).collect::<Vec<f64>>().len()
            } else {
                (0..=self.time).map(|t| t as f64).filter(|t| t >= &root_1 && t <= &root_2).collect::<Vec<f64>>().len()
            }
        }
    }
}
