use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::pipeline::*;

/// A loader with a key generating function
pub struct KeyedLoader {
    files: Vec<String>,
    file_sizes: Vec<usize>,
    delimeter: String,
}

impl KeyedLoader {
    pub fn new(files: &[&str], delimeter: &str) -> Self {
        // Get file sizes
        let file_sizes: Vec<usize> = files
            .iter()
            .map(|f| {
                let file = File::open(f).unwrap();
                let reader = BufReader::new(file);
                let mut delimeter_count = 0;
                if delimeter == "\n" {
                    delimeter_count = reader.lines().count();
                } else {
                    for line in reader.lines().flatten() {
                        delimeter_count += line.matches(delimeter).count();
                    }
                    delimeter_count += 1; // Since delimeters divide the examples, there should be 1 more example than delimeter
                }
                delimeter_count
            })
            .collect();

        KeyedLoader {
            files: files.iter().map(|s| s.to_string()).collect(),
            file_sizes,
            delimeter: delimeter.to_string(),
        }
    }
}

impl Node for KeyedLoader {
    type Input = Vec<usize>;
    type Output = Vec<String>;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        // Get bounds to load from
        let (min, max) = input.iter().minmax().into_option().unwrap().to_owned();
        let (mut min, mut max) = (*min, *max);
        let (mut min_file, mut max_file) = (0, 0);
        let mut counter = 0;
        for (index, file_size) in self.file_sizes.iter().enumerate() {
            counter += file_size;
            if counter > min {
                min_file = index;
                min -= counter + file_size;
            }
            if counter + file_size > max {
                max_file = index;
                max -= counter + file_size;
            }
        }
        // Sort inputs and keep track of order (orig order, sorted indexes)
        let mut sorted_inputs: Vec<(usize, usize)> = input.into_iter().enumerate().collect();
        sorted_inputs.sort_by(|a, b| a.1.cmp(&b.1));

        // Load all segments from min to max
        let mut buffer = Vec::with_capacity(sorted_inputs.len());
        for file_index in min_file..max_file + 1 {
            let file = File::open(&self.files[file_index]).unwrap();
            let reader = BufReader::new(file);

            let mut index_counter = 0;
            let mut segment_counter = if file_index == min_file { min } else { 0 };
            let segments_to_take = if file_index == max_file {
                max
            } else {
                self.file_sizes[file_index]
            };
            if self.delimeter == "\n" {
                for line in reader.lines().flatten() {
                    if segment_counter == sorted_inputs[index_counter].1 {
                        buffer.push(line);
                        index_counter += 1;
                        if index_counter == sorted_inputs.len() {
                            return buffer;
                        }
                    }
                    segment_counter += 1;
                }
            } else {
                let mut intermediate_segment = "".to_string();
                for line in reader.lines().flatten() {
                    let line_segments: Vec<&str> = line.split(&self.delimeter).collect();

                    if segment_counter == sorted_inputs[index_counter].1 {
                        buffer.push(format!("{}{}", intermediate_segment, line_segments[0]));
                        index_counter += 1;
                        if index_counter == sorted_inputs.len() {
                            return buffer;
                        }
                    }
                    for line_segment in line_segments
                        .iter()
                        .take((segments_to_take - counter).min(line_segments.len() - 1))
                    {
                        if segment_counter == sorted_inputs[index_counter].1 {
                            buffer.push(line_segment.to_string());
                            index_counter += 1;
                            if index_counter == sorted_inputs.len() {
                                return buffer;
                            }
                        }
                    }
                    intermediate_segment = line_segments.last().unwrap().to_string();

                    segment_counter += line_segments.len() - 1;
                    if segment_counter >= segments_to_take {
                        break;
                    }
                }
            }
        }

        buffer
    }

    fn reset(&mut self) {
        // Recalculate file sizes
        let file_sizes = self
            .files
            .iter()
            .map(|f| {
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
            })
            .collect();
        self.file_sizes = file_sizes;
    }

    fn data_remaining(&self, before: usize) -> usize {
        before
    }
}

impl ExplicitNode<Vec<usize>, Vec<String>> for KeyedLoader {
    fn process(&mut self, input: Vec<usize>) -> Vec<String> {
        <Self as Node>::process(self, input)
    }

    fn data_remaining(&self, before: usize) -> usize {
        <Self as Node>::data_remaining(self, before)
    }

    fn reset(&mut self) {
        <Self as Node>::reset(self);
    }
}