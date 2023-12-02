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
        if game.is_possible_with_given_configuration(14, 13, 12) {
            result += game.id as i32;
        }
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
    fn is_possible_with_given_configuration(&self, blue: usize, green: usize, red:usize) -> bool {
        for set in &self.sets {
            if let Some(quantity) = set.blue {
                if quantity > blue {
                    return false;
                }
            }
            if let Some(quantity) = set.green {
                if quantity > green {
                    return false;
                }
            }
            if let Some(quantity) = set.red {
                if quantity > red {
                    return false;
                }
            }
        }
        true
    }
}


fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
