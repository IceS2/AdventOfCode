use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let workflows: HashMap<String, Workflow> = input[0]
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| Workflow::from(s))
        .map(|w| (w.name.clone(), w))
        .collect();

    let part_ratings: Vec<PartRating> = input[1]
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect();


    let mut sum: usize = 0;

    for part_rating in part_ratings {
        let mut response: WorkflowResponse = WorkflowResponse::Routed("in".to_string());

        while let WorkflowResponse::Routed(workflow) = response {
            response = workflows[&workflow].run(&part_rating);
        }

        if response == WorkflowResponse::Accepted {
            sum += part_rating.x + part_rating.m + part_rating.a + part_rating.s;
        }
    }

    println!("Sum: {:?}", sum);

}

#[derive(Debug)]
struct PartRating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl PartRating {
    fn get_rating(&self, category: &Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s
        }
    }
}

impl From<&str> for PartRating {
    fn from(s: &str) -> Self {
        let splitted: Vec<(char, usize)> = s[1..s.len() - 1]
            .split(',')
            .map(|rate|
                rate
                    .split('=')
                    .collect::<Vec<&str>>()
            )
            .map(|rate| (
                rate[0].chars().next().unwrap(),
                rate[1].parse().unwrap()
            ))
            .collect();

        let map: HashMap<char, usize> = splitted.into_iter().collect();

        Self {
            x: map[&'x'],
            m: map[&'m'],
            a: map[&'a'],
            s: map[&'s'],
        }
    }
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
enum Operator {
    LesserThan,
    GreaterThan,
    EqualTo
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '<' => Operator::LesserThan,
            '>' => Operator::GreaterThan,
            '=' => Operator::EqualTo,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Rule {
    category: Option<Category>,
    operator: Option<Operator>,
    value: Option<usize>,
    if_true: String
}

impl Rule {
    fn apply(&self, part_rating: &PartRating) -> Option<String> {
        match self.operator {
            Some(Operator::LesserThan) => {
                if part_rating.get_rating(self.category.as_ref().unwrap()) < self.value.unwrap() {
                    Some(self.if_true.clone())
                } else {
                    None
                }
            },
            Some(Operator::GreaterThan) => {
                if part_rating.get_rating(self.category.as_ref().unwrap()) > self.value.unwrap() {
                    Some(self.if_true.clone())
                } else {
                    None
                }
            },
            Some(Operator::EqualTo) => {
                if part_rating.get_rating(self.category.as_ref().unwrap()) == self.value.unwrap() {
                    Some(self.if_true.clone())
                } else {
                    None
                }
            },
            None => Some(self.if_true.clone())
        }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        if s.contains(':') {
            let re: Regex = Regex::new(r"(?<category>[xmas])(?<operator>[=<>])(?<value>\d+):(?<if_true>\w+)").unwrap();
            let captures = re.captures(s).unwrap();

            Self {
                category: Some(captures["category"].chars().next().unwrap().into()),
                operator: Some(captures["operator"].chars().next().unwrap().into()),
                value: Some(captures["value"].parse().unwrap()),
                if_true: captures["if_true"].to_string(),
            }
        } else {
            Self {
                category: None,
                operator: None,
                value: None,
                if_true: s.to_string(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum WorkflowResponse {
    Accepted,
    Refused,
    Routed(String)
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: HashMap<usize, Rule>
}

impl Workflow {
    fn run(&self, part_rating: &PartRating) -> WorkflowResponse {
        for idx in 0..self.rules.len() {
            match self.rules[&idx].apply(&part_rating) {
                None => { continue; },
                Some(answer) => {
                    let response = match answer.as_str() {
                        "R" => WorkflowResponse::Refused,
                        "A" => WorkflowResponse::Accepted,
                        workflow => WorkflowResponse::Routed(workflow.to_string())
                    };
                    return response;
                }
            }
        }
        unreachable!();
    }
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Self {
        let splitted: Vec<&str> = s.split('{').collect();

        let mut rules: HashMap<usize, Rule> = HashMap::new();

        for (idx, rule) in splitted[1][..splitted[1].len() - 1].split(',').enumerate() {
            rules.insert(idx, rule.into());
        }

        Self {
            name: splitted[0].to_string(),
            rules
        }
    }
}
