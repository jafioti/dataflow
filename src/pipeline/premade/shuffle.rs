use rand::{rngs::{StdRng}, seq::SliceRandom, SeedableRng};

use crate::pipeline::Node;
use std::marker::PhantomData;

pub struct Shuffle<T> {
    rng: StdRng,
    _phantom: PhantomData<T>
}

impl <T> Default for Shuffle<T> {
    fn default() -> Self {
        Self {
            rng: StdRng::from_entropy(),
            _phantom: Default::default()
        }
    }
}

impl<T> Node for Shuffle<T> {
    type Input = Vec<T>;
    type Output = Vec<T>;

    fn process(&mut self, mut input: Self::Input) -> Self::Output {
        input.shuffle(&mut self.rng);
        input
    }
}