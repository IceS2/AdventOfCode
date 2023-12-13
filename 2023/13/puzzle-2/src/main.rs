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
        // println!("Input: {:?}", input);
        let point_summary = input.find_reflection_point_summary(1);
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

fn find_reflection_point(matrix: &Vec<Vec<char>>, known_smudges: usize) -> Option<usize> {
    let number_of_rows: usize = matrix.len();

    for (row_index, _) in matrix.iter().enumerate() {
        if row_index == number_of_rows - 1 {
            return None;
        }

        let limit = Ord::min(row_index, number_of_rows - row_index) + 1;
        let mut errors: usize = 0;

        for i in 0..limit {
            if row_index + i + 1 < number_of_rows {
                let left = &matrix[row_index - i];
                let right = &matrix[row_index + i + 1];

                errors += left.iter().zip(right).filter(|(l, r)| l != r).count();
            }
        }

        if errors == known_smudges {
            return Some(row_index + 1);
        }
    }
    None
}

// #[derive(Debug, PartialEq, Eq)]
// enum ReflectionPoint {
//     Row(usize),
//     Column(usize)
// }

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
    fn find_reflection_point_summary(&self, known_smudges: usize) -> usize {
        match find_reflection_point(&self.rows, known_smudges) {
            Some(row_reflection) => 100 * row_reflection,
            None => find_reflection_point(&transpose(self.rows.clone()), known_smudges).unwrap()
        }

        // let columns = self.rows[0].len();
        // let mut new_reflection_point: Option<ReflectionPoint> = None;
        // let mut index: usize = 0;
        // println!("Original reflection point: {:?}", original_reflection_point);
        //
        // while new_reflection_point.is_none() {
        //     // println!("New reflection point: {:?}", new_reflection_point);
        //     // println!("Index: {:?}", index);
        //     let row = index / columns;
        //     let col = index % columns;
        //
        //     let mut corrected = self.rows.clone();
        //
        //     match corrected[row][col] {
        //         '#' => corrected[row][col] = '.',
        //         '.' => corrected[row][col] = '#',
        //         _ => unreachable!()
        //     }
        //
        //     new_reflection_point = match find_reflection_point(&corrected) {
        //         Some(row_reflection) => Some(ReflectionPoint::Row(row_reflection)),
        //         None => {
        //             find_reflection_point(&transpose(corrected.clone())).map(ReflectionPoint::Column)
        //         }
        //     };
        //
        //     index += 1;
        // }
        // println!("New reflection point: {:?}", new_reflection_point);
        //
        // if let Some(reflection_point) = new_reflection_point {
        //     match reflection_point {
        //         ReflectionPoint::Row(point) => return 100 * point,
        //         ReflectionPoint::Column(point) => return point,
        //     }
        // }
        //
        // unreachable!()
    }
}
