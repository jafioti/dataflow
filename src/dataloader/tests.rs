use rand::{prelude::SliceRandom, thread_rng};
use super::Dataloader;
use crate::pipeline::{Node, Batch};


/// A "loader" to load a full range of numbers randomly
struct CreateRange {
    nums_to_make: Vec<usize>,
    current_progress: usize,
}

impl CreateRange {
    pub fn new(max: usize) -> Self {
        CreateRange {
            nums_to_make: (0..max).collect(),
            current_progress: 0
        }
    }
}

impl Node for CreateRange {
    type Input = ();
    type Output = usize;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        let data = self.nums_to_make[self.current_progress..self.current_progress + input.len()].to_vec();
        self.current_progress += input.len();
        data
    }

    fn reset(&mut self) {
        self.nums_to_make.shuffle(&mut thread_rng());
        self.current_progress = 0;
    }

    fn data_remaining(&self) -> usize {
        self.nums_to_make.len() - self.current_progress
    }
}

#[test]
fn test_dataloader() {
    // Write a dataloader test
    let pipeline = CreateRange::new(10_000)
        .add_fn(|i| i * 10)
        .add_node(Batch::new(10));
    let mut loader = Dataloader::new(pipeline);
    assert_eq!(loader.len(), 10_000);
    
    // Run for 5_000 steps and collect results
    let mut data = Vec::with_capacity(10_000);
    for example in &mut loader {
        data.extend(example.into_iter());
        if data.len() == 5_000 {break;}
    }

    // Check the examples, 5_000 should be retrieved
    assert_eq!(data.len(), 5_000);

    // Run for the rest of the data and store it
    for example in &mut loader {
        data.extend(example.into_iter());
    }
    assert_eq!(loader.len(), 10_000); // Make sure the loader reset

    // Sort read data
    data.sort_unstable();

    // Compare data
    assert_eq!(data, (0..10_000).into_iter().map(|i| i * 10).collect::<Vec<usize>>())
}