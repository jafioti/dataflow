use std::marker::PhantomData;

use crate::pipeline::Node;

/// Create batches from examples
pub struct Batch<T> {
    _phantom: PhantomData<T>,
    batch_size: usize,
}

impl<T> Batch<T> {
    pub fn new(batch_size: usize) -> Self {
        Batch {
            _phantom: PhantomData::default(),
            batch_size,
        }
    }
}

impl<T> Node for Batch<T> {
    type Input = T;
    type Output = Vec<T>;
    fn process(&mut self, mut input: Vec<Self::Input>) -> Vec<Self::Output> {
        let mut batches = Vec::with_capacity(input.len() / self.batch_size);
        while !input.is_empty() {
            batches.push(
                input
                    .drain(..usize::min(self.batch_size, input.len()))
                    .collect(),
            );
        }
        batches
    }
}
