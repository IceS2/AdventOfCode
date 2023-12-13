use std::{path::Path, fs::File, io::{self, BufRead}, collections::{VecDeque, HashMap}};

fn main() {
    let input = read_input("input.txt").unwrap();

    let mut sum: u64 = 0;

    for line in input {
        let mut memo: HashMap<(VecDeque<char>, VecDeque<usize>, usize), u64> = HashMap::new();

        let line_split: Vec<String> = line.unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let mut row: VecDeque<char> = line_split[0]
            .chars()
            .collect();

        let mut row_clone = row.clone();

        let mut groups: VecDeque<usize> = line_split[1]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .iter()
            .map(|c| c.parse().unwrap())
            .collect();

        let mut groups_clone = groups.clone();

        for _ in 0..4 {
            row.push_back('?');
            row.append(&mut row_clone.clone());

            groups.append(&mut groups_clone.clone());
        }

        sum += solve(row, groups, 0, &mut memo);
    }

    println!("Sum: {:?}", sum);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(mut s: VecDeque<char>, mut groups_left: VecDeque<usize>, current_group_size: usize, memo: &mut HashMap<(VecDeque<char>, VecDeque<usize>, usize), u64>) -> u64 {
    if groups_left.is_empty() {
        match s.iter().any(|c| c == &'#') {
            true => return 0,
            false => return 1
        }
    }

    let key = (s.clone(), groups_left.clone(), current_group_size);

    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let count = match s.pop_front() {
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
                solve(s, groups_left, current_group_size, memo)
            } else if current_group_size == groups_left[0] {
                groups_left.pop_front();
                solve(s, groups_left, 0, memo)
            } else {
                0
            }
        },
        Some('#') => {
            if current_group_size <= groups_left[0]{
                solve(s, groups_left, current_group_size + 1, memo)
            } else {
                0
            }
        },
        Some('?') => {
            let mut s1_clone = s.clone();
            let mut s2_clone = s.clone();

            s1_clone.push_front('.');
            s2_clone.push_front('#');

            solve(s1_clone, groups_left.clone(), current_group_size, memo) + solve(s2_clone, groups_left, current_group_size, memo)
        },
        _ => unreachable!()
    };

    memo.insert(key, count);

    count
}
