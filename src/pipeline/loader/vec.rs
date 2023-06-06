use rand::{seq::SliceRandom, thread_rng};

use crate::prelude::Node;

pub struct VecLoader<T> {
    elements: Vec<T>,
    shuffle: bool,
    current_progress: usize,
}

impl<T> VecLoader<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            shuffle: false,
            current_progress: 0,
        }
    }

    pub fn shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }
}

impl<T: Clone> Node<Vec<()>> for VecLoader<T> {
    type Output = Vec<T>;

    fn reset(&mut self) {
        if self.shuffle {
            self.elements.shuffle(&mut thread_rng());
        }
    }

    fn process(&mut self, input: Vec<()>) -> Self::Output {
        if self.current_progress >= self.elements.len() {
            return vec![];
        }
        let elements = self.elements
            [self.current_progress..(self.current_progress + input.len()).min(self.elements.len())]
            .to_vec();
        self.current_progress += input.len();
        elements
    }

    fn data_remaining(&self, _: usize) -> usize {
        self.elements.len().saturating_sub(self.current_progress)
    }
}
