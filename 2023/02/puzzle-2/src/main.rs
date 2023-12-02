use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let input = read_input("input.txt").unwrap();

    let mut result: i32 = 0;

    for line in input {
        let game: Game = line.unwrap().into();
        let minimum_set_of_cubes = game.find_minimum_set_of_cubes();
        result += minimum_set_of_cubes.into_iter().fold(1, |acc, q| acc * q as i32)
    }

    println!("Result: {:?}", result);

}

#[derive(Debug)]
struct Set {
    blue: Option<usize>,
    green: Option<usize>,
    red: Option<usize>,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl From<String> for Game {
    fn from(s: String) -> Self {
        let re = Regex::new(r"Game (?<id>\d+): (?<sets>.+)").unwrap();
        let captures = re.captures(&s).unwrap();

        let id: usize = captures["id"].parse().unwrap();

        let sets: Vec<Set> = captures["sets"].split(';')
            .map(|s| s.trim().split(", ").collect())
            .map(|v: Vec<&str>| {
                let mut set_map: HashMap<String, usize> = HashMap::new();

                for item in v {
                    let color_set: Vec<&str> = item.split(' ').collect();
                    set_map.insert(color_set[1].to_string(), color_set[0].to_string().parse().unwrap());
                }

                set_map
            })
            .map(|mut h: HashMap<String, usize>| Set{ blue: h.remove("blue"), green: h.remove("green"), red: h.remove("red") })
            .collect();

        Self {
            id,
            sets
        }
    }
}

impl Game {
    fn find_minimum_set_of_cubes(&self) -> Vec<usize> {
        let mut blue: usize = 0;
        let mut green: usize = 0;
        let mut red: usize = 0;

        for set in &self.sets {
            if let Some(quantity) = set.blue {
                if quantity > blue {
                    blue = quantity;
                }
            }
            if let Some(quantity) = set.green {
                if quantity > green {
                    green = quantity;
                }
            }
            if let Some(quantity) = set.red {
                if quantity > red {
                    red = quantity;
                }
            }
        }

        vec![blue, green, red]
    }
}


fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
