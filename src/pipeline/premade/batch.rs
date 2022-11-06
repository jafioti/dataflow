use std::marker::PhantomData;

use itertools::Itertools;

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

    fn data_remaining(&self, before: usize) -> usize {
        before / self.batch_size
    }
}

/// Create batches from examples
pub struct ArrayBatch<const B: usize, T> {
    _phantom: PhantomData<T>,
}

impl <const B: usize, T>ArrayBatch<B, T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default()
        }
    }
}

impl <const B: usize, T>Node for ArrayBatch<B, T> {
    type Input = T;
    type Output = [T; B];
    fn process(&mut self, mut input: Vec<Self::Input>) -> Vec<Self::Output> {
        let mut batches = Vec::with_capacity(input.len() / B);
        let chunks = input.into_iter().chunks(B);
        let mut chunks_iter = chunks.into_iter();
        while let Some(Ok(b)) = chunks_iter.next().map(|i| i.collect::<Vec<T>>().try_into()) {
            batches.push(b);
        }
        batches
    }

    fn data_remaining(&self, before: usize) -> usize {
        before / B
    }
}
