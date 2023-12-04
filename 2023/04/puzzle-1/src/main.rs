use std::{path::Path, fs::File, io::{self, BufRead}, collections::HashSet};
use regex::Regex;

fn main() {
    let input = read_input("input.txt").unwrap();
    let mut total_points: i32 = 0;

    for line in input {
        let card: Card = line.unwrap().into();
        total_points += card.calculate_points();
    }
    println!("Total points: {:?}", total_points);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>
}

impl From<String> for Card {
    fn from(s: String) -> Self {
        let re = Regex::new(r"Card\s+(?<id>\d+): (?<wnumbers>[0-9\s]+)\|(?<numbers>[0-9\s]+)").unwrap();
        let captures = re.captures(&s).unwrap();

        let id: usize = captures["id"].parse().unwrap();
        let winning_numbers:HashSet<usize> = captures["wnumbers"].trim().split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let numbers: HashSet<usize> = captures["numbers"].trim().split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();

        Self {
            id,
            winning_numbers,
            numbers
        }
    }
}

impl Card {
    fn calculate_points(&self) -> i32 {
        let intersection: Vec<usize> = self.winning_numbers.intersection(&self.numbers).copied().collect();

        if intersection.is_empty() {
            return 0;
        }

        2_i32.pow((intersection.len() - 1) as u32)
    }
}
