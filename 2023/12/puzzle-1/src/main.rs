use std::{path::Path, fs::File, io::{self, BufRead}, collections::VecDeque};

fn main() {
    let input = read_input("input.txt").unwrap();

    let mut sum: u32 = 0;

    for line in input {
        let line_split: Vec<String> = line.unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let mut row: VecDeque<char> = line_split[0]
            .chars()
            .collect();

        let groups: VecDeque<usize> = line_split[1]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .iter()
            .map(|c| c.parse().unwrap())
            .collect();

        sum += solve(row, groups, 0);
    }

    println!("Sum: {:?}", sum);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(mut s: VecDeque<char>, mut groups_left: VecDeque<usize>, current_group_size: usize) -> u32 {
    // println!("Char: {:?}", c);
    // println!("String: {:?}", s);
    // println!("Groups left: {:?}", groups_left);
    // println!("Current group size: {:?}", current_group_size);
    if groups_left.is_empty() {
        match s.iter().any(|c| c == &'#') {
            true => return 0,
            false => return 1
        }
    }

    match s.pop_front() {
        None => {
            if groups_left.len() != 1 {
                0
            } else if current_group_size == groups_left[0] {
                1
            } else {
                0
            }
        }
        Some('.') => {
            if current_group_size == 0 {
                solve(s, groups_left, current_group_size)
            } else if current_group_size == groups_left[0] {
                groups_left.pop_front();
                solve(s, groups_left, 0)
            } else {
                0
            }
        },
        Some('#') => {
            if current_group_size <= groups_left[0]{
                solve(s, groups_left, current_group_size + 1)
            } else {
                0
            }
        },
        Some('?') => {
            let mut s1_clone = s.clone();
            let mut s2_clone = s.clone();

            s1_clone.push_front('.');
            s2_clone.push_front('#');

            solve(s1_clone, groups_left.clone(), current_group_size) + solve(s2_clone, groups_left, current_group_size)
        },
        _ => unreachable!()
    }
}
