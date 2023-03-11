use rand::{prelude::SliceRandom, thread_rng};
use std::{
    fs::File,
    io::{BufRead, BufReader}, path::Path,
};

use crate::pipeline::*;

/// Given files, randomly load segments seperated by a delimeter
pub struct RandomLoader {
    files: Vec<String>,            // The files to load from
    delimeter: String,             // The delimiter to split examples by
    load_order: Vec<usize>,        // A full vector of indexes for every example, shuffled on reset
    currently_loaded_index: usize, // The last example we loaded as an index of the load_order vector (starts at 0)
    max_index: usize,              // The max index to load
    min_index: usize,              // The min index to load
}

impl RandomLoader {
    pub fn new<T: ToString>(files: &[T]) -> Self {
        RandomLoader {
            files: files.iter().map(|s| s.to_string()).collect(),
            delimeter: "\n".to_string(),
            load_order: vec![],
            currently_loaded_index: 0,
            min_index: 0,
            max_index: usize::MAX,
        }
    }

    /// Create a new RandomLoader with all files in a directory
    pub fn from_directory<T: AsRef<Path>>(path: T) -> Self {
        let files = std::fs::read_dir(path)
            .unwrap()
            .map(|r| r.unwrap().path().to_str().unwrap().to_string())
            .collect();
        RandomLoader {
            files,
            delimeter: "\n".to_string(),
            load_order: vec![],
            currently_loaded_index: 0,
            min_index: 0,
            max_index: usize::MAX,
        }
    }

    pub fn with_delimeter(self, delimeter: String) -> Self {
        RandomLoader { delimeter, ..self }
    }

    pub fn max_index(self, max_index: usize) -> Self {
        RandomLoader { max_index, ..self }
    }

    pub fn min_index(self, min_index: usize) -> Self {
        RandomLoader { min_index, ..self }
    }
}

impl Node for RandomLoader {
    type Input = Vec<()>;
    type Output = Vec<String>;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        // Load next input.len() examples in order, then shuffle them
        let mut examples_to_load = self.load_order[self.currently_loaded_index
            ..self
                .load_order
                .len()
                .min(self.currently_loaded_index + input.len())]
            .to_vec();
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
                        if current_example == examples_to_load.len() {
                            break;
                        }
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
                            for s in split[1..split.len() - 1].iter() {
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
                        if current_index >= examples_to_load.len() {
                            break;
                        }
                    } else {
                        // No delimeter, just append to intermediate
                        intermediate.push_str(&line);
                    }
                }
            }
            if current_example == examples_to_load.len() {
                break;
            }
        }

        self.currently_loaded_index += loaded.len();

        loaded.shuffle(&mut thread_rng());
        loaded
    }

    fn reset(&mut self) {
        // Count the total number of examples
        let mut total_examples = 0;
        for file in &self.files {
            let file = File::open(file).unwrap();
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
            total_examples += delimeter_count;
            if total_examples >= self.max_index {
                break;
            }
        }
        // Setup load_order (randomize on two levels: blocks of 100,000, and inside blocks)
        let mut rng = thread_rng();
        // Get starting block indexes
        let mut block_indexes: Vec<usize> = (usize::max(0, self.min_index)
            ..usize::min(total_examples, self.max_index))
            .step_by(100_000)
            .collect();
        block_indexes.shuffle(&mut rng);
        // Fill in blocks
        self.load_order = block_indexes
            .iter()
            .map(|i| {
                let mut indexes: Vec<usize> =
                    (*i..usize::min(i + 100_000, self.max_index)).collect();
                indexes.shuffle(&mut rng);
                indexes
            })
            .fold(
                Vec::with_capacity(
                    usize::min(total_examples, self.max_index) - usize::max(0, self.min_index),
                ),
                |mut acc, i| {
                    acc.extend(i.into_iter());
                    acc
                },
            );
        self.currently_loaded_index = 0;
    }

    fn data_remaining(&self, _before: usize) -> usize {
        self.load_order.len() - self.currently_loaded_index
    }
}