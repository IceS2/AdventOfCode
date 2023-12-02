use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input = read_input("input.txt").unwrap();
    let mut result: i32 = 0;

    for line in input {
        println!("Line: {:?}", &line);
        let parsed_line: SplittedLine = line.unwrap().into();
        let relevant_number = parsed_line.find_relevant_number();

        println!("Parsed Line: {:?}", parsed_line);
        println!("Relevant Number: {:?}", relevant_number);
        println!();
        result += relevant_number;
    }

    println!("{:?}", result);
    // let split_line: SplittedLine = "twovgtprdzcjjzkq3ffsbcblnpq".to_string().into();
    // println!("twovgtprdzcjjzkq3ffsbcblnpq");
    // println!("{:?}", split_line);
    // println!("{:?}", split_line.find_relevant_number());
    //
    // let split_line: SplittedLine = "two8sixbmrmqzrrb1seven".to_string().into();
    // println!("two8sixbmrmqzrrb1seven");
    // println!("{:?}", split_line);
    // println!("{:?}", split_line.find_relevant_number());
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct SplittedLine {
    head: Option<String>,
    tail: Option<String>,
    first_digit: Option<char>,
    last_digit: Option<char>,
}

impl From<String> for SplittedLine {
    fn from(s: String) -> Self {
        let first_digit_position: Option<usize> = s.chars().position(|c| c.is_ascii_digit());
        let last_digit_position: Option<usize> = s.chars().rev().position(|c| c.is_ascii_digit()).map(|x| s.len() - x - 1);
         Self {
            head: if let Some(position) = first_digit_position { Some(s[..position].to_string()) } else { Some(s[..].to_string())},
            tail: if let Some(position) = last_digit_position { Some(s[position + 1..].to_string()) } else { Some(s[..].to_string())},
            first_digit: if let Some(position) = first_digit_position { Some(s.chars().nth(position).unwrap().clone()) } else { None },
            last_digit: if let Some(position) = last_digit_position { Some(s.chars().nth(position).unwrap().clone()) } else { None }
        }
    }
}

impl SplittedLine {
    fn find_first_digit(&self) -> Option<char> {
        let numbers_map: HashMap<String, char> = HashMap::from([
            ("ONE".to_string(), '1'),
            ("TWO".to_string(), '2'),
            ("THREE".to_string(), '3'),
            ("FOUR".to_string(), '4'),
            ("FIVE".to_string(), '5'),
            ("SIX".to_string(), '6'),
            ("SEVEN".to_string(), '7'),
            ("EIGHT".to_string(), '8'),
            ("NINE".to_string(), '9')
        ]);
        if let Some(head) = &self.head {
            let head_length: usize = head.len();

            if head_length < 3 {
                self.first_digit
            } else {
                let mut result: Option<char> = None;
                let mut offset: usize = 0;
                let mut word_length: usize = 3;
                while offset + 3 <= head_length {
                    if offset + word_length <= head_length {
                        let string_to_test = head[offset..offset+word_length].to_uppercase();

                            match numbers_map.get(&string_to_test) {
                                None => (),
                                Some(number) => {
                                    result = Some(*number);
                                    break;
                                }
                            }
                    }

                    if word_length == 5 {
                        word_length = 3;
                        offset = offset + 1;
                    } else {
                        word_length = word_length + 1;
                    }
                }
                match result {
                    None => self.first_digit,
                    Some(result) => Some(result)
                }
            }
        } else {
            self.first_digit
        }
    }

    fn find_last_digit(&self) -> Option<char> {
        let numbers_map: HashMap<String, char> = HashMap::from([
            ("ONE".to_string(), '1'),
            ("TWO".to_string(), '2'),
            ("THREE".to_string(), '3'),
            ("FOUR".to_string(), '4'),
            ("FIVE".to_string(), '5'),
            ("SIX".to_string(), '6'),
            ("SEVEN".to_string(), '7'),
            ("EIGHT".to_string(), '8'),
            ("NINE".to_string(), '9')
        ]);
        if let Some(tail) = &self.tail {
            let tail_length: usize = tail.len();

            if tail_length < 3 {
                self.last_digit
            } else {
                let mut result: Option<char> = None;
                let mut offset: usize = 0;
                let mut word_length: usize = 3;
                while offset + 3 <= tail_length {
                    if offset + word_length <= tail_length {

                        let string_to_test = tail[tail_length-word_length-offset..tail_length-offset].to_uppercase();

                            match numbers_map.get(&string_to_test) {
                                None => (),
                                Some(number) => {
                                    result = Some(*number);
                                    break;
                                }
                            }
                    }

                    if word_length == 5 {
                        word_length = 3;
                        offset = offset + 1;
                    } else {
                        word_length = word_length + 1;
                    }
                }
                match result {
                    None => self.last_digit,
                    Some(result) => Some(result)
                }
            }
        } else {
            self.last_digit
        }

    }

    fn find_relevant_number(&self) -> i32 {
        let mut relevant_number_str: String = String::new();

        relevant_number_str.push(self.find_first_digit().unwrap());
        relevant_number_str.push(self.find_last_digit().unwrap());

        relevant_number_str.parse().unwrap()
    }
}
