use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    let input: Vec<String> = read_input("input.txt").unwrap().map(|l| l.unwrap()).collect();

    let times: Vec<u32> = input[0]
        .split("Time:")
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|r| !r.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = input[1]
        .split("Distance:")
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|r| !r.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let races: AllRaces = AllRaces::new(times, distances);

    let result: u32 = races.races.iter()
        .map(|race| race.determine_number_of_ways_to_beat_the_record())
        .fold(1, |acc, x| acc * x as u32);

    println!("{:?}", result);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32
}

impl Race {
    fn determine_number_of_ways_to_beat_the_record(&self) -> usize {
        // The function that defines the distance is given by:
        // d = ct * (t - ct) // Where d = distance, ct = charging time, t = time
        // 0 = - ct^2 + ct * t - d
        // ----
        // a = -1, b = t, c = -d
        //
        // delta = b^2 - 4ac
        // bhaskara = (-b +- sqrt(delta)) / 2a

        let a: i32 = -1;
        let b: i32 = self.time as i32;
        let c: i32 = -(self.distance as i32);

        let delta: f32 = (b.pow(2) - 4 * a * c) as f32;

        if delta < 0.0 {
            0
        } else {
            let root_1: f32 = ((-b) as f32 + delta.sqrt()) / (2 * a) as f32;
            let root_2: f32 = ((-b) as f32 - delta.sqrt()) / (2 * a) as f32;
            if root_1 == root_2 {
                (0..=self.time).map(|t| t as f32).filter(|t| t >= &root_1).collect::<Vec<f32>>().len()
            } else {
                (0..=self.time).map(|t| t as f32).filter(|t| t >= &root_1 && t <= &root_2).collect::<Vec<f32>>().len()
            }
        }
    }
}

#[derive(Debug)]
struct AllRaces {
    races: Vec<Race>
}

impl AllRaces {
    fn new(times: Vec<u32>, distances: Vec<u32>) -> Self {
        let races: Vec<Race> = times.iter().zip(distances.iter()).map(|(time, distance)| Race { time: *time, distance: *distance }).collect();

        Self {
            races
        }
    }
}
