use std::{fs, collections::HashMap, ops::Range};
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

    let initial_part_rating_range: PartRatingRange = PartRatingRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    let mut state = vec![WorkflowResponse::Routed((initial_part_rating_range, "in".to_string()))];
    let mut valid_ranges: Vec<PartRatingRange> = vec![];

    while let Some(head) = state.pop() {
        match head {
            WorkflowResponse::Routed((part_rating_range, workflow)) => {
                state.append(&mut workflows[&workflow].run(&part_rating_range));
            },
            WorkflowResponse::Refused(_) => {
                continue;
            },
            WorkflowResponse::Accepted(part_rating_range) => {
                valid_ranges.push(part_rating_range);
            },
        }
    }

    println!("Valid ranges: {:?}", valid_ranges);

    let combinations = valid_ranges.iter().fold(0, |acc, valid_range| {
        acc + (valid_range.x.len() * (valid_range.m.len()) * (valid_range.a.len()) * (valid_range.s.len()))
    });


    println!("Combinations: {:?}", combinations);

}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRatingRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRatingRange {
    fn get_rating_range(&self, category: &Category) -> Range<usize> {
        match category {
            Category::X => self.x.clone(),
            Category::M => self.m.clone(),
            Category::A => self.a.clone(),
            Category::S => self.s.clone(),
        }
    }

    fn set_rating_range(&mut self, category: &Category, range: Range<usize>) {
        match category {
            Category::X => self.x = range,
            Category::M => self.m = range,
            Category::A => self.a = range,
            Category::S => self.s = range,
        };
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
    fn apply(&self, part_rating_range: &PartRatingRange) -> Option<(PartRatingRange, String)> {
        match self.operator {
            Some(Operator::LesserThan) => {
                let range = part_rating_range.get_rating_range(self.category.as_ref().unwrap());
                let value = self.value.unwrap();

                if range.start > value || range.end <= value {
                    None
                } else {
                    let mut new_part_rating_range: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range.set_rating_range(self.category.as_ref().unwrap(), range.start..value);

                    Some((new_part_rating_range, self.if_true.clone()))
                }
            },
            Some(Operator::GreaterThan) => {
                let range = part_rating_range.get_rating_range(self.category.as_ref().unwrap());
                let value = self.value.unwrap();

                if range.start > value || range.end <= value {
                    None
                } else {
                    let mut new_part_rating_range: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range.set_rating_range(self.category.as_ref().unwrap(), value + 1..range.end);

                    Some((new_part_rating_range, self.if_true.clone()))
                }
            },
            Some(Operator::EqualTo) => {
                let range = part_rating_range.get_rating_range(self.category.as_ref().unwrap());
                let value = self.value.unwrap();

                if range.start > value || range.end <= value {
                    None
                } else {
                    let mut new_part_rating_range: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range.set_rating_range(self.category.as_ref().unwrap(), value..value + 1);

                    Some((new_part_rating_range, self.if_true.clone()))
                }
            },
            None => Some((part_rating_range.clone(), self.if_true.clone()))
        }
    }

    fn get_rating_range_for_which_it_does_not_apply(
        &self,
        part_rating_range: &PartRatingRange
    ) -> Option<Vec<PartRatingRange>> {

        match self.operator {
            Some(Operator::LesserThan) => {
                let range = part_rating_range.get_rating_range(self.category.as_ref().unwrap());
                let value = self.value.unwrap();

                if range.start > value || range.end <= value {
                    Some(vec![part_rating_range.clone()])
                } else {
                    let mut new_part_rating_range: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range.set_rating_range(self.category.as_ref().unwrap(), value..range.end);

                    Some(vec![new_part_rating_range])
                }
            },
            Some(Operator::GreaterThan) => {
                let range = part_rating_range.get_rating_range(self.category.as_ref().unwrap());
                let value = self.value.unwrap();

                if range.start > value || range.end <= value {
                    Some(vec![part_rating_range.clone()])
                } else {
                    let mut new_part_rating_range: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range.set_rating_range(self.category.as_ref().unwrap(), range.start..value + 1);

                    Some(vec![new_part_rating_range])
                }
            },
            Some(Operator::EqualTo) => {
                let range = part_rating_range.get_rating_range(self.category.as_ref().unwrap());
                let value = self.value.unwrap();

                if range.start > value || range.end <= value {
                    Some(vec![part_rating_range.clone()])
                } else if range.start == value && range.end == value + 1 {
                    None
                } else {
                    let mut new_part_rating_range_below: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range_below.set_rating_range(
                        self.category.as_ref().unwrap(),
                        range.start..value
                    );

                    let mut new_part_rating_range_above: PartRatingRange = part_rating_range.clone();
                    new_part_rating_range_above.set_rating_range(
                        self.category.as_ref().unwrap(),
                        value + 1..range.end
                    );

                    Some(vec![new_part_rating_range_below, new_part_rating_range_above])
                }
            },
            None => None
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
    Accepted(PartRatingRange),
    Refused(PartRatingRange),
    Routed((PartRatingRange, String))
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: HashMap<usize, Rule>
}

impl Workflow {
    fn run(&self, part_rating_range: &PartRatingRange) -> Vec<WorkflowResponse> {
        let mut results: Vec<WorkflowResponse> = vec![];
        let mut part_rating_ranges_left: Vec<PartRatingRange> = vec![part_rating_range.clone()];

        for idx in 0..self.rules.len() {
            for part_rating_range_to_apply in &part_rating_ranges_left {
                match self.rules[&idx].apply(part_rating_range_to_apply) {
                    None => { continue; },
                    Some(answer) => {
                        let (new_part_rating_range, response) = answer;

                        match response.as_str() {
                            "R" => {
                                results.push(WorkflowResponse::Refused(new_part_rating_range))
                            },
                            "A" => {
                                results.push(WorkflowResponse::Accepted(new_part_rating_range))
                            },
                            workflow => {
                                results.push(WorkflowResponse::Routed((new_part_rating_range, workflow.to_string())))
                            }
                        };
                    }
                }
            }

            let mut new_part_rating_ranges_left: Vec<PartRatingRange> = vec![];

            for part_rating_range_left in part_rating_ranges_left {
                match self.rules[&idx].get_rating_range_for_which_it_does_not_apply(&part_rating_range_left) {
                    None => { continue; },
                    Some(mut new_part_rating_ranges) => {
                        new_part_rating_ranges_left.append(&mut new_part_rating_ranges)
                    }
                }

            }

            part_rating_ranges_left = new_part_rating_ranges_left;
        }
        results
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
