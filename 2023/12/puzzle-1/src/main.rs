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

        sum += solve(row.pop_front(), row, groups, 0);
    }

    println!("Sum: {:?}", sum);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(c: Option<char>, mut s: VecDeque<char>, mut groups_left: VecDeque<usize>, current_group_size: usize) -> u32 {
    // println!("Char: {:?}", c);
    // println!("String: {:?}", s);
    // println!("Groups left: {:?}", groups_left);
    // println!("Current group size: {:?}", current_group_size);

    if groups_left.is_empty() {
        return 1;
    }
    // if groups_left.iter().sum::<usize>() > s.len() {
    //     return 0;
    // }

    match c {
        None => {
            1
        }
        Some('.') => {
            if current_group_size == 0 {
                solve(s.pop_front(), s, groups_left, current_group_size)
            } else if current_group_size == groups_left[0] {
                groups_left.pop_front();
                solve(s.pop_front(), s, groups_left, 0)
            } else if current_group_size != groups_left[0] {
                0
            } else {
                unreachable!()
            }
        },
        Some('#') => {
            let current_group_size = current_group_size + 1;

            if current_group_size > groups_left[0] {
                0
            } else if current_group_size <= groups_left[0] {
                solve(s.pop_front(), s, groups_left, current_group_size)
            } else {
                unreachable!()
            }

        },
        Some('?') => {
            solve(Some('.'), s.clone(), groups_left.clone(), current_group_size) + solve(Some('#'), s, groups_left, current_group_size)

        },
        _ => unreachable!()
    }
}
