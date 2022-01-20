use std::{fs::File, io::{BufReader, BufRead}};
use rand::{prelude::SliceRandom, thread_rng};

use crate::pipeline::Node;

/// Given files, randomly load segments seperated by a delimeter
pub struct RandomLoader {
    files: Vec<String>, // The files to load from
    delimeter: String, // The delimiter to split examples by
    load_order: Vec<usize>, // A full vector of indexes for every example, shuffled on reset
    currently_loaded_index: usize // The last example we loaded as an index of the load_order vector (starts at 0)
}

impl RandomLoader {
    pub fn new(files: Vec<String>) -> Self {
        RandomLoader {
            files,
            delimeter: "\n".to_string(),
            load_order: vec![],
            currently_loaded_index: 0
        }
    }

    pub fn with_delimeter(self, delimeter: String) -> Self {
        RandomLoader {delimeter, ..self}
    }
}

impl Node for RandomLoader {
    type Input = ();
    type Output = String;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        // Load next input.len() examples in order, then shuffle them
        let mut examples_to_load = self.load_order[self.currently_loaded_index..self.currently_loaded_index + input.len()].to_vec();
        examples_to_load.sort_unstable();
        
        // Run through each example in each file
        let mut current_index = 0;
        let mut current_example = 0;
        let mut loaded = vec![];
        for file in &self.files {
            let file = File::open(file).unwrap();
            let reader = BufReader::new(file);
            if self.delimeter == "\n" {
                for line in reader.lines().flatten() {
                    if current_index == examples_to_load[current_example] {
                        loaded.push(line);
                        current_example += 1;
                        if current_example == examples_to_load.len() {break;}
                    }
                    current_index += 1;
                }
            } else {
                let mut intermediate = String::new();
                for line in reader.lines().flatten() {
                    if line.contains(&self.delimeter) {
                        let split: Vec<&str> = line.split(&self.delimeter).collect();
                        // Add first
                        if line.starts_with(&self.delimeter) {
                            if current_index == examples_to_load[current_example] {
                                loaded.push(intermediate.clone());
                                current_example += 1;
                            }
                            current_index += 1;
                            intermediate = String::new();
                        }
                        if intermediate.is_empty() {
                            if current_index == examples_to_load[current_example] {
                                loaded.push(split[0].to_string());
                                current_example += 1;
                            }
                        } else if current_index == examples_to_load[current_example] {
                            intermediate.push_str(split[0]);
                            loaded.push(intermediate.clone());
                            current_example += 1;
                        }
                        current_index += 1;
                        // Add middle
                        if split.len() > 1 {
                            for s in split[1..split.len()-1].iter() {
                                if current_index == examples_to_load[current_example] {
                                    loaded.push(s.to_string());
                                    current_example += 1;
                                }
                                current_index += 1;
                            }
                        }
                        // Add end
                        if line.ends_with(&self.delimeter) {
                            if current_index == examples_to_load[current_example] {
                                loaded.push(split.last().unwrap().to_string());
                                current_example += 1;
                            }
                            current_index += 1;
                        } else {
                            intermediate = split.last().unwrap().to_string();
                        }
                    } else {
                        // No delimeter, just append to intermediate
                        intermediate.push_str(&line);
                    }
                }
            }
        }

        loaded.shuffle(&mut thread_rng());
        loaded
    }

    fn reset(&mut self) {
        // Count the total number of examples
        let total_examples = self.files.iter().map(|f| {
            let file = File::open(f).unwrap();
            let reader = BufReader::new(file);
            let mut delimeter_count = 0;
            if self.delimeter == "\n" {
                delimeter_count = reader.lines().count();
            } else {
                for line in reader.lines().flatten() {
                    delimeter_count += line.matches(&self.delimeter).count();
                }
                delimeter_count += 1; // Since delimeters divide the examples, there should be 1 more example than delimeter
            }
            delimeter_count
        }).sum();
        // Setup load_order
        self.load_order = (0..total_examples).collect();
        self.load_order.shuffle(&mut thread_rng());
        self.currently_loaded_index = 0;
    }

    fn data_remaining(&self) -> usize {
        self.load_order.len() - self.currently_loaded_index
    }
}