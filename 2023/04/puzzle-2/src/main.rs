use std::{path::Path, fs::File, io::{self, BufRead}, collections::{HashSet, HashMap}};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let input = read_input("input.txt").unwrap();
    let mut card_stack: CardStack = CardStack::new();

    for line in input {
        let card: Card = line.unwrap().into();
        card_stack.add(card);
    }

    card_stack.process();
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
    fn calculate_points(&self) -> usize {
        let intersection: Vec<usize> = self.winning_numbers.intersection(&self.numbers).copied().collect();

        intersection.len()
    }
}

#[derive(Debug)]
struct CardStack {
    cards: HashMap<usize, Card>
}

impl CardStack {
    fn new() -> Self {
        Self {
            cards: HashMap::new()
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.entry(card.id).or_insert(card);
    }

    fn process(&self) {
        let mut ref_map: HashMap<usize, usize> = self.cards.keys().map(|k| (*k, 1)).collect();

        for card_id in self.cards.keys().sorted() {
            let points = &self.cards.get(card_id).unwrap().calculate_points();
            let card_quantity = *ref_map.get(card_id).unwrap();

            for point in 1..points+1 {
                let id = card_id + point;
                ref_map.entry(id).and_modify(|q| *q += card_quantity);
            }
        }

        println!("{:?}", ref_map.values().sum::<usize>());
    }
}
