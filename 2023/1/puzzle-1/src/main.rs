use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = read_input("input.txt").unwrap();
    let mut result: i32 = 0;

    for line in input {
        let digits: Vec<char> = line.unwrap().chars().filter(|c| c.is_ascii_digit()).collect();
        let mut number_as_str: String = String::from("");
        number_as_str.push(digits[0]);
        number_as_str.push(digits[digits.len() - 1]);
        let number: i32 =  number_as_str.parse().unwrap();
        result += number;
    }

    println!("{:?}", result);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
