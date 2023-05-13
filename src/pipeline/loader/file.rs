use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};

use crate::pipeline::*;

pub struct FileLoader {
    files: Vec<PathBuf>,
    currently_loaded_index: usize, // The last example we loaded as an index of the load_order vector (starts at 0)
}

impl FileLoader {
    pub fn new(mut files: Vec<PathBuf>) -> Self {
        FileLoader {
            files: {
                files.shuffle(&mut thread_rng());
                files
            },
            currently_loaded_index: 0,
        }
    }

    pub fn from_directory<P: AsRef<Path>>(path: P) -> Self {
        FileLoader {
            files: std::fs::read_dir(path)
                .unwrap()
                .flatten()
                .map(|f| f.path())
                .collect_vec(),
            currently_loaded_index: 0,
        }
    }
}

impl Node<Vec<()>> for FileLoader {
    type Output = Vec<(PathBuf, Vec<u8>)>;

    fn process(&mut self, input: Vec<()>) -> Self::Output {
        let mut read_data = vec![];
        for file in self.files[self.currently_loaded_index
            ..(self.currently_loaded_index + input.len()).min(self.files.len() - 1)]
            .iter()
        {
            let mut data = Vec::new();
            let mut f = File::open(file).expect("FileLoader failed to load file!");
            f.read_to_end(&mut data).expect("Failed to read file!");
            read_data.push((file.clone(), data));
        }
        self.currently_loaded_index =
            (self.currently_loaded_index + input.len()).min(self.files.len());
        read_data
    }

    fn reset(&mut self) {
        self.files.shuffle(&mut thread_rng());
        self.currently_loaded_index = 0;
    }

    fn data_remaining(&self, _before: usize) -> usize {
        self.files.len() - self.currently_loaded_index
    }
}
