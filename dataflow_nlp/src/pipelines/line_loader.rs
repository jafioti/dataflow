use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use dataflow::pipeline::*;

/// Given files, randomly load segments seperated by a delimeter
pub struct RandomLoader {
    files: Vec<String>, // The files to load from
    delimeter: String,  // The delimiter to split examples by
    total_examples: usize,
    currently_loaded_index: usize, // The last example we loaded as an index of the load_order vector (starts at 0)
    max_index: usize,              // The max index to load
    min_index: usize,              // The min index to load
}

impl RandomLoader {
    pub fn new<T: ToString>(files: &[T]) -> Self {
        RandomLoader {
            files: files.iter().map(|s| s.to_string()).collect(),
            delimeter: "\n".to_string(),
            total_examples: 0,
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
            total_examples: 0,
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

/// Load segments of text seperated by a delimeter, from start to end segments
fn load_text_segments(
    path: &str,
    indexes: &[usize],
    current_segment_index: &mut usize,
    delimiter: &str,
) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut segments = Vec::new();
    let mut current_segment = String::new();

    for line in reader.lines().flatten() {
        if line.contains(delimiter) {
            let mut line_segments = line.split(delimiter);

            // Handle beginning segment
            if let Some(segment) = line_segments.next() {
                if *current_segment_index == indexes[segments.len()] {
                    segments.push(format!("{current_segment}{segment}"));
                    current_segment.clear();
                }
                *current_segment_index += 1;
            }
            // Handle middle segments
            for segment in line_segments {
                let Some(&ind) = indexes.get(segments.len()) else {
                    return Ok(segments);
                };
                if *current_segment_index == ind {
                    segments.push(format!("{current_segment}{segment}"));
                }
                *current_segment_index += 1;
            }
            // We aren't supposed to finalize the last segment
            if let Some(last) = segments.pop() {
                current_segment = last;
                current_segment.push('\n');
                *current_segment_index -= 1;
            }
        } else if *current_segment_index == indexes[segments.len()] {
            current_segment.push_str(&line);
            current_segment.push('\n');
        }

        if segments.len() >= indexes.len() {
            break;
        }
    }

    Ok(segments)
}

impl Node<Vec<()>> for RandomLoader {
    type Output = Vec<String>;

    fn process(&mut self, input: Vec<()>) -> Self::Output {
        // Run through each example in each file
        let mut current_index = 0;
        let mut loaded = vec![];
        for file in &self.files {
            loaded.append(
                &mut load_text_segments(
                    file,
                    &(self.currently_loaded_index
                        ..(self.currently_loaded_index + input.len()).min(self.max_index))
                        .collect::<Vec<_>>(),
                    &mut current_index,
                    &self.delimeter,
                )
                .unwrap(),
            );
            if loaded.len() >= input.len() {
                break;
            }
        }

        loaded.truncate(input.len());
        self.currently_loaded_index += loaded.len();
        loaded
    }

    fn reset(&mut self) {
        // Count the total number of examples
        self.total_examples = 0;
        for file in &self.files {
            let reader = BufReader::new(File::open(file).unwrap());
            let mut delimeter_count = 0;
            if self.delimeter == "\n" {
                delimeter_count = reader.lines().count();
            } else {
                delimeter_count += reader
                    .lines()
                    .flatten()
                    .map(|line| line.matches(&self.delimeter).count())
                    .sum::<usize>();
                delimeter_count += 1; // Since delimeters divide the examples, there should be 1 more example than delimeter
            }
            self.total_examples += delimeter_count;
            if self.total_examples >= self.max_index {
                break;
            }
        }
        self.total_examples = self.total_examples.min(self.max_index - self.min_index);
        self.currently_loaded_index = self.min_index;
    }

    fn data_remaining(&self, _before: usize) -> usize {
        self.total_examples - (self.currently_loaded_index - self.min_index)
    }
}
