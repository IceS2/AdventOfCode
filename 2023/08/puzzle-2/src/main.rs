// LCM Solution works for the input they prepared but it doesn't work for generic input based on
// the problem constraints.
//
// For generic input we should get the biggest cycle (That allows us to jump more steps at once)
// and test each iteration against all other cycles. If for a given iteration all end in Z-nodes,
// solution found.
use std::{fs, collections::HashMap};
use regex::Regex;
use rayon::prelude::*;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let contents_splitted: Vec<&str> = contents.split("\n\n").collect();

    let instructions: Vec<char> = contents_splitted[0].chars().collect();
    let mut map: Map = Map::new();

    for node_desc in contents_splitted[1].split('\n').filter(|r| !r.is_empty()) {
        let re = Regex::new(r"(?<current>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
        let captures = re.captures(node_desc).unwrap();

        let node: Node = Node::new(&captures["current"], &captures["left"], &captures["right"]);
        map.add_node(node);
    }

    map.navigate(&instructions);
}

#[derive(Debug)]
struct Node {
    current: String,
    left: String,
    right: String
}

impl Node {
    fn new(current: &str, left: &str, right: &str) -> Self {
        Self {
            current: current.to_string(),
            left: left.to_string(),
            right: right.to_string()
        }
    }

    fn get_next(&self, instruction: &char) -> &str {
        match instruction {
            'L' => self.left.as_str(),
            'R' => self.right.as_str(),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Cycle {
    start: u32,
    length: u32,
    relevant_nodes: HashMap<String, u32>
}

#[derive(Debug)]
struct Map {
    map: HashMap<String, Node>
}

impl Map {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn add_node(&mut self, node: Node) {
        self.map.insert(node.current.clone(), node);
    }

    fn get_nodes_ending_with_a(&self) -> Vec<&str> {
        self.map.keys().filter(|n| n.ends_with('A')).map(|n| n.as_str()).collect()
    }

    fn get_cycle(&self, start: &str, instructions: &Vec<char>) -> Cycle {
        let mut count: u32 = 0;
        let mut current: &str = start;
        let mut instructions_index: usize = 0;

        let mut visited: HashMap<(&str, usize), u32> = HashMap::new();
        let mut relevant_nodes: Vec<(&str, u32)> = vec![];

        loop {
            let instruction = &instructions[instructions_index];
            current = self.map.get(current).unwrap().get_next(instruction);

            let new_visit = (current, instructions_index);

            if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(new_visit) {
                count += 1;
                e.insert(count);
                if current.ends_with('Z') {
                    relevant_nodes.push((current, count))
                }
            } else {
                let cycle_start = *visited.get(&new_visit).unwrap();

                return Cycle {
                    start: cycle_start,
                    length: count - cycle_start + 1,
                    relevant_nodes: HashMap::from_iter(
                        relevant_nodes
                            .into_iter()
                            .filter(|(_, v)| *v >= cycle_start)
                            .map(|(k, v)| (k.to_string(), v))),
                };
            }

            if instructions_index == instructions.len() -1 {
                instructions_index = 0;
            } else {
                instructions_index += 1;
            }
        }

    }

    fn navigate(&self, instructions: &Vec<char>) {
        let current_nodes: Vec<&str> = self.get_nodes_ending_with_a();
        let cycles: Vec<Cycle> = current_nodes.into_par_iter().map(|node| self.get_cycle(node, instructions)).collect();

        println!("Cycles: {:?}", cycles);

        let relevant_nodes_location: Vec<u64> = cycles.iter().map(|c| *c.relevant_nodes.values().min().unwrap() as u64).collect();
        println!("Relevant nodes: {:?}", relevant_nodes_location);

        let lcm: u128 = relevant_nodes_location.iter().fold(1, |acc, x| primefactor::u128_lcm(acc, *x as u128));
        println!("LCM: {}", lcm);
    }
}
