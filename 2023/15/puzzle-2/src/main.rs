use std::{fs, collections::HashMap};

fn main() {
    let input: Vec<Step> = fs::read_to_string("input.txt").unwrap()
        .split(',')
        .filter(|step| !step.is_empty())
        .map(|step| step.strip_suffix('\n').unwrap_or(step).into())
        .collect();

    let mut lava_facility: LavaFacility = LavaFacility {
        boxes: HashMap::new()
    };

    lava_facility.follow_steps(input);

    // println!("Lava Facility: {:#?}", lava_facility);

    println!("Result: {}", lava_facility.calculate_focusing_power());
}

#[derive(Debug)]
enum Operation {
    Remove,
    Set
}

#[derive(Debug)]
struct Step {
    lense: Lense,
    operation: Operation,
    box_id: usize
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        let (lense, operation) = if s.contains('=') {
            let operation: Operation = Operation::Set;

            let splitted_step: Vec<&str> = s.split('=').collect();
            let lense = Lense {
                label: splitted_step[0].to_string(),
                focal_length: splitted_step[1].parse().unwrap()
            };

            (lense, operation)
        } else {
            let operation: Operation = Operation::Remove;
            let lense = Lense {
                label: s[..s.len() - 1].to_string(),
                focal_length: 0
            };

            (lense, operation)
        };

        let box_id: usize = lense.label
            .chars()
            .fold(0, |acc, c| ((acc + c as u32) * 17) % 256) as usize;

        Self {
            lense,
            operation,
            box_id
        }
    }
}

#[derive(Debug, Clone)]
struct Lense {
    label: String,
    focal_length: usize
}

#[derive(Debug)]
struct Box {
    id: usize,
    lenses: Vec<Lense>,
    lense_map: HashMap<String, Lense>
}

#[derive(Debug)]
struct LavaFacility {
    boxes: HashMap<usize, Box>
}

impl LavaFacility {
    fn follow_steps(&mut self, steps: Vec<Step>) {
        for step in steps {
            match step.operation {
                Operation::Set => {
                    self.boxes
                        .entry(step.box_id)
                        .and_modify(|e| {
                            if e.lense_map.get(&step.lense.label).is_some() {
                                let index = e.lenses.iter().position(|lense| lense.label == step.lense.label).unwrap();
                                e.lenses[index] = step.lense.clone();
                            } else {
                                e.lenses.push(step.lense.clone());
                                e.lense_map.insert(step.lense.label.clone(), step.lense.clone());
                            }
                        })
                        .or_insert(Box {
                            id: step.box_id,
                            lenses: vec![step.lense.clone()],
                            lense_map: HashMap::from([(step.lense.label.clone(), step.lense)])
                        });
                },
                Operation::Remove => {
                    self.boxes
                        .entry(step.box_id)
                        .and_modify(|e| {
                            e.lenses.retain(|lense| lense.label != step.lense.label);
                            e.lense_map.remove(&step.lense.label);
                        });
                }
            }
        }
    }

    fn calculate_focusing_power(&self) -> usize {
        self.boxes.values().fold(0, |acc, b| {
            acc + b.lenses.iter().enumerate().fold(0, |inner_acc, (index, lense)| {
                inner_acc + (1 + b.id) * (index + 1) * lense.focal_length
            })
        })
    }
}
