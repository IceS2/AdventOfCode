use std::{fs, collections::HashMap};
use regex::Regex;

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

    println!("{:?}", map.navigate("AAA", "ZZZ", &instructions));
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

    fn navigate(&self, start: &str, end: &str, instructions: &Vec<char>) -> u32 {
        let mut count: u32 = 0;
        let mut instructions_index: usize = 0;
        let mut current: &str = start;

        while current != end {
            let instruction = &instructions[instructions_index];
            current = self.map.get(current).unwrap().get_next(instruction);
            count += 1;
            if instructions_index == instructions.len() - 1 {
                instructions_index = 0;
            } else {
                instructions_index += 1;
            }
        }
        count
    }
}
