use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    let input = read_input("input.txt").unwrap();
    let mut prediction_sum: i32 = 0;

    for line in input {
        let history: ValueHistory = line.unwrap().into();
        prediction_sum += ValueHistory::predict_previous_value(&history.history[..]);
    }

    println!("Prediction Sum: {:?}", prediction_sum);
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct ValueHistory {
    history: Vec<i32>
}

impl From<String> for ValueHistory {
    fn from(s: String) -> Self {
        Self {
            history: s.split(' ').map(|v| v.parse().unwrap()).collect()
        }
    }
}

impl ValueHistory {
    fn predict_previous_value(history: &[i32]) -> i32 {
        if history.iter().all(|v| v == &0) {
            0
        } else {
            let sub_vector: Vec<i32> = history.windows(2).map(|w| w[1] - w[0]).collect();
            history.first().unwrap() - ValueHistory::predict_previous_value(&sub_vector[..])
        }
    }
}
