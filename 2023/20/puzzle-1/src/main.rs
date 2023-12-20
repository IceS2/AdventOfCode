use std::{fs, collections::{HashMap, VecDeque}, cell::RefCell};
use regex::Regex;

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    let mut modules: HashMap<String, Module> = input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| Module::from(s.as_str()))
        .map(|module| (module.name.clone(), module))
        .collect();

    let conjunction_modules: Vec<String> = modules
        .iter()
        .filter(|(_, module)| {
            matches!(*module.module_type.borrow(), ModuleType::Conjunction(_))
        })
        .map(|(name, _)| name.clone())
        .collect();

    for module in conjunction_modules {
        let inputs: Vec<String> = modules
            .iter()
            .filter(|(_, m)| {
                m.destination_modules.contains(&module)
            })
            .map(|(name, _)| name.clone())
            .collect();

        modules.entry(module).and_modify(|e| {
            if let ModuleType::Conjunction(ref mut map) = *e.module_type.borrow_mut() {

                for input in inputs.iter() {
                    map.insert(input.to_string(), Pulse::Low);
                }

            }
        });
    }

    // let button: Module = Module {
    //     name: "button".to_string(),
    //     destination_modules: vec!["broadcaster".to_string()],
    //     module_type: RefCell::new(ModuleType::Button)
    // };

    let initial_pulse: Vec<(String, String, Pulse)> = vec![(
        "button".to_string(),
        "broadcaster".to_string(),
        Pulse::Low
    )];

    let mut low_pulses: i32 = 0;
    let mut high_pulses: i32 = 0;


    for _ in 0..1000 {
        // println!("Iteration: {:?}", i + 1);
        // println!("{}\n", "-".repeat(60));
        let mut pulses: VecDeque<_> = initial_pulse.clone().into();

        while let Some(pulse_to_send) = pulses.pop_front() {
            let (source, destination, pulse) = pulse_to_send;

            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }
            // println!("{:?} -{:?}-> {:?}", source, pulse, destination);

            if let Some(module) = modules.get(&destination) {
                if let Some((new_destination_modules, new_pulse)) = module.handle(source, pulse) {
                    for new_destination in new_destination_modules {
                        pulses.push_back((destination.clone(), new_destination.to_string(), new_pulse.clone()));
                    }
                }
            }
        }
        // println!();
    }


    println!("High Pulses: {:?} - Low Pulses: {:?}", high_pulses, low_pulses);
    println!("{:?}", high_pulses * low_pulses);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pulse {
    Low,
    High
}

#[derive(Debug, Copy, Clone)]
enum ModuleStatus {
    On,
    Off,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(ModuleStatus),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
    Button,
    Untyped,
}



trait PulseHandler {
    fn handle(&self, source: String, pulse: Pulse) -> Option<(Vec<String>, Pulse)>;
}

#[derive(Debug)]
struct Module {
    name: String,
    destination_modules: Vec<String>,
    module_type: RefCell<ModuleType>
}

impl PulseHandler for Module {
    fn handle(&self, source: String, pulse: Pulse) -> Option<(Vec<String>, Pulse)>{
        match *self.module_type.borrow_mut() {
            ModuleType::Button => {
                Some((vec!["broadcaster".to_string()], Pulse::Low))
            },
            ModuleType::Broadcast => {
                Some((self.destination_modules.clone(), pulse))
            },
            ModuleType::FlipFlop(ref mut status) => {
                match pulse {
                    Pulse::High => None,
                    Pulse::Low => {
                        match status {
                            ModuleStatus::Off => {
                                *status = ModuleStatus::On;
                                Some((self.destination_modules.clone(), Pulse::High))
                            },
                            ModuleStatus::On => {
                                *status = ModuleStatus::Off;
                                Some((self.destination_modules.clone(), Pulse::Low))
                            },
                        }
                    }
                }
            },
            ModuleType::Conjunction(ref mut map) => {
                map.entry(source).and_modify(|e| *e = pulse);
                if map.values().all(|p| *p == Pulse::High) {
                    Some((self.destination_modules.clone(), Pulse::Low))
                } else {
                    Some((self.destination_modules.clone(), Pulse::High))
                }
            },
            ModuleType::Untyped => None,
        }
    }
}

impl From<&str> for Module {
    fn from(s: &str) -> Self {
        let splitted: Vec<&str> = s.split(" -> ").collect();

        let re: Regex = Regex::new(r"(?<type>broadcaster|%|&)(?<name>\w*)").unwrap();
        let captures = re.captures(splitted[0]).unwrap();

        let module_type: &str = &captures["type"];
        let module_name: &str = &captures["name"];

        let destination_modules: Vec<&str> = splitted[1]
            .split(',')
            .map(|s| s.trim())
            .collect();

        match module_type {
            "broadcaster" => Self {
                name: "broadcaster".to_string(),
                destination_modules: destination_modules.iter().map(|s| s.to_string()).collect(),
                module_type: RefCell::new(ModuleType::Broadcast)
            },
            "%" => Self {
                name: module_name.to_string(),
                destination_modules: destination_modules.iter().map(|s| s.to_string()).collect(),
                module_type: RefCell::new(ModuleType::FlipFlop(ModuleStatus::Off))
            },
            "&" => {
                let destination_modules: Vec<String> = destination_modules
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                let conjunction_map: HashMap<String, Pulse> = HashMap::new();

                Self {
                    name: module_name.to_string(),
                    destination_modules,
                    module_type: RefCell::new(ModuleType::Conjunction(conjunction_map))
                }
            },
            _ => Self {
                name: "broadcaster".to_string(),
                destination_modules: destination_modules.iter().map(|s| s.to_string()).collect(),
                module_type: RefCell::new(ModuleType::Untyped)
            },
        }
    }
}
