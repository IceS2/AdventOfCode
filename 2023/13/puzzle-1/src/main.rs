use std::fs;

fn main() {
    let inputs: Vec<Pattern> = fs::read_to_string("input.txt").unwrap()
        .split("\n\n")
        .map(|input|
            input.split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>())
        .map(|v| v.into())
        .collect();
    let mut sum: usize = 0;

    for input in inputs {
        let point_summary = input.find_reflection_point_summary();
        sum += point_summary;
        println!("Input: {:?}", point_summary);
    }
    println!("Sum: {:?}", sum);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let col_len = v[0].len();

    let mut inner_vec_as_iter: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..col_len)
        .map(|_| {
            inner_vec_as_iter
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_reflection_point(matrix: &Vec<Vec<char>>) -> Option<usize> {
    let number_of_rows: usize = matrix.len();

    'next_row: for (row_index, _) in matrix.iter().enumerate() {
        if row_index == number_of_rows - 1 {
            return None;
        }

        let limit = Ord::min(row_index, number_of_rows - row_index) + 1;

        for i in 0..limit {
            if row_index + i + 1 < number_of_rows && matrix[row_index - i] != matrix[row_index + i + 1] {
                continue 'next_row
            }
        }
        return Some(row_index + 1);
    }
    None
}


#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>
}

impl From<Vec<Vec<char>>> for Pattern {
    fn from(v: Vec<Vec<char>>) -> Self {
        Self { rows: v }
    }
}

impl Pattern {
    fn find_reflection_point_summary(&self) -> usize {
        match find_reflection_point(&self.rows) {
            Some(row_reflection) => 100 * row_reflection,
            None => find_reflection_point(&transpose(self.rows.clone())).unwrap()
        }
    }
}
