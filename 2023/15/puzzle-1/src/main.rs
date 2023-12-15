use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap()
        .split(',')
        .map(|step|
            step
                .chars()
                .filter(|c| c != &'\n')
                .collect::<Vec<char>>()
        )
        .filter(|step| !step.is_empty())
        .collect();

    let result: u32 = input
        .iter()
        .map(|step|
            step
                .iter()
                .fold(0, |acc, c| ((acc + *c as u32) * 17) % 256)
        )
        .sum();

    println!("Result: {}", result);
}
